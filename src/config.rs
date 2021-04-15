pub const KILOBYTE: usize = 1024;
pub const MEGABYTE: usize = KILOBYTE * KILOBYTE;
pub const GIGABYTE: usize = MEGABYTE * MEGABYTE;

pub const MAX_ITEM_SIZE: usize = 1 * MEGABYTE;

#[derive(Debug)]
pub struct CacheConfig {
    pub max_item_size: usize,
    pub max_entries: usize,
}
