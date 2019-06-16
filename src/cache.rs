use crate::error::{Error, Result};
use std::{
    borrow::Cow,
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
    path::Path,
};

/// Enum to indicate the state of the cache
pub(crate) enum CacheState<'a> {
    /// Current head and cached head are the same
    Current(u64),
    /// Cached head is older than current head
    Old(Cache<'a>),
    /// No cache was found
    No,
}

impl<'a> CacheState<'a> {
    pub(crate) fn read_from_file(path: impl AsRef<Path>, head: &str) -> Result<CacheState> {
        if path.as_ref().exists() {
            let cache: Cache = serde_json::from_reader(BufReader::new(File::open(path)?))?;
            if cache.head == head {
                Ok(CacheState::Current(cache.count))
            } else {
                Ok(CacheState::Old(cache))
            }
        } else {
            Ok(CacheState::No)
        }
    }

    pub(crate) fn calculate_new_cache(self, count: u64, head: Cow<'a, str>) -> Cache {
        match self {
            CacheState::Old(mut cache) => {
                cache.head = head;
                cache.count += count;
                cache
            }
            CacheState::No | CacheState::Current(_) => Cache { head, count },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Cache<'a> {
    pub head: Cow<'a, str>,
    pub count: u64,
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
