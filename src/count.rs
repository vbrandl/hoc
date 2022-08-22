use crate::error::Result;
use std::{
    fs::{read_dir, ReadDir},
    iter::once,
    path::Path,
    result::Result as StdResult,
};

/// The on disk layout for served repos is `<service>/<user>/<repo>`
/// so to get the amount of repos, we just have to count everything
/// in `*/*/*` to get the count.
#[instrument]
pub fn count_repositories<P>(repo_path: P) -> Result<usize>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    trace!("Counting repositories");
    std::fs::create_dir_all(&repo_path)?;
    Ok(once(read_dir(repo_path)?)
        .flat_map(sub_directories)
        .flat_map(sub_directories)
        .flat_map(sub_directories)
        .count())
}

fn sub_directories(dir: ReadDir) -> impl Iterator<Item = ReadDir> {
    dir.filter_map(StdResult::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|entry| read_dir(entry.path()).ok())
}
