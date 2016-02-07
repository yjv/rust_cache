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

#[cfg(test)]
mod test {
    use super::HashMapCache;
    use super::time::Duration;
    use super::super::common::Cache;

    #[test]
    fn string_hash_map() {
        let value1: String = "hello".to_string();
        let value2: String = "goodbye".to_string();
        let mut cache = HashMapCache::new();
        let _ = cache.save(&"key1".to_string(), &value1, Duration::seconds(34));
        let _ = cache.save(&"key2".to_string(), &value2, Duration::weeks(12));
        assert_eq!(Ok(Some(value1)), Cache::<String>::fetch(&mut cache, &"key1".to_string()));
        assert_eq!(Ok(Some(value2)), Cache::<String>::fetch(&mut cache, &"key2".to_string()));
        assert_eq!(Ok(None), Cache::<String>::fetch(&mut cache, &"key3".to_string()));
    }
}