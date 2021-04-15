use std::{
    borrow::Borrow,
    collections::hash_map::RandomState,
    hash::Hash,
    ops::Deref,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use dashmap::{
    mapref::{multiple::RefMulti, one::Ref},
    rayon::map::Iter,
    DashMap,
};
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use rayon::prelude::*;

use crate::config::CacheConfig;

pub const CHECK_TOP: usize = 10;

pub type CacheEntry = Rc<str>;
pub type CacheData = String;
pub type LastAccessLock<T> = RwLock<T>;

#[derive(Debug)]
pub enum RefreshMechanic {
    FileWatcher,
    None,
}

#[derive(Debug)]
pub struct CacheValue {
    data: CacheData,
    refresh_mechanic: RefreshMechanic,
    last_access: LastAccessLock<Instant>,
}

impl CacheValue {
    pub fn new(data: CacheData) -> Self {
        Self {
            data,
            refresh_mechanic: RefreshMechanic::None,
            last_access: LastAccessLock::new(Instant::now()),
        }
    }

    #[inline(always)]
    pub fn get<'a>(&'a self) -> &'a CacheData {
        let mut write_guard = self.last_access.write();
        *write_guard = Instant::now();
        drop(write_guard);
        &self.data
    }

    #[inline(always)]
    pub(crate) fn data(&self) -> &CacheData {
        &self.data
    }
}
pub struct CacheValueRef<'a> {
    data_ref: Ref<'a, CacheEntry, CacheValue, RandomState>,
}

impl<'a> CacheValueRef<'a> {
    #[inline(always)]
    pub fn new(data_ref: Ref<'a, CacheEntry, CacheValue, RandomState>) -> Self {
        Self { data_ref }
    }

    #[inline(always)]
    pub fn get(&self) -> &CacheData {
        self.data_ref.get()
    }

    #[inline(always)]
    pub fn get_key(&self) -> &CacheEntry {
        self.data_ref.key()
    }
}

impl<'a> Deref for CacheValueRef<'a> {
    type Target = CacheValue;
    fn deref(&self) -> &Self::Target {
        self.data_ref.value()
    }
}

impl Drop for CacheValue {
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct Cache {
    config: CacheConfig,
    storage: DashMap<CacheEntry, CacheValue>,
    len: AtomicUsize,
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            storage: DashMap::with_capacity(config.max_entries),
            config,
            len: AtomicUsize::new(0),
        }
    }

    pub fn get<K>(&self, key: &K) -> Option<CacheValueRef>
    where
        CacheEntry: Borrow<K>,
        K: Hash + Eq + ?Sized,
    {
        self.storage.get(key).map(CacheValueRef::new)
    }

    pub fn insert(&self, key: impl Into<CacheEntry>, data: impl Into<CacheData>) {
        let data = data.into();
        let key = key.into();
        if data.len() > self.config.max_item_size {
            return;
        } else if self.storage.contains_key(&key) {
        } else if self.len.load(Ordering::Acquire) >= self.config.max_entries {
            // Get top longest living cache data then get the first least using cache data.
            // Remove it.
            let old_info = self
                .storage
                .iter()
                .min_by_key(|lru_data| *lru_data.last_access.read())
                .map(|lru_data| lru_data.key().clone());

            if let Some(old_info) = old_info {
                self.storage.remove(&old_info);
            }
        } else {
            self.len.fetch_add(1, Ordering::Release);
        }

        self.storage.insert(key, CacheValue::new(data));
    }

    pub fn clear(&self) {
        self.storage.clear();
        self.len.store(0, Ordering::Release);
    }

    pub fn remove<K>(&self, key: &K)
    where
        CacheEntry: Borrow<K>,
        K: Hash + Eq + ?Sized,
    {
        self.storage.remove(key);
        self.len.fetch_sub(1, Ordering::AcqRel);
    }

    fn storage(&self) -> &DashMap<CacheEntry, CacheValue> {
        &self.storage
    }

    pub fn format(&self) -> String {
        let mut string_builder = String::new();
        for data in self.storage.iter() {
            let fmt = format!("{}: {}\n", data.key(), data.value().data());
            string_builder.push_str(&fmt);
        }

        string_builder
    }

    pub fn print(&self) {
        println!("{}", self.format());
    }
}
