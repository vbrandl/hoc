use crate::{
    config::Settings,
    error::{Error, Result},
    platform::Platform,
};

use std::{
    collections::BTreeSet,
    fs::{OpenOptions, create_dir_all, remove_dir_all},
    io::{self, BufReader},
    path::PathBuf,
};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace};

pub(crate) trait Cache<K, V> {
    fn load(&self, key: &K) -> Result<Option<V>>;
    fn store(&self, key: K, value: V) -> Result<()>;

    fn clear(&self, platform: Platform, owner: &str, repo: &str) -> Result<()>;
}

pub(crate) trait ToQuery {
    fn to_query(&self) -> String;
}

pub(crate) type Excludes = BTreeSet<String>;

impl ToQuery for Excludes {
    fn to_query(&self) -> String {
        let excludes: Vec<_> = self.iter().map(AsRef::as_ref).collect();
        let excludes = excludes.join(",");
        let excludes = urlencoding::encode(&excludes);
        excludes.to_string()
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct CacheKey {
    platform: Platform,
    owner: String,
    repo: String,
    branch: String,
    excludes: Excludes,
}

impl CacheKey {
    pub(crate) fn new(
        platform: Platform,
        owner: String,
        repo: String,
        branch: String,
        excludes: Excludes,
    ) -> Self {
        Self {
            platform,
            owner,
            repo,
            branch,
            excludes,
        }
    }

    fn cache_file(&self, settings: &Settings) -> PathBuf {
        let excludes = self.excludes.to_query();

        settings
            .cachedir
            .join(self.platform.domain())
            .join(self.owner.as_str())
            .join(self.repo.as_str())
            .join(self.branch.as_str())
            .join(excludes.as_str())
            .join("cache")
            .with_extension("json")
    }
}

pub(crate) struct Persist {
    in_memory: InMemoryCache,
    disk: DiskCache,
}

impl Persist {
    pub(crate) fn new(settings: Settings) -> Self {
        Self {
            in_memory: InMemoryCache::new(),
            disk: DiskCache { settings },
        }
    }
}

impl Drop for Persist {
    fn drop(&mut self) {
        info!("persisting cache");
        for r in &self.in_memory.cache {
            let platform = *r.key();
            for r in r.value() {
                let owner = r.key();
                for r in r.value() {
                    let repo = r.key();
                    for r in r.value() {
                        let branch = r.key();
                        for r in r.value() {
                            let excludes = r.key().clone();
                            let key = CacheKey::new(
                                platform,
                                owner.clone(),
                                repo.clone(),
                                branch.clone(),
                                excludes,
                            );
                            if let Err(err) = self.disk.store(key, r.value().clone()) {
                                error!(%err, key = ?r.key(), "cannot write cache to disk");
                            } else {
                                trace!(key = ?r.key(), "persisted");
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Cache<CacheKey, CacheEntry> for Persist {
    fn load(&self, key: &CacheKey) -> Result<Option<CacheEntry>> {
        if let Some(val) = self.in_memory.load(key)? {
            Ok(Some(val))
        } else if let Some(val) = self.disk.load(key)? {
            self.in_memory.store(key.clone(), val.clone())?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    fn store(&self, key: CacheKey, value: CacheEntry) -> Result<()> {
        self.in_memory.store(key, value)
    }

    fn clear(&self, platform: Platform, owner: &str, repo: &str) -> Result<()> {
        let im_res = self.in_memory.clear(platform, owner, repo);
        let disk_res = self.disk.clear(platform, owner, repo);
        if let Err(e) = im_res {
            Err(e)?
        } else if let Err(e) = disk_res {
            Err(e)?
        } else {
            Ok(())
        }
    }
}

struct InMemoryCache {
    #[allow(clippy::type_complexity)]
    cache: DashMap<
        Platform,
        DashMap<String, DashMap<String, DashMap<String, DashMap<Excludes, CacheEntry>>>>,
    >,
}

impl InMemoryCache {
    fn new() -> Self {
        Self {
            cache: DashMap::new(),
        }
    }
}

impl Cache<CacheKey, CacheEntry> for InMemoryCache {
    fn store(&self, key: CacheKey, value: CacheEntry) -> Result<()> {
        self.cache
            .entry(key.platform)
            .or_default()
            .entry(key.owner)
            .or_default()
            .entry(key.repo)
            .or_default()
            .entry(key.branch)
            .or_default()
            .insert(key.excludes, value);
        Ok(())
    }

    fn load(&self, key: &CacheKey) -> Result<Option<CacheEntry>> {
        Ok(self.cache.get(&key.platform).and_then(|c| {
            c.get(&key.owner).and_then(|c| {
                c.get(&key.repo).and_then(|c| {
                    c.get(&key.branch)
                        .and_then(|c| c.get(&key.excludes).map(|r| r.value().clone()))
                })
            })
        }))
    }

    fn clear(&self, platform: Platform, owner: &str, repo: &str) -> Result<()> {
        if let Some(c) = self.cache.get(&platform)
            && let Some(c) = c.value().get(owner)
        {
            c.value().remove(repo);
        }
        Ok(())
    }
}

struct DiskCache {
    settings: Settings,
}

impl Cache<CacheKey, CacheEntry> for DiskCache {
    fn load(&self, key: &CacheKey) -> Result<Option<CacheEntry>> {
        let cache_file = key.cache_file(&self.settings);
        match OpenOptions::new().read(true).open(&cache_file) {
            Ok(f) => Ok(serde_json::from_reader(BufReader::new(f))?),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(e)?
                }
            }
        }
    }

    fn store(&self, key: CacheKey, value: CacheEntry) -> Result<()> {
        trace!("writing cache");

        let cache_file = key.cache_file(&self.settings);

        let parent = cache_file.parent().ok_or(Error::Internal)?;
        create_dir_all(parent)?;

        serde_json::to_writer(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(cache_file)?,
            &value,
        )?;
        Ok(())
    }

    fn clear(&self, platform: Platform, owner: &str, repo: &str) -> Result<()> {
        let cache_dir = self
            .settings
            .cachedir
            .join(platform.url_path())
            .join(owner)
            .join(repo);
        remove_dir_all(cache_dir).or_else(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CacheEntry {
    /// HEAD commit ref
    pub head: String,
    /// `HoC` value
    pub count: u64,
    /// Number of commits
    pub commits: u64,
}

impl CacheEntry {
    pub(crate) fn update(self, count: u64, commits: u64) -> Self {
        Self {
            count: self.count + count,
            commits: self.commits + commits,
            ..self
        }
    }
}
