use crate::{
    cache::{Cache, CacheEntry, Excludes, HocParams},
    error::{Error, Result},
    http::AppState,
    statics::CLIENT,
};

use std::{fs::create_dir_all, path::Path, process::Command, sync::atomic::Ordering};

use git2::{BranchType, Repository};
use gix_glob::{Pattern, pattern::Case, wildmatch::Mode};
use tracing::{debug, info, trace, warn};

async fn remote_exists(url: &str) -> Result<bool> {
    let resp = CLIENT.head(url).send().await?;
    Ok(resp.status() == reqwest::StatusCode::OK)
}

fn fetch(path: impl AsRef<Path>, branch: &str) -> Result<()> {
    let repo = Repository::open_bare(path)?;
    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&[branch], None, None)?;
    Ok(())
}

pub(crate) async fn hoc(params: &HocParams, state: &AppState) -> Result<()> {
    let repo_path = params.repo(&state.settings);
    let repo = if repo_path.exists() {
        Repository::open_bare(&repo_path)?
    } else {
        let url = params.url();
        let remote_exists = remote_exists(&url).await?;
        if !remote_exists {
            warn!("Repository does not exist");
            state.cache.store(params.clone(), CacheEntry::NotFound)?;
            return Ok(());
        }
        info!("Cloning for the first time");
        create_dir_all(&repo_path)?;
        let repo = Repository::init_bare(&repo_path)?;
        repo.remote_add_fetch("origin", "refs/heads/*:refs/heads/*")?;
        repo.remote_set_url("origin", &url)?;
        state.repo_count.fetch_add(1, Ordering::Relaxed);
        repo
    };

    fetch(&repo_path, &params.branch)?;

    let head = repo
        .find_branch(&params.branch, BranchType::Local)
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
        arg.push(format!("{head}..{}", params.branch));
        arg_commit_count.push(format!("{head}..{}", params.branch));
    } else {
        debug!("Creating cache");
        arg.push(params.branch.clone());
        arg_commit_count.push(params.branch.clone());
    }

    arg.push("--".to_string());
    arg.push(".".to_string());

    let output = Command::new("git")
        .args(&arg)
        .current_dir(&repo_path)
        .output()?
        .stdout;
    let output = String::from_utf8_lossy(&output);

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
        |c| c.update(count, commits),
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
