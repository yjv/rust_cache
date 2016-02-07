extern crate crypto;
extern crate time;
use std::fs;
use std::path;
use std::os::unix::fs::DirBuilderExt;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;
use std::str::FromStr;
use common::Cacheable;
use common::Cache;
use common::CacheEntry;
use common::ParseCacheEntryError;
use self::time::Duration;
use std::convert::From;
use std::io::Read;
use std::io::Write;

pub struct FilesystemCache {
    directory: path::PathBuf,
    extension: String,
    umask: u16
}

pub enum Error<T> {
    ExistsButIsNotDirectory,
    DirectoryNotWritable,
    Io(::std::io::Error),
    CacheEntryFailedToParse(ParseCacheEntryError),
    CacheSerializationFailure(T)
}

impl<T> From<::std::io::Error> for Error<T> {
    fn from(error: ::std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl<T> From<ParseCacheEntryError> for Error<T> {
    fn from(error: ParseCacheEntryError) -> Self {
        Error::CacheEntryFailedToParse(error)
    }
}

impl FilesystemCache {
    pub fn new<T>(directory: String, extension: String, umask: u16) -> Result<FilesystemCache, Error<T>> {
        let path = path::Path::new(&directory[..]);

        if path.exists() && !path.is_dir() {
            return Err(Error::ExistsButIsNotDirectory);
        }

        if !path.is_dir() {
            let _ = try!(fs::DirBuilder::new().recursive(true).mode(umask).create(directory.clone()));
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

impl<T: Cacheable> Cache<T> for FilesystemCache {
    type Error = Error<<T as Cacheable>::Error>;
    fn fetch(&mut self, key: &String) -> Result<Option<T>, Self::Error> {
        let path = self.get_file_path(key);

        if !path.is_file() {
            return Ok(None);
        }

        let mut file = try!(fs::File::open(path));
        let mut string = String::new();
        try!(file.read_to_string(&mut string));
        let entry = try!(CacheEntry::from_str(&string[..]));

        if entry.expired() {
            return Ok(None);
        }

        Ok(Some(try!(T::from_cache(&entry.string).map_err(Error::CacheSerializationFailure))))
    }

    fn save(&mut self, key: &String, value: &T, ttl: Duration) -> Result<(), Self::Error> {
        let path = self.get_file_path(key);
        let mut tmp_path = path.clone();
        let mut new_file_name = tmp_path.file_name().unwrap().to_owned();
        new_file_name.push("-new");
        tmp_path.set_file_name(new_file_name);
        let mut file = try!(fs::OpenOptions::new().create(true).write(true).create(true).truncate(true).open(tmp_path));
        let entry = CacheEntry::new(try!(value.to_cache().map_err(Error::CacheSerializationFailure)), ttl);
        let _ = try!(file.write(&entry.to_string().into_bytes()));

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