use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use std::io::{Error as IoError, Read, Write};
use std::fs::{File, OpenOptions};

/// A common trait to attach to metadata types to make
/// loading and unloading of their configurations easier
pub trait MetadataStore<T: Storable>: Storable {
    /// Returns a colletion that can then be used to load
    /// associated files from disk
    /// 
    /// Takes a base path (as a directory)
    fn fetch(&self, path: &str) -> Vec<T>;
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
