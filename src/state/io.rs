//! Serailisation specific structures

use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use std::io::{Error as IoError, Read, Write};
use std::{fs::{File, OpenOptions},
          path::Path};

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

        let mut f = File::open(p)?;
        f.read_to_string(&mut string)?;
        return Ok(serde_json::from_str(&string)?);
    }

    /// Write this type to disk somewhere
    fn save(&self, dir: &str, name: &str) -> Result<(), IoError> {
        let p = Path::new(dir).join(name);
        let mut file = OpenOptions::new().write(true).open(p)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        return Ok(());
    }

}

/// Represents the novel config on disk
///
/// This is a file with the `.novel` extention
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NovelData {
    pub name: String,
    pub author: String,
    pub version: u8,
    pub external_universe: Option<String>,
}

/// Represents the universe config on disk
///
/// This is a file with the `.universe` extention
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniverseData {
    pub name: String,
    pub description: String,
}

impl NovelData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, author: String, path: &str) -> Option<NovelData> {
        let mut f = match File::create(path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let me = NovelData {
            name,
            author,
            version: 0,
            external_universe: None,
        };

        f.write_all(serde_json::to_string(&me).unwrap().as_bytes())
            .unwrap();
        return Some(me);
    }

    /// Load an existing metadata file from disk
    pub fn load(path: &str) -> Option<NovelData> {
        return match File::open(path) {
            Ok(ref mut f) => match f.get_string() {
                Ok(c) => serde_json::from_str(&c).unwrap(),
                Err(_) => None,
            },
            _ => None,
        };
    }
}

impl UniverseData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Option<UniverseData> {
        let mut f = match File::create(path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let me = UniverseData { name, description };

        f.write_all(serde_json::to_string(&me).unwrap().as_bytes())
            .unwrap();
        return Some(me);
    }

    /// Load an existing metadata file from disk
    pub fn load(path: &str) -> Option<UniverseData> {
        return match File::open(path) {
            Ok(ref mut f) => match f.get_string() {
                Ok(c) => serde_json::from_str(&c).unwrap(),
                Err(_) => None,
            },
            _ => None,
        };
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
