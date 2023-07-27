use crate::cache_types::*;
// use crate::cache_storage::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub unsafe fn set_cache<T>(key: String, value: T, expire: u128) -> CacheResponseType<T> {
    // validate required params
    if key == "" || value == None {
        return CacheResponseType{
            ok: false,
            message: "cache-key and value are required".to_string(),
            value: ()
        }
    }
    // expire default Value (in seconds)
    let mut cache_expire = expire.clone();
    if expire == 0 {
        cache_expire = 300;
    }

    // current time in millisecond to set cache expiry time
    let mut current_time_in_milli_seconds: u128 = 0;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Some(val) => {
            current_time_in_milli_seconds = val.as_millis();
        }
        _ => {
            let msg: String = format!("unable to set cache value - error setting current time for cache-expiration");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
    // TODO: review unsafe variable | e.g. use storage singleton
    // compute secure cache-key
    let cache_key = format!("{}{}", key, KEY_CODE);
    let cache_key_clone = cache_key.clone();
    // TODO: transform expire u64 to expire + time.Now
    let cache_value = CacheValueType{value, expire: cache_expire * 1000 + current_time_in_milli_seconds };
    MC_SIMPLE_CACHE.insert(cache_key_clone, cache_value);

    // validate value caching
    match MC_SIMPLE_CACHE.get(&cache_key) {
        Some(val) => {
            let msg: String = format!("task completed successfully");
            CacheResponseType{ok: true, message: msg, value: val }
        },
        None => {
            let msg: String = format!("unable to set cache value");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
}

pub unsafe fn get_cache<T>(key: String) -> CacheResponseType<T> {
    // validate required params
    if key == "" {
        return CacheResponseType{
            ok: false,
            message: "cache-key is required".to_string(),
            value: ()
        }
    }
    // compute secure cache-key
    let cache_key = format!("{}{}", key, KEY_CODE);
    // current time in millisecond to set cache expiry time
    let mut current_time_in_milli_seconds: u128 = 0;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Some(val) => {
            current_time_in_milli_seconds = val.as_millis();
        }
        _ => {
            let msg: String = format!("unable to set cache value - error setting current time for cache-expiration");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
    // fetch cache value from shared memory storage
    match MC_SIMPLE_CACHE.get(&cache_key) {
        Some(val)  => if val.value != None && val.expire > current_time_in_milli_seconds {
            let msg: String = format!("task completed successfully");
            CacheResponseType{ok: true, message: msg, value: val }
        } else if val.value != None && val.expire < current_time_in_milli_seconds {
            MC_SIMPLE_CACHE.remove(&cache_key);
            let msg: String = format!("cache expired and deleted");
            CacheResponseType{ok: true, message: msg, value: () }
        } else {
            let msg: String = format!("cache info does not exist");
            CacheResponseType{ok: true, message: msg, value: () }
        },
        None => {
            let msg: String = format!("cache info does not exist");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
}

pub unsafe fn delete_cache<T>(key: String) -> CacheResponseType<T> {
    // validate required params
    if key == "" {
        return CacheResponseType{
            ok: false,
            message: "cache-key is required".to_string(),
            value: ()
        }
    }
    let cache_key = format!("{}{}", key, KEY_CODE);
    match MC_SIMPLE_CACHE.remove(&cache_key) {
        Some(val) => {
            let msg: String = format!("task completed successfully");
            CacheResponseType{ok: true, message: msg, value: val }
        },
        None => {
            let msg: String = format!("task not completed, cache-key-value not found");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
}

pub unsafe fn clear_cache<T>() -> CacheResponseType<T> {
    MC_SIMPLE_CACHE.clear();
    let msg: String = format!("task completed successfully");
    CacheResponseType{ok: true, message: msg, value: ()}
}
