use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use std::io::{Error as IoError, Read, Write};
use std::{fs::OpenOptions, path::Path};

/// A common trait to attach to metadata types to make
/// loading and unloading of their configurations easier
pub trait MetadataStore<T: Storable>: Storable {
    /// Returns a colletion that can then be used to load
    /// associated files from disk
    fn fetch(&self) -> Vec<T>;
}

pub trait Storable: Serialize + DeserializeOwned {
    /// Load the metadata structure associated with a type
    fn load(dir: &str, name: &str) -> Result<Self, IoError> {
        let p = Path::new(dir).join(name);
        let mut string = String::new();

        let mut f = OpenOptions::new().read(true).open(p)?;
        f.read_to_string(&mut string)?;
        return Ok(serde_json::from_str(&string)?);
    }

    /// Write this type to disk somewhere
    fn save(&self, dir: &str, name: &str) -> Result<(), IoError> {
        let p = Path::new(dir).join(name);
        let mut file = OpenOptions::new().write(true).open(p)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;
        return Ok(());
    }
}
