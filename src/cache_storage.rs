use crate::cache_types::*;

//secret keyCode for added security
pub const KEY_CODE: &str = "mcconnect_20200320";

// TODO: review variable generic <T> setting and use of unsafe static mut variables | consider safe alternative

// Initialise simple-cache object/dictionary (map)
pub static mut MC_SIMPLE_CACHE: SimpleCache<T> = SimpleCache::new();

// Initialise hash-cache object/dictionary (map)
pub static mut MC_HASH_CACHE: HashCache<T> = HashCache::new();

// pub unsafe fn cache_storage<T>(mc_cache: &mut SimpleCache<T>) -> &SimpleCache<T> {
// Create singleton cache storage for simple-cache - at application level
pub unsafe fn cache_storage<T>() -> &SimpleCache<T> {
    let mut mc_simple_cache = SimpleCache::new();
    return &mc_simple_cache
}

// Create singleton cache storage for hash-cache - at application level
pub fn hash_cache_storage<T>() -> &HashCache<T> {
    let mut mc_hash_cache = HashCache::new();
    return &mc_hash_cache
}