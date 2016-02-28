extern crate redis;
extern crate time;
use super::common::{Cacheable, Cache};
use self::time::Duration;
use self::redis::{Connection, Commands, RedisError};
use std::convert::From;
use std::any::Any;

///This impl allow syou to use a redis backend for caching
pub struct RedisCache<'a> {
    connection: &'a Connection
}

#[derive(Debug)]
pub enum Error {
    CacheSerializationFailure(Box<Any>),
    RedisError(RedisError)
}

impl From<RedisError> for Error {
    fn from(error: RedisError) -> Self {
        Error::RedisError(error)
    }
}

impl<'a> RedisCache<'a> {
    pub fn new(connection: &'a Connection) -> RedisCache<'a> {
        RedisCache {
            connection: connection
        }
    }
}

impl<'a> Cache for RedisCache<'a> {
    type Error = Error;
    fn fetch<T: Cacheable>(&mut self, key: &String) -> Result<Option<T>, Self::Error> {
        if let Some(ref value) = try!(self.connection.get(&key[..])) {
            Ok(Some(try!(T::from_cache(value).map_err(|e| Error::CacheSerializationFailure(Box::new(e))))))
        } else {
            Ok(None)
        }
    }

    fn save<T: Cacheable>(&mut self, key: &String, value: &T, ttl: Duration) -> Result<(), Self::Error> {
        try!(self.connection.set_ex(key.clone(), try!(value.to_cache().map_err(|e| Error::CacheSerializationFailure(Box::new(e)))), ttl.num_seconds() as usize));
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), Self::Error> {
        try!(self.connection.del(key.clone()));
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        try!(redis::cmd("FLUSHDB").query(self.connection));
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::redis;
    use super::RedisCache;
    use super::time::Duration;
    use super::super::common::Cache;

    #[test]
    fn string_redis() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let connection = client.get_connection().unwrap();

        let value1: String = "hello".to_string();
        let value2: String = "goodbye".to_string();
        let mut cache = RedisCache::new(&connection);
        let _ = cache.save(&"key1".to_string(), &value1, Duration::seconds(34)).unwrap();
        let _ = cache.save(&"key2".to_string(), &value2, Duration::weeks(12)).unwrap();
        assert_eq!(Some(value1), Cache::fetch::<String>(&mut cache, &"key1".to_string()).unwrap());
        assert_eq!(Some(value2), Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key3".to_string()).unwrap());
        Cache::delete(&mut cache, &"key".to_string()).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key".to_string()).unwrap());
        Cache::clear(&mut cache).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
    }
}