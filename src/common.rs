extern crate time;
use self::time::Duration;
use self::time::Tm;
use std::str::FromStr;

pub trait Cacheable: Sized {
    type Error;
    fn to_cache(&self) -> Result<String, Self::Error>;
    fn from_cache(string: &String) -> Result<Self, Self::Error>;
}

pub trait Cache<T: Cacheable> {
    type Error;
    fn fetch(&mut self, key: &String) -> Result<Option<T>, Self::Error>;
    fn save(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::Error>;
    fn delete(&mut self, key: &String) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
}

impl<T: FromStr + ToString + Sized> Cacheable for T {
    type Error = T::Err;
    fn to_cache(&self) -> Result<String, Self::Error> {
        Ok(self.to_string())
    }

    fn from_cache(string: &String) -> Result<Self, Self::Error> {
        T::from_str(&string[..])
    }
}

pub struct CacheEntry {
    pub string: String,
    pub expires: Tm
}

impl CacheEntry {
    pub fn expired(&self) -> bool {
        time::now_utc() > self.expires
    }
}

pub struct NullCache;

impl<T: Cacheable> Cache<T> for NullCache {
    type Error = ();
    fn fetch(&mut self, _: &String) -> Result<Option<T>, Self::Error> {
        Ok(None)
    }

    fn save(&mut self, _: &String, _: &T, _: Duration) -> Result<(), Self::Error> {
        Ok(())
    }

    fn delete(&mut self, _: &String) -> Result<(), Self::Error> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}