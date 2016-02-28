extern crate time;
use self::time::Duration;
use std::collections::HashMap;
use super::common::CacheEntry;
use super::common::Cache;
use super::common::Cacheable;
use std::any::Any;

pub struct HashMapCache {
    hash_map: HashMap<String, CacheEntry>
}

#[derive(Debug)]
pub enum Error {
    CacheSerializationFailure(Box<Any>)
}

///This cache allows for in memory caching. this can be useful for really simple
/// caching requirements where you dont want to have to use a filesystem or some external
/// storage but still want caching
impl HashMapCache {
    pub fn new() -> Self {
        HashMapCache {
            hash_map: HashMap::new()
        }
    }
}

impl Cache for HashMapCache {
    type Error = Error;
    fn fetch<T: Cacheable>(&mut self, key: &String) -> Result<Option<T>, Self::Error> {
        Ok(if let Some(entry) = self.hash_map.get(key) {
            if entry.expired() {
                None
            } else {
                Some(try!(T::from_cache(&entry.string).map_err(|e| Error::CacheSerializationFailure(Box::new(e)))))
            }
        } else {
            None
        })
    }

    fn save<T: Cacheable>(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::Error> {
        self.hash_map.insert(
            key.clone(),
            CacheEntry::new(try!(item.to_cache().map_err(|e| Error::CacheSerializationFailure(Box::new(e)))), ttl)
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
        assert_eq!(Some(value1), Cache::fetch::<String>(&mut cache, &"key1".to_string()).unwrap());
        assert_eq!(Some(value2), Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key3".to_string()).unwrap());
        Cache::delete(&mut cache, &"key".to_string()).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key".to_string()).unwrap());
        Cache::clear(&mut cache).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
    }
}