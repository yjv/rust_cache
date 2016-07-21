extern crate time;
extern crate crypto;
use std::collections::HashMap;
use time::Duration;
use time::Tm;
use std::fs;
use std::path;
use std::os::unix::fs::DirBuilderExt;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::error::Error as ErrorTrait;
use std::str::FromStr;
use std::string::ToString;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::any::Any;
use std::fmt::Error as FmtError;

#[cfg(test)]
mod test {
    use super::HashMapCache;
    use super::Cache;
    use time::Duration;
    use super::NullCache;

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

pub trait Cacheable: Sized {
    type ToError;
    type FromError;
    fn to_cache(&self) -> Result<String, Self::ToError>;
    fn from_cache(string: &String) -> Result<Self, Self::FromError>;
}

pub trait Cache<T: Cacheable> {
    type FetchError;
    type SaveError;
    type DeleteError;
    type ClearError;
    fn fetch(&mut self, key: &String) -> Result<Option<T>, Self::FetchError>;
    fn save(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::SaveError>;
    fn delete(&mut self, key: &String) -> Result<(), Self::DeleteError>;
    fn clear(&mut self) -> Result<(), Self::ClearError>;
}

impl<T: FromStr + ToString + Sized> Cacheable for T {
    type ToError = ();
    type FromError = T::Err;
    fn to_cache(&self) -> Result<String, Self::ToError> {
        Ok(self.to_string())
    }

    fn from_cache(string: &String) -> Result<Self, Self::FromError> {
        T::from_str(&string[..])
    }
}

pub struct HashMapCache {
    hash_map: HashMap<String, CacheEntry>
}

struct CacheEntry {
    pub string: String,
    pub expires: Tm
}

impl CacheEntry {
    fn expired(&self) -> bool {
        time::now_utc() > self.expires
    }
}

impl HashMapCache {
    pub fn new() -> Self {
        HashMapCache {
            hash_map: HashMap::new()
        }
    }
}
enum HashMapCacheError<T: Cacheable> {
    ItemFromCache(T::FromError),
    ItemToCache(T::ToError),
}

impl<T: Cacheable> Cache<T> for HashMapCache {
    type FetchError = HashMapCacheError<T>;
    type SaveError = HashMapCacheError<T>;
    type DeleteError = ();
    type ClearError = ();

    fn fetch(&mut self, key: &String) -> Result<Option<T>, Self::FetchError> {
        Ok(if let Some(entry) = self.hash_map.get(key) {
            if entry.expired() {
                None
            } else {
                Some(try!(T::from_cache(&entry.string).map_err(HashMapCacheError::ItemFromCache)))
            }
        } else {
            None
        })
    }

    fn save(&mut self, key: &String, item: &T, ttl: Duration) -> Result<(), Self::SaveError> {
        self.hash_map.insert(
            key.clone(),
            CacheEntry {
                string: try!(item.to_cache().map_err(HashMapCacheError::ItemToCache)),
                expires: time::now() + ttl
            }
        );
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), Self::DeleteError> {
        self.hash_map.remove(key);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::ClearError> {
        self.hash_map.clear();
        Ok(())
    }
}

pub struct NullCache;

impl<T: Cacheable> Cache<T> for NullCache {
    type FetchError = ();
    type SaveError = ();
    type DeleteError = ();
    type ClearError = ();

    fn fetch(&mut self, _: &String) -> Result<Option<T>, Self::FetchError> {
        Ok(None)
    }

    fn save(&mut self, _: &String, _: &T, _: Duration) -> Result<(), Self::SaveError> {
        Ok(())
    }

    fn delete(&mut self, _: &String) -> Result<(), Self::DeleteError> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::ClearError> {
        Ok(())
    }
}

pub struct FilesystemCache {
    directory: path::PathBuf,
    extension: String,
    umask: u16
}

impl FilesystemCache {
    pub fn new(directory: String, extension: String, umask: u16) -> Result<FilesystemCache, String> {
        let path = path::Path::new(&directory[..]);

        if !path.is_dir() {
            let _ = try!(fs::DirBuilder::new().recursive(true).mode(umask).create(directory.clone()).map_err(|e| format!("{}", e)));
        }

        if path.metadata().unwrap().permissions().readonly() {
            Err("directory is read only".to_string())
        } else {
            Ok(FilesystemCache {
                directory: try!(path.canonicalize().map_err(|e| format!("{}", e))),
                extension: extension,
                umask: umask
            })
        }
    }

    fn get_file_path(&self, key: &String) -> path::PathBuf {
        let mut sha1 = Sha1::new();
        sha1.input_str(&key[..]);
        let sha1_result = sha1.result_str();
        let (part1, part2) = sha1_result.split_at(2);
        let mut buffer = self.directory.clone();
        buffer.push(part1);
        buffer.push(part2);
        buffer.push(key);
        buffer.set_extension(self.extension.clone());
        buffer
    }
}

//impl<T: Cacheable, U: ErrorTrait> Cache<T, U> for FilesystemCache {
//    fn fetch(&mut self, key: &String) -> Result<Option<T>, U> {
//        let path = self.get_file_path(key);
//
//        if !path.is_file() {
//            return Ok(None);
//        }
//
//        let file = try!(fs::File::open(path));
//        let string = String::new();
//        try!(file.read_to_string(&mut string));
//        let parts: Vec<String> = string.splitn(2, '\n');
//        let entry =
//    }
//
//    fn save(&mut self, key: &String, value: &T, ttl: Duration) -> Result<(), U> {
//        Ok(())
//    }
//
//    fn delete(&mut self, key: &String) -> Result<(), U> {
//        Ok(())
//    }
//
//    fn clear(&mut self) -> Result<(), U> {
//        Ok(())
//    }
//}