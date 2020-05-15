use crate::error::Result;
use std::{fs::read_dir, path::Path, result::Result as StdResult};

pub(crate) fn count_repositories<P>(repo_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(&repo_path)?;
    Ok(read_dir(repo_path)?
        .filter_map(StdResult::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .map(|entry| read_dir(entry.path()))
        .filter_map(StdResult::ok)
        .flat_map(|dir| {
            dir.filter_map(StdResult::ok)
                .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        })
        .map(|entry| read_dir(entry.path()))
        .filter_map(StdResult::ok)
        .flat_map(|dir| {
            dir.filter_map(StdResult::ok)
                .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        })
        .count())
}
