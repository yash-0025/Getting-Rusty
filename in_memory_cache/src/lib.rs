use std::collections::HashMap;
use std::time::{ Duration, Instant };


#[derive(Debug, Clone)]
struct CacheItem<V> {
    value: V,
    expires_at: Option<Instant>,
}

impl<v> Cacheitem<V> {
    // A helper method to easily check if this specific item has expired
    fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(time) => Instant::now() >= time, // Expired if right now is past the expiration time
            None =>false, // never expires
        }
    }
}

// Our main cache structure Notice the Trait bound on K!
#[derive(Debug, Clone)]
pub struct Cache<K: std::hash::Hash + std::cmp::Eq, V> {
    store: HashMap<K, CacheItem<V>>
}