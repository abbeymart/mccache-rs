# mccache | github.com/abbeymart/mccache-rs

- Simple and Hash In-Memory Information Cache
- Simple Cache: key-value pair storage
- Hash Cache: hash-hash-value storage
- See the test folder for test cases / scenarios and usage

## API - Simple Cache

- Simple Cache is key-value caching for number, string and object values
- It exposes the following functions: setCache, getCache, deleteCache and
  clearCache

## API - Hash Cache

- Hash Cache: hash-key-value storage for caching number, string and object
  values
- It exposes the following functions: setHashCache, getHashCache,
  deleteHashCache and clearHashCache

## Usage

```rust

use crate::cache_types::*;

// simple-cache
use crate::simple_cache::*;

// hash-cache
use crate::hash_cache::*;


// See the test folder for different test cases / scenarios and usage
```
