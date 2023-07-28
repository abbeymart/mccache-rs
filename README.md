# mccache | github.com/abbeymart/mccache-rs

- Simple and Hash In-Memory Information Cache
- Simple Cache: key-value pair storage
- Hash Cache: hash-hash-value storage
- See the test folder for test cases / scenarios and usage

## API - Simple Cache

- Simple Cache: key-value caching for rust-lang supported generic types - e.g. number, string and object values
- It exposes the following functions: set_cache, get_cache, delete_cache and clear_cache

## API - Hash Cache

- Hash Cache: hash-key-value storage for caching for rust-lang supported generic types - e.g. number, string and object values
- It exposes the following functions: set_hash_cache, get_hash_cache, delete_hash_cache and clear_hash_cache

## Usage

```rust

use crate::cache_types::*;

// simple-cache
use crate::simple_cache::*;

// hash-cache
use crate::hash_cache::*;


// See the test folder for different test cases / scenarios and usage
```
