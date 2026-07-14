use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct CacheItem<V> {
    value: V,
    expires_at: Option<Instant>,
}

impl<V> CacheItem<V> {
    // A helper method to easily check if this specific item has expired
    fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(time) => Instant::now() >= time, // Expired if right now is past the expiration time
            None => false, // Never expires

        }
    }
}

impl<K: std::hash::Hash + std::cmp::Eq, V> Cache<K, V> {
    // Creates a fresh empty cache
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    // Inserts a new item into the cache
    pub fn set(&mut self, key: K, value:V, ttl:Option<Duration>) {
        // Calculate exactly what time on the stopwatch this items should expire
        let expires_at = match ttl {
            Some(duration) => Some(Instant::now() + duration),
            None => None, // No TTL provided so it lives forever
        };

        // Wrap it in our struct and insert it into the HashMap
        let item = CacheItem { value , expires_at };
        self.store.insert(key, itemm);
    }
}


// Our main Cache structure. Notice the Trait bounds on K!
#[derive(Debug, Clone)]
pub struct Cache<K: std::hash::Hash + std::cmp::Eq, V> {
    store: HashMap<K, CacheItem<V>>, 
}