use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use std::fs::{self, File, OpenOptions};
use std::io::{Error as IoError, Read, Write};

/// A simple trait a metadata object can implement
/// to gain the ability to be indexable
pub trait Indexable {
    fn index(&self) -> Vec<String>;

    /// Open a directory and get a list of all paths that might be
    /// relevant to a given metastore
    fn fetch(&self, path: &str, extention: &str) -> Result<Vec<String>, IoError> {
        Ok(fs::read_dir(path)?
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|f| match f.file_type() {
                Ok(vf) => vf.is_file(),
                _ => false,
            })
            .map(|de| de.path())
            .filter(|p| match p.extension() {
                Some(oss) => match oss.to_str() {
                    Some(s) => s.ends_with(extention),
                    _ => false,
                },
                _ => false,
            })
            .filter_map(|p| p.into_os_string().into_string().ok())
            .collect())
    }

}

/// A common trait to attach to metadata types to make
/// loading and unloading of their configurations easier
pub trait MetadataStore<T: Storable>: Indexable {

    /// Returns a colletion that can then be used to load
    /// associated files from disk
    ///
    /// Takes a base path (as a directory)
    fn pull(&self, base: &str) -> Result<Vec<T>, IoError>;
}

/// Describes a storable/ serialisable object.
///
/// This trait can easily be implemented by any object that
/// derives Serialize, Deserialize. As it has no un-implemented
/// functions there is no work to do for the end-user.
pub trait Storable: Serialize + DeserializeOwned {
    /// Load the metadata structure associated with a type
    fn load(path: &str) -> Result<Self, IoError> {
        let mut string = String::new();
        println!("Attempting to load: '{}'", path);
        let mut f = OpenOptions::new().read(true).open(path)?;
        f.read_to_string(&mut string)?;
        return Ok(serde_json::from_str(&string)?);
    }

    /// Create a new file and return an Error if that fails
    fn create(self, path: &str) -> Result<Self, IoError> {
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&self)?.as_bytes())?;
        return Ok(self);
    }

    /// Write this type to disk somewhere
    fn save(&self, path: &str) -> Result<(), IoError> {
        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;
        return Ok(());
    }
}

/// A utility trait to read the conents from a file in
/// a single line.
pub trait FileToString {
    /// Read the file contents into a string without any
    /// error handling.
    fn get_string(&mut self) -> Result<String, IoError>;
}

impl FileToString for File {
    fn get_string(&mut self) -> Result<String, IoError> {
        let mut s = String::new();
        return match self.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        };
    }
}
