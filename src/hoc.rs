use crate::{
    cache::{Cache, CacheEntry, CacheKey, Excludes, Persist},
    error::Error,
    platform::Platform,
};

use std::path::Path;

use anyhow::Result;
use gix::{
    ObjectId, Repository, Tree,
    glob::{Pattern, pattern::Case, wildmatch::Mode},
    object::tree::diff::Action,
    refs::PartialNameRef,
};

use itertools::Itertools;
use tracing::{debug, trace};

fn compile_patterns(excludes: &Excludes) -> Vec<Pattern> {
    excludes
        .iter()
        .filter_map(|pattern| {
            let pattern = pattern.trim();
            if pattern.ends_with('/') {
                let pattern = pattern.to_string() + "*";
                Pattern::from_bytes(pattern.as_bytes())
            } else {
                Pattern::from_bytes(pattern.as_bytes())
            }
        })
        .collect()
}

pub(crate) struct HocCount {
    pub(crate) hoc: u64,
    pub(crate) commits: u64,
    pub(crate) head: String,
}

pub(crate) fn hoc(
    repo_dir: impl AsRef<Path>,
    platform: Platform,
    owner: &str,
    repo: &str,
    cache: &Persist,
    branch: &str,
    excludes: Excludes,
) -> Result<HocCount> {
    let repo_dir = repo_dir
        .as_ref()
        .join(platform.domain())
        .join(owner)
        .join(repo);
    let patterns = compile_patterns(&excludes);

    let r = gix::open(repo_dir)?;
    let branch_ref: &PartialNameRef = branch.try_into()?;
    let branch_ref = r
        .try_find_reference(branch_ref)?
        .ok_or_else(|| Error::BranchNotFound)?;

    let head = branch_ref.target().id().to_string();

    let key = CacheKey::new(platform, owner.into(), repo.into(), branch.into(), excludes);
    let cached = cache.load(&key)?;

    let kind = if let Some(cached) = cached.as_ref() {
        debug!("using cache");
        if cached.head == head {
            trace!("cache up to date");
            return Ok(HocCount {
                hoc: cached.count,
                commits: cached.commits,
                head,
            });
        }
        trace!("updating cache");
        Kind::Since(&cached.head)
    } else {
        debug!("Creating cache");
        Kind::Full
    };
    let range = Range { branch, kind };

    let mut x = hoc_inner(&r, &range, &patterns)?;

    let cached = cached.map_or_else(
        || CacheEntry {
            head: head.clone(),
            count: x.hoc,
            commits: x.commits,
        },
        |c| c.update(x.hoc, x.commits),
    );

    x.hoc = cached.count;
    x.commits = cached.commits;

    cache.store(key, cached)?;

    Ok(x)
}

fn hoc_inner(repo: &Repository, range: &Range, exclude: &[Pattern]) -> Result<HocCount> {
    let branch_ref: &PartialNameRef = range.branch.try_into()?;
    let branch_ref = repo
        .try_find_reference(branch_ref)?
        .ok_or_else(|| Error::BranchNotFound)?;

    let head = branch_ref.target().id().to_string();
    let boundary = match range.kind {
        Kind::Full => None,
        Kind::Since(since) => Some(ObjectId::from_hex(since.as_bytes())?),
    };

    let mut stop = false;
    let walk = repo.rev_walk([branch_ref.id()]);
    let history: Vec<_> = walk
        .all()?
        .filter_map(Result::ok)
        .take_while(|info| {
            let res = stop;
            stop = Some(info.id) == boundary;
            !res
        })
        .collect();
    let commits = u64::try_from(history.len())?
        - if let Kind::Since(_) = range.kind {
            1
        } else {
            0
        };

    let insert_empty = if let Some(info) = &history.last()
        && let Kind::Since(since) = range.kind
        && info.id == ObjectId::from_hex(since.as_bytes())?
    {
        false
    } else {
        true
    };

    let history = std::iter::chain(
        history
            .into_iter()
            .map(|i| i.object())
            .filter_map(Result::ok)
            .map(|o| o.tree())
            .filter_map(Result::ok),
        if insert_empty {
            Some(empty_tree(repo))
        } else {
            None
        },
    );

    let hoc = history.tuple_windows().fold(0, |sum, (next, prev)| {
        (if let Ok(mut changes) = prev.changes()
            && let Ok(mut resource_cache) = prev.repo.diff_resource_cache_for_tree_diff()
        {
            let mut x = 0;
            changes
                .for_each_to_obtain_tree(&next, |change| {
                    if !exclude.iter().any(|p| {
                        p.matches_repo_relative_path(
                            change.location(),
                            None,
                            None,
                            Case::Sensitive,
                            Mode::empty(),
                        )
                    }) && let Some(counts) = change
                        .diff(&mut resource_cache)
                        .ok()
                        .and_then(|mut platform| platform.line_counts().ok())
                        .flatten()
                    {
                        x += u64::from(counts.insertions) + u64::from(counts.removals);
                    }

                    resource_cache.clear_resource_cache_keep_allocation();
                    Ok::<_, std::convert::Infallible>(Action::Continue)
                })
                // TODO:
                .unwrap();
            x
        } else {
            0
        }) + sum
    });
    Ok(HocCount { hoc, commits, head })
}

fn empty_tree(repo: &Repository) -> Tree<'_> {
    Tree {
        repo,
        id: ObjectId::Sha1([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        data: Vec::new(),
    }
}

struct Range<'a> {
    branch: &'a str,
    kind: Kind<'a>,
}

enum Kind<'a> {
    Full,
    Since(&'a str),
}
