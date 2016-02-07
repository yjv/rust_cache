
#[cfg(test)]
mod test {
    use super::hash_map::HashMapCache;
    use super::common::Cache;
    use time::Duration;
    use super::common::NullCache;

    #[test]
    fn string_hash_map() {
        let value1: String = "hello".to_string();
        let value2: String = "goodbye".to_string();
        let mut cache = HashMapCache::new();
        let _ = cache.save(&"key1".to_string(), &value1, Duration::seconds(34));
        let _ = cache.save(&"key2".to_string(), &value2, Duration::weeks(12));
        assert_eq!(value1, Cache::<String>::fetch(&mut cache, &"key1".to_string()).unwrap().unwrap());
        assert_eq!(value2, Cache::<String>::fetch(&mut cache, &"key2".to_string()).unwrap().unwrap());
        assert_eq!(None, Cache::<String>::fetch(&mut cache, &"key3".to_string()).unwrap());
    }

    #[test]
    fn null_cache() {
        let mut cache = NullCache;
        assert_eq!(Ok(()), cache.save(&"key2".to_string(), &"value1".to_string(), Duration::seconds(34)));
        assert_eq!(Ok(None), Cache::<String>::fetch(&mut cache, &"key1".to_string()));
        assert_eq!(Ok(()), Cache::<String>::delete(&mut cache, &"key3".to_string()));
        assert_eq!(Ok(()), Cache::<String>::clear(&mut cache));
    }
}

pub mod common;
pub mod hash_map;
pub mod filesystem;
