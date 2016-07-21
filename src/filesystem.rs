extern crate crypto;
extern crate time;
use std::fs;
use std::path;
#[cfg(all(unix))]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;
use std::str::FromStr;
use common::{Cacheable, Cache, CacheEntry, ParseCacheEntryError};
use self::time::Duration;
use std::convert::From;
use std::io::{Read, Write};
use std::any::Any;

///Filesystem cache just takes a dir at mimimum and allows you to use that dir
/// as a caching system
pub struct FilesystemCache {
    directory: path::PathBuf,
    extension: String,
    umask: u16
}

#[derive(Debug)]
pub enum Error {
    ExistsButIsNotDirectory,
    DirectoryNotWritable,
    Io(::std::io::Error),
    CacheEntryFailedToParse(ParseCacheEntryError),
    CacheSerializationFailure(Box<Any>)
}

impl From<::std::io::Error> for Error {
    fn from(error: ::std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<ParseCacheEntryError> for Error {
    fn from(error: ParseCacheEntryError) -> Self {
        Error::CacheEntryFailedToParse(error)
    }
}

impl FilesystemCache {
    pub fn new(directory: String) -> Result<FilesystemCache, Error> {
        Self::new_with_extension(directory, "cache".to_string())
    }

    pub fn new_with_extension(directory: String, extension: String) -> Result<FilesystemCache, Error> {
        Self::new_with_extension_and_umask(directory, extension, 0o002)
    }

    pub fn new_with_extension_and_umask(directory: String, extension: String, umask: u16) -> Result<FilesystemCache, Error> {
        let path = path::Path::new(&directory[..]);

        if path.exists() && !path.is_dir() {
            return Err(Error::ExistsButIsNotDirectory);
        }

        if !path.is_dir() {
            let mut builder = fs::DirBuilder::new();

            if cfg!(all(unix)) {
                builder.mode(0o777 & !umask);
            }
            try!(builder.recursive(true).create(directory.clone()));
        }

        if path.metadata().unwrap().permissions().readonly() {
            Err(Error::DirectoryNotWritable)
        } else {
            Ok(FilesystemCache {
                directory: path.canonicalize().unwrap(),
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

impl Cache for FilesystemCache {
    type Error = Error;
    fn fetch<T: Cacheable>(&mut self, key: &String) -> Result<Option<T>, Self::Error> {
        let path = self.get_file_path(key);

        if !path.is_file() {
            return Ok(None);
        }

        let mut file = try!(fs::File::open(path.clone()));
        let mut string = String::new();
        try!(file.read_to_string(&mut string));
        let entry = try!(CacheEntry::from_str(&string[..]));

        if entry.expired() {
            return Ok(try!(fs::remove_file(path).map(|_| None)));
        }

        Ok(Some(try!(T::from_cache(&entry.string).map_err(|e| Error::CacheSerializationFailure(Box::new(e))))))
    }

    fn save<T: Cacheable>(&mut self, key: &String, value: &T, ttl: Duration) -> Result<(), Self::Error> {
        let path = self.get_file_path(key);
        let mut tmp_path = path.clone();
        let mut new_file_name = tmp_path.file_name().unwrap().to_owned();
        new_file_name.push("-new");
        tmp_path.set_file_name(new_file_name);
        let mut open_options = fs::OpenOptions::new();
        let mut builder = fs::DirBuilder::new();

        if cfg!(all(unix)) {
            open_options.mode(0777 & !self.umask);
            builder.mode(0o777 & !self.umask);
        }

        try!(builder.recursive(true).create(tmp_path.parent().unwrap()));
        let mut file = try!(open_options.create(true).write(true).create(true).truncate(true).open(tmp_path.clone()));
        let entry = CacheEntry::new(try!(value.to_cache().map_err(|e| Error::CacheSerializationFailure(Box::new(e)))), ttl);
        let _ = try!(file.write(&entry.to_string().into_bytes()));

        try!(fs::rename(tmp_path, path));
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), Self::Error> {
        let path = self.get_file_path(key);

        if !path.is_file() {
            return Ok(());
        }

        Ok(try!(fs::remove_file(path)))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        for child in try!(fs::read_dir(self.directory.clone())) {
            let child = try!(child).path();
            let stat = try!(fs::symlink_metadata(&*child));
            if stat.is_dir() {
                try!(fs::remove_dir_all(&*child));
            } else {
                try!(fs::remove_file(&*child));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::FilesystemCache;
    use super::time::Duration;
    use super::super::common::Cache;
    use std::fs::remove_dir_all;

    #[test]
    fn string_filesystem() {
        let value1: String = "hello".to_string();
        let value2: String = "goodbye".to_string();
        let mut cache = FilesystemCache::new("hello".to_string()).unwrap();
        assert_eq!((), cache.save(&"key1".to_string(), &value1, Duration::seconds(34)).unwrap());
        assert_eq!((), cache.save(&"key2".to_string(), &value2, Duration::weeks(12)).unwrap());
        assert_eq!(Some(value1), Cache::fetch::<String>(&mut cache, &"key1".to_string()).unwrap());
        assert_eq!(Some(value2), Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key3".to_string()).unwrap());
        Cache::delete(&mut cache, &"key".to_string()).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key".to_string()).unwrap());
        Cache::clear(&mut cache).unwrap();
        assert_eq!(None, Cache::fetch::<String>(&mut cache, &"key2".to_string()).unwrap());
        let _ = remove_dir_all("hello");
    }
}