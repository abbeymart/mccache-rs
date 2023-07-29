#[path = "./cache_types.rs"]
mod cache_types;
#[path = "./simple_cache.rs"]
mod simple_cache;
#[path = "./hash_cache.rs"]
mod hash_cache;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
