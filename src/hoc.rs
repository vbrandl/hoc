use crate::{
    cache::{Cache, CacheEntry, CacheKey, Excludes, Persist},
    error::{Error, Result},
    platform::Platform,
};

use std::{path::Path, process::Command};

use git2::{BranchType, Repository};
use gix_glob::{Pattern, pattern::Case, wildmatch::Mode};
use tracing::{debug, trace};

pub(crate) fn hoc(
    repo_dir: impl AsRef<Path>,
    platform: Platform,
    owner: &str,
    repo: &str,
    cache: &Persist,
    branch: &str,
    excludes: Excludes,
) -> Result<(u64, String, u64)> {
    let repo_dir = repo_dir
        .as_ref()
        .join(platform.domain())
        .join(owner)
        .join(repo);

    let repository = Repository::open_bare(&repo_dir)?;
    // TODO: do better...
    let head = repository
        .find_branch(branch, BranchType::Local)
        .map_err(|_| Error::BranchNotFound)?
        .into_reference();
    let head = format!("{}", head.target().ok_or(Error::BranchNotFound)?);
    let mut arg_commit_count = vec!["rev-list".to_string(), "--count".to_string()];
    let mut arg = vec![
        "log".to_string(),
        "--pretty=tformat:".to_string(),
        "--numstat".to_string(),
        "--ignore-space-change".to_string(),
        "--ignore-all-space".to_string(),
        "--ignore-submodules".to_string(),
        "--no-color".to_string(),
        "--find-copies-harder".to_string(),
        "-M".to_string(),
        "--diff-filter=ACDM".to_string(),
    ];

    let patterns = compile_patterns(&excludes);
    let key = CacheKey::new(platform, owner.into(), repo.into(), branch.into(), excludes);
    let cached = cache.load(&key)?;
    if let Some(cached) = cached.as_ref() {
        debug!("using cache");
        if cached.head == head {
            trace!("cache up to date");
            return Ok((cached.count, head, cached.commits));
        }
        trace!("updating cache");
        arg.push(format!("{head}..{branch}"));
        arg_commit_count.push(format!("{head}..{branch}"));
    } else {
        debug!("Creating cache");
        arg.push(branch.to_string());
        arg_commit_count.push(branch.to_string());
    }

    arg.push("--".to_string());
    arg.push(".".to_string());

    let output = Command::new("git")
        .args(&arg)
        .current_dir(&repo_dir)
        .output()?
        .stdout;
    let output = String::from_utf8_lossy(&output);

    let output_commits = Command::new("git")
        .args(&arg_commit_count)
        .current_dir(&repo_dir)
        .output()?
        .stdout;
    let output_commits = String::from_utf8_lossy(&output_commits);

    let commits: u64 = output_commits.trim().parse()?;
    let count: u64 = output.lines().fold(0, |sum, line| {
        let mut parts = line.split_whitespace();
        let additions = parts.next();
        let deletions = parts.next();
        let file_path = parts.next().unwrap_or_default();

        if matches(file_path, &patterns) {
            sum
        } else {
            let additions: u64 = additions.and_then(|s| s.parse().ok()).unwrap_or_default();
            let deletions: u64 = deletions.and_then(|s| s.parse().ok()).unwrap_or_default();

            sum + additions + deletions
        }
    });

    let cached = cached.map_or_else(
        || CacheEntry {
            head: head.clone(),
            count,
            commits,
        },
        |c| c.update(count, commits),
    );
    cache.store(key, cached.clone())?;

    Ok((cached.count, cached.head, cached.commits))
}

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

fn matches(file_path: &str, patterns: &[Pattern]) -> bool {
    if file_path.is_empty() {
        false
    } else {
        let file_path = file_path.into();
        patterns.iter().any(|pattern| {
            pattern.matches_repo_relative_path(
                file_path,
                None,
                None,
                Case::Sensitive,
                Mode::empty(),
            )
        })
    }
}
