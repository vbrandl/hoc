use crate::error::{Error, Result};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
};

/// Enum to indicate the state of the cache
pub(crate) enum CacheState<'a> {
    /// Current head and cached head are the same
    Current {
        count: u64,
        commits: u64,
        cache: Cache<'a>,
    },
    /// Cached head is older than current head
    Old {
        head: String,
        cache: Cache<'a>,
    },
    NoneForBranch(Cache<'a>),
    /// No cache was found
    No,
}

impl<'a> CacheState<'a> {
    pub(crate) fn read_from_file(
        path: impl AsRef<Path>,
        branch: &str,
        head: &str,
    ) -> Result<CacheState<'a>> {
        if path.as_ref().exists() {
            let cache: Cache = serde_json::from_reader(BufReader::new(File::open(path)?))?;
            Ok(cache
                .entries
                .get(branch)
                .map(|c| {
                    if c.head == head {
                        CacheState::Current {
                            count: c.count,
                            commits: c.commits,
                            // TODO: get rid of clone
                            cache: cache.clone(),
                        }
                    } else {
                        CacheState::Old {
                            head: c.head.to_string(),
                            // TODO: get rid of clone
                            cache: cache.clone(),
                        }
                    }
                })
                // TODO: get rid of clone
                .unwrap_or_else(|| CacheState::NoneForBranch(cache.clone())))
        } else {
            Ok(CacheState::No)
        }
    }

    pub(crate) fn calculate_new_cache(
        self,
        count: u64,
        commits: u64,
        head: Cow<'a, str>,
        branch: &'a str,
    ) -> Cache<'a> {
        match self {
            CacheState::Old { mut cache, .. } => {
                if let Some(mut cache) = cache.entries.get_mut(branch) {
                    cache.head = head;
                    cache.count += count;
                    cache.commits += commits;
                }
                cache
            }
            CacheState::Current { cache, .. } => cache,
            CacheState::NoneForBranch(mut cache) => {
                cache.entries.insert(
                    branch.into(),
                    CacheEntry {
                        head,
                        count,
                        commits,
                    },
                );
                cache
            }
            CacheState::No => {
                let mut entries = HashMap::with_capacity(1);
                entries.insert(
                    branch.into(),
                    CacheEntry {
                        commits,
                        head,
                        count,
                    },
                );
                Cache { entries }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Cache<'a> {
    pub entries: HashMap<Cow<'a, str>, CacheEntry<'a>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CacheEntry<'a> {
    /// HEAD commit ref
    pub head: Cow<'a, str>,
    /// HoC value
    pub count: u64,
    /// Number of commits
    pub commits: u64,
}

impl<'a> Cache<'a> {
    pub(crate) fn write_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        create_dir_all(path.as_ref().parent().ok_or(Error::Internal)?)?;
        serde_json::to_writer(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?,
            self,
        )?;
        Ok(())
    }
}
