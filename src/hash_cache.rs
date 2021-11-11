use crate::cache_types::*;
use crate::cache_storage::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub unsafe fn set_hash_cache<T>(key: String, hash: String, value: T, expire: u128) -> CacheResponseType<T> {
    // validate required params
    if key == "" || hash == "" || value == None {
        return CacheResponseType{
            ok: false,
            message: "hash, cache-key and value are required".to_string(),
            value: ()
        }
    }
    // expire default Value (in seconds)
    let mut cache_expire = expire.clone();
    if expire == 0 {
        cache_expire = 300;
    }
    let current_time_in_milli_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    // TODO: review unsafe variable | e.g. use storage singleton
    // compute secure cache-key and hash-key
    let hash_key = format!("{}{}", hash, KEY_CODE);
    let cache_key = format!("{}{}", key, KEY_CODE);
    let mut hash_value: HashCacheValueType<T> = HashCacheValueType::new();
    let cache_key_clone = cache_key.clone();
    let hash_key_clone = hash_key.clone();
    // let val = value.clone();
    let cache_value = CacheValueType{value, expire: cache_expire * 1000 + current_time_in_milli_seconds };
    hash_value.insert(hash_key_clone, cache_value);
    MC_HASH_CACHE.insert(cache_key_clone, hash_value);
    // validate value caching
    match MC_HASH_CACHE.get(&cache_key) {
        Some(val) => {
            let msg: String = format!("task completed successfully");
            CacheResponseType{ok: true, message: msg, value: val }
        },
        Err(_) => {
            let msg: String = format!("unable to set cache value");
            CacheResponseType{ok: true, message: msg, value: val }
        }
    }
}

pub unsafe fn get_hash_cache<T>(key: String, hash: String) -> CacheResponseType<T> {
    // validate required params
    if key == "" || hash == "" {
        return CacheResponseType{
            ok: false,
            message: "hash and cache-key are required".to_string(),
            value: ()
        }
    }
    // compute secure cache-key
    let hash_key = format!("{}{}", hash, KEY_CODE);
    let cache_key = format!("{}{}", key, KEY_CODE);
    // let cache_key_clone = cache_key.clone();
    let current_time_in_milli_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match MC_HASH_CACHE.get(&hash_key) {
        Some(kVal) => match kVal.get(&cache_key) {
            Some(val)  => if val.value != None && val.expire > current_time_in_milli_seconds {
                let msg: String = format!("task completed successfully");
                CacheResponseType{ok: true, message: msg, value: val }
            } else if val.value != None && val.expire < current_time_in_milli_seconds {
                // remove cache-item
                match MC_HASH_CACHE[hash_key].remove(&cache_key) {
                    Some(val) => {
                        let msg: String = format!("cache expired and deleted");
                        CacheResponseType{ok: true, message: msg, value: val }
                    },
                    None => {
                        let msg: String = format!("cache info does not exist");
                        CacheResponseType{ok: true, message: msg, value: () }
                    }
                }
            } else {
                let msg: String = format!("cache info does not exist");
                CacheResponseType{ok: true, message: msg, value: () }
            },
            Err(_) => {
                let msg: String = format!("cache info does not exist");
                CacheResponseType{ok: true, message: msg, value: () }
            }
        },
        Err(_) => {
            let msg: String = format!("cache info does not exist");
            CacheResponseType{ok: true, message: msg, value: () }
        }
    }
}

pub unsafe fn delete_hash_cache<T>(key: String, hash: String, by: String) -> CacheResponseType<T> {
    // validate required params
    if key == "" || hash == "" || value == None {
        return CacheResponseType{
            ok: false,
            message: "hash, cache-key and value are required".to_string(),
            value: ()
        }
    }
    let mut del_by = "hash".to_string();
    if by != "key" {
        del_by = by.to_string();
    }
    if key == "" || hash == "" && by == "key" {
        return CacheResponseType{
            ok: false,
            message: "hash and cache keys are required".to_string(),
            value: ()
        }
    }
    if hash == "" && by == "hash" {
        return CacheResponseType{
            ok: false,
            message: "hash key is required".to_string(),
            value: ()
        }
    }
    // compute secure cache-key
    let hash_key = format!("{}{}", hash, KEY_CODE);
    let cache_key = format!("{}{}", key, KEY_CODE);

    if del_by == "hash" {
        match MC_HASH_CACHE.remove(&cache_key) {
            Some(val) => {
                let msg: String = format!("task completed successfully");
                CacheResponseType{ok: true, message: msg, value: val }
            },
            None => {
                let msg: String = format!("task not completed, hash-value not found");
                CacheResponseType{ok: true, message: msg, value: val }
            }
        }
    }

    if del_by == "key" {
        match MC_HASH_CACHE.get(&hash_key) {
            Some(_) => {
                match MC_HASH_CACHE[hash_key].remove(&cache_key) {
                    Some(val) => {
                        let msg: String = format!("task completed successfully");
                        CacheResponseType{ok: true, message: msg, value: val }
                    },
                    None => {
                        let msg: String = format!("task not completed, hash-value not found");
                        CacheResponseType{ok: true, message: msg, value: () }
                    }
                }
            },
            None => {
                let msg: String = format!("task not completed, hash-value not found");
                CacheResponseType{ok: true, message: msg, value: () }
            }
        }
    }

    let msg: String = format!("testing");
    CacheResponseType{ok: true, message: msg, value: nil}
}

pub unsafe fn clear_hash_cache<T>() -> CacheResponseType<T> {
    MC_HASH_CACHE.clear();
    let msg: String = format!("task completed successfully");
    CacheResponseType{ok: true, message: msg, value: ()}
}