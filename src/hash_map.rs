extern crate time;
use self::time::Duration;
use std::collections::HashMap;
use super::common::CacheEntry;
use super::common::Cache;
use super::common::Cacheable;

pub struct HashMapCache {
    hash_map: HashMap<String, CacheEntry>
}

impl HashMapCache {
    pub fn new() -> Self {
        HashMapCache {
            hash_map: HashMap::new()
        }
    }
}

impl<T: Cacheable> Cache<T> for HashMapCache {
    type Error = T::Error;
    fn fetch(&mut self, key: &String) -> Result<Option<T>, Self::Error> {
        Ok(if let Some(entry) = self.hash_map.get(key) {
            if entry.expired() {
                None
            } else {
                Some(try!(T::from_cache(&entry.string)))
            }
        } else {
            None
        })
    }

    fn save(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::Error> {
        self.hash_map.insert(
            key.clone(),
            CacheEntry {
                string: try!(item.to_cache()),
                expires: time::now() + ttl
            }
        );
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), Self::Error> {
        self.hash_map.remove(key);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.hash_map.clear();
        Ok(())
    }
}