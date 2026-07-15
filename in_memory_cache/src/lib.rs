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

// impl<...Context> Cache<K,V,Context>: Whenver we add  a generic to a struct we must add it to the impl blocks declaration so the compiler knows it exists . Notice we don't need the =() default here defauls only go on the struct defintion.
// _marker: std::marker::PhantomData : Inside new() we actually have to instantiate the field . We just type std::marker::PhantomData . We don't even have to pass the <Context to it here because Rusts type inference is smart enough to figure it out>
impl<K: std::hash::Hash + std::cmp::Eq, V, Context> Cache<K, V, Context> {
    // Creates a fresh empty cache
    pub fn new() -> Self {
        Cache {
            store: HashMap::new(),
            _marker: std::marker::PhantomData,
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
// #[derive(Debug, Clone)]
// pub struct Cache<K: std::hash::Hash + std::cmp::Eq, V> {
//     store: HashMap<K, CacheItem<V>>, 
// }

// updated cache struct
// Context = () - We added a 3rd generic parameter named context . The =() syntax assigns a Default type parameter. IF a user types Cache<String, i32>. Rust automatically fills in the blank and treats it as Cache<String, i32, ()>. The () is the empty type (a unit type with no data)
// _marker: This is a new field we are adding to the struct. The underscore_  at the beginning tells the compiler . I am not actually going to use thhis field in my code so don't give me an unused variable warning
// std::marker::PhantomData<Context>: this is the zero byte ghost field . It tells the compiler to pretend we are storgin data of type Context inside the struct just so the compiler can enforce type safety rules.
pub struct Cache<K: std::hash::Hash + std::cmp::Eq , V, Context = ()> {
    store: HashMap<K, CacheItem<V>>,
    _marker: std::marker::PhantomData<Context>,
}