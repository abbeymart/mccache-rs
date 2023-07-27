use std::collections::HashMap;

pub type ValueType<T> = T;
pub struct CacheValueType<T> {
    pub value: T,
    pub expire: u128,
}
pub struct CacheResponseType<T> {
    pub ok: bool,
    pub message: String,
    pub value: T,
}

pub type HashCacheValueType<T> = HashMap<String, CacheValueType<T>>;

pub type SimpleCache<T> = HashMap<String, CacheValueType<T>>;

pub type HashCache<T> = HashMap<String, HashCacheValueType<T>>;


//secret keyCode for added security
pub const KEY_CODE: &str = "mcconnect_20200320";

// Initialise simple-cache object/dictionary (map)
pub static mut MC_SIMPLE_CACHE: SimpleCache<T> = SimpleCache::new();

// Initialise hash-cache object/dictionary (map)
pub static mut MC_HASH_CACHE: HashCache<T> = HashCache::new();
