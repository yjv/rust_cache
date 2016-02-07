extern crate crypto;
use std::fs;
use std::path;
use std::os::unix::fs::DirBuilderExt;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;

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