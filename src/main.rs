#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)]

mod cache;
mod config;
mod server;
mod threadpool;
mod utils;

use std::rc::*;
use std::sync::*;
use std::time::*;

use cache::*;
use config::*;
use utils::*;

fn main() {
    const MAX_ENTRIES: usize = 10000;
    const MAX_CAPACITY: usize = 100;

    let cacher = Cache::new(CacheConfig {
        max_entries: MAX_CAPACITY,
        max_item_size: GIGABYTE,
    });

    let mut v = vec![];

    for i in 0..MAX_ENTRIES {
        let data = i.to_string();
        let key: CacheEntry = data.as_str().into();
        v.push((key.clone(), data));
    }

    let last = v.iter().last().unwrap().0.clone();
    let u = v.clone();

    let start = Instant::now();

    for (k, v) in v.into_iter() {
        cacher.insert(k, v);
    }

    let elapsed = start.elapsed();

    println!(
        "insert {} entries: {:?} ({:?} per iter)",
        MAX_ENTRIES,
        elapsed,
        elapsed / MAX_ENTRIES as u32
    );

    println!("starting ...");
    let start = Instant::now();

    for (k, _) in u.iter() {
        let cv = cacher.get(&last);
        if let Some(val) = cv {
            let _ = val.get().clone();
            drop(val);
        }
    }

    let elapsed = start.elapsed();

    println!(
        "cache hit {} entries: {:?} ({:?} per iter)",
        MAX_ENTRIES,
        elapsed,
        elapsed / MAX_ENTRIES as u32
    );
}
