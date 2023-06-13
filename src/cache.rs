use crate::error::{Error, Result};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
};

/// Enum to indicate the state of the cache
#[derive(Debug)]
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
    #[instrument]
    pub(crate) fn read_from_file(
        path: impl AsRef<Path> + std::fmt::Debug,
        branch: &str,
        head: &str,
    ) -> Result<CacheState<'a>> {
        trace!("Reading cache");
        if path.as_ref().exists() {
            let cache: Cache = serde_json::from_reader(BufReader::new(File::open(path)?))?;
            Ok(cache
                .entries
                .get(branch)
                .map_or_else(
                // TODO: get rid of clone
|| CacheState::NoneForBranch(cache.clone()),
                    |c| {
                    if c.head == head {
                        trace!("Cache is up to date");
                        CacheState::Current {
                            count: c.count,
                            commits: c.commits,
                            // TODO: get rid of clone
                            cache: cache.clone(),
                        }
                    } else {
                        trace!("Cache is out of date");
                        CacheState::Old {
                            head: c.head.to_string(),
                            // TODO: get rid of clone
                            cache: cache.clone(),
                        }
                    }
                })
                    )
        } else {
            Ok(CacheState::No)
        }
    }

    #[instrument]
    pub(crate) fn calculate_new_cache(
        self,
        count: u64,
        commits: u64,
        head: Cow<'a, str>,
        branch: &'a str,
    ) -> Cache<'a> {
        trace!("Calculating new cache");
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
                trace!("Creating new cache for branch");
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
                trace!("Creating new cache file");
                let mut entries = HashMap::with_capacity(1);
                entries.insert(
                    branch.into(),
                    CacheEntry {
                        head,
                        count,
                        commits,
                    },
                );
                Cache { entries }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Cache<'a> {
    pub entries: HashMap<Cow<'a, str>, CacheEntry<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CacheEntry<'a> {
    /// HEAD commit ref
    pub head: Cow<'a, str>,
    /// HoC value
    pub count: u64,
    /// Number of commits
    pub commits: u64,
}

impl<'a> Cache<'a> {
    #[instrument]
    pub(crate) fn write_to_file(&self, path: impl AsRef<Path> + std::fmt::Debug) -> Result<()> {
        trace!("Persisting cache to disk");
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
