use crate::{
    cache::{Cache, CacheEntry, Excludes, HocParams},
    error::{Error, Result},
    http::AppState,
};

use std::{path::Path, process::Command, sync::atomic::Ordering};

use git2::{BranchType, ErrorCode, Repository, build::RepoBuilder};
use gix_glob::{Pattern, pattern::Case, wildmatch::Mode};
use tracing::{debug, info, instrument, trace, warn};

#[instrument("fetch", skip(path), fields(path = ?path.as_ref().display()))]
fn fetch(path: impl AsRef<Path>, branch: Option<&str>) -> Result<()> {
    info!("fetching");
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&[branch.unwrap_or("refs/heads/*:refs/heads/*")], None, None)?;
    Ok(())
}

#[instrument("clone", skip(path), fields(path = ?path.as_ref().display(), origin))]
fn clone(path: impl AsRef<Path>, origin: &str) -> Result<Option<Repository>> {
    info!("cloning");
    Ok(
        match RepoBuilder::new().bare(true).clone(origin, path.as_ref()) {
            Ok(repo) => Ok(Some(repo)),
            Err(e) if e.code() == ErrorCode::Auth => Ok(None),
            Err(e) => Err(e),
        }?,
    )
}

fn find_default_branch(repo: &Repository) -> Result<Option<String>> {
    Ok(repo
        .head()?
        .name()
        .map(|s| s.strip_prefix("refs/heads/").unwrap_or(s).to_string()))
}

#[instrument(skip(state))]
async fn open_repo(params: &HocParams, state: &AppState) -> Result<Option<Repository>> {
    let repo_path = params.repo(&state.settings);
    let repo = if repo_path.exists() {
        trace!("using existing repo");
        let repo = Repository::open_bare(&repo_path)?;
        {
            let repo_path = repo_path.clone();
            let branch = params.branch.clone();
            //
            // TODO: this will not abort nicely and must wait for the current fetch to complete
            tokio::task::spawn_blocking(move || fetch(&repo_path, branch.as_deref()))
        }
        .await??;
        Some(repo)
    } else {
        let url = params.url();
        info!("cloning for the first time");
        if let Some(repo) = {
            let repo_path = repo_path.clone();
            tokio::task::spawn_blocking(move || clone(&repo_path, &url))
        }
        .await??
        {
            state.repo_count.fetch_add(1, Ordering::Relaxed);
            Some(repo)
        } else {
            warn!("repository does not exist");
            state.cache.store(params.clone(), CacheEntry::NotFound)?;
            None
        }
    };
    Ok(repo)
}

#[instrument(skip(state))]
pub(crate) async fn hoc(params: &HocParams, state: &AppState) -> Result<()> {
    let Some(repo) = open_repo(params, state).await? else {
        return Ok(());
    };

    let branch = if let Some(ref branch) = params.branch {
        branch.clone()
    } else {
        find_default_branch(&repo)?.ok_or(Error::BranchNotFound)?
    };

    let head = repo
        .find_branch(&branch, BranchType::Local)
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

    let patterns = compile_patterns(&params.excludes);
    let cached = state.cache.load(params)?;
    if let Some(CacheEntry::Cached {
        head: cached_head, ..
    }) = cached.as_ref()
    {
        debug!("using cache");
        if cached_head == &head {
            trace!("cache up to date");
            return Ok(());
        }
        trace!("updating cache");
        arg.push(format!("{head}..{branch}"));
        arg_commit_count.push(format!("{head}..{branch}"));
    } else {
        debug!("Creating cache");
        arg.push(branch.clone());
        arg_commit_count.push(branch.clone());
    }

    arg.push("--".to_string());
    arg.push(".".to_string());

    let repo_path = params.repo(&state.settings);

    // TODO: this is also kinda blocking but should be fast enough
    let output = Command::new("git")
        .args(&arg)
        .current_dir(&repo_path)
        .output()?
        .stdout;
    let output = String::from_utf8_lossy(&output);

    // TODO: this is also kinda blocking but should be fast enough
    let output_commits = Command::new("git")
        .args(&arg_commit_count)
        .current_dir(&repo_path)
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
        || CacheEntry::Cached {
            head: head.clone(),
            count,
            commits,
        },
        |c| c.update(count, commits, &head),
    );
    state.cache.store(params.clone(), cached.clone())?;

    Ok(())
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
