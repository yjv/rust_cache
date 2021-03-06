extern crate time;
use self::time::{Duration, Tm, Timespec, now};
use std::str::FromStr;
use std::convert::From;
use std::any::Any;

///
/// Trait for items that want to be cacheable
/// allows for special to string method for when they get cached
/// all items already able to be converted to a from a string have their default impl
pub trait Cacheable: Sized {
    type Error: Any;
    fn to_cache(&self) -> Result<String, Self::Error>;
    fn from_cache(string: &String) -> Result<Self, Self::Error>;
}

///Trait to implement for actual cache implementations
pub trait Cache {
    type Error;
    fn fetch<T: Cacheable>(&mut self, key: &String) -> Result<Option<T>, Self::Error>;
    fn save<T: Cacheable>(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::Error>;
    fn delete(&mut self, key: &String) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
}

impl<T: FromStr + ToString + Sized> Cacheable for T where T::Err: Any {
    type Error = T::Err;
    fn to_cache(&self) -> Result<String, Self::Error> {
        Ok(self.to_string())
    }

    fn from_cache(string: &String) -> Result<Self, Self::Error> {
        T::from_str(&string[..])
    }
}

///This is a useful struct for use inside a cache implentation to keep track
/// of the data and the expiration together. it can be converted to a form a string iteself for storage in
/// any medium that accepts strings
#[derive(Debug, Eq, PartialEq)]
pub struct CacheEntry {
    pub string: String,
    pub expires: Tm
}

impl CacheEntry {
    pub fn new(string: String, ttl: Duration) -> Self {
        CacheEntry {
            string: string,
            expires: time::now() + ttl
        }
    }
    pub fn expired(&self) -> bool {
        time::now_utc() > self.expires
    }
}

impl ToString for CacheEntry {
    fn to_string(&self) -> String {
        let timespec = self.expires.to_timespec();
        format!("{},{}\n{}", timespec.sec, timespec.nsec, self.string)
    }
}

impl FromStr for CacheEntry {
    type Err = ParseCacheEntryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.splitn(3, |c| c == ',' || c == '\n').collect();
        Ok(CacheEntry {
            string: try!(data.get(2).ok_or(ParseCacheEntryError::NotEnoughParts)).to_string(),
            expires: time::at(Timespec::new(
                try!(i64::from_str(data[0])),
                try!(i32::from_str(data[1]))
            ))
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseCacheEntryError {
    NotEnoughParts,
    TimespecParseError(::std::num::ParseIntError)
}

impl From<::std::num::ParseIntError> for ParseCacheEntryError {
    fn from(error: ::std::num::ParseIntError) -> Self {
        ParseCacheEntryError::TimespecParseError(error)
    }
}

///This sturct is for use in places that require a Cache impl but you dont want to actually
/// cache anything. exmples could be testing and such
pub struct NullCache;

impl Cache for NullCache {
    type Error = ();
    fn fetch<T>(&mut self, _: &String) -> Result<Option<T>, Self::Error> {
        Ok(None)
    }

    fn save<T>(&mut self, _: &String, _: &T, _: Duration) -> Result<(), Self::Error> {
        Ok(())
    }

    fn delete(&mut self, _: &String) -> Result<(), Self::Error> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::time::Duration;
    use super::NullCache;
    use super::Cache;
    use super::time::now;
    use super::CacheEntry;
    use std::str::FromStr;
    use super::ParseCacheEntryError;

    #[test]
    fn null_cache() {
        let mut cache = NullCache;
        assert_eq!(Ok(()), cache.save(&"key2".to_string(), &"value1".to_string(), Duration::seconds(34)));
        assert_eq!(Ok(None), Cache::fetch::<String>(&mut cache, &"key1".to_string()));
        assert_eq!(Ok(()), Cache::delete(&mut cache, &"key3".to_string()));
        assert_eq!(Ok(()), Cache::clear(&mut cache));
    }

    #[test]
    fn cache_entry_string_conversion() {
        let expires = now();
        let entry = CacheEntry {
            string: "hello".to_string(),
            expires: expires
        };
        let string = entry.to_string();
        assert_eq!(format!("{},{}\nhello", expires.to_timespec().sec, expires.to_timespec().nsec), string);
        let new_entry = CacheEntry::from_str(&string[..]).unwrap();
        assert_eq!(entry.string, new_entry.string);
        assert_eq!(entry.expires, new_entry.expires);
        assert_eq!(Err(ParseCacheEntryError::NotEnoughParts), CacheEntry::from_str("21,21"));
        assert_eq!(Err(ParseCacheEntryError::NotEnoughParts), CacheEntry::from_str("21"));
        assert_eq!(Err(ParseCacheEntryError::NotEnoughParts), CacheEntry::from_str(""));
        match CacheEntry::from_str("hello,21\nsdffdssdf") {
            Err(ParseCacheEntryError::TimespecParseError(_)) => (),
            _ => panic!("cache entry did not return a parseint error on from_str call")
        };
        match CacheEntry::from_str("21,hello\nreyrtyrt") {
            Err(ParseCacheEntryError::TimespecParseError(_)) => (),
            _ => panic!("cache entry did not return a parseint error on from_str call")
        };
    }
}