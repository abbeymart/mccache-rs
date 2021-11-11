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
