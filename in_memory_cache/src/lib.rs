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
        self.store.insert(key, item);
    }

    // Retrieves an item from the cache. Returns None if it doesn't exist Or if it expired
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // 1. Check if it exists in the HashMap at all
            if let Some(item) = self.store.get(key) {
                // Check if it has epxired
                if item.is_expired() {
                    // It expired Delte it right now so it stops taking up memory
                    self.store.remove(key);
                    return None;
                }
                // It exists and is valid. Return the value
            // We have to use .get() again because we can't return a referenc3
            // to something while simultaneously holding a mutable reference to remove it
            return self.store.get(key).map(|i| &i.value);
            }

            None
    }

    // &mut self, - we need mutable access to the cache because we are deleting data
    // key: &K - We take a reference to the key just looking at it so we know what to delete
    // self.store.remove(key) - Calls the built in HashMap method to instatnly drop the item from memory
    pub fn delete(&mut self, key: &K) {
        self.store.remove(key);
    }


    // &mut self - We need mutable access because we are modifying the HashMap
    // self.store.retain(...), .retain() is a powerful built in method for HashMap and Vectors. 
    // It loops over every single item. If the closure inside it retruns true , it keeps the item . If it returns false, it deletes the item
    // |_key, item| - this is our closure . It takes two arguments from teh HashMap :the key and teh value item. We put an underscore _key to tell the Rust compilre I know the key is here but I am purposely ignoring it don't give me an unused variable warining
    // !item.is_expired() :The ! means NOT. So we are saying "Retain this item only if it is not expired"
    pub fn cleanup_expired(&mut self) {
        self.store.retain(|_key, item| !item.is_expired());
    }
}


// Our main Cache structure. Notice the Trait bounds on K!
#[derive(Debug, Clone)]
pub struct Cache<K: std::hash::Hash + std::cmp::Eq, V> {
    store: HashMap<K, CacheItem<V>>, 
}