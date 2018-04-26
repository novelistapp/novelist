//! Serailisation specific structures

pub mod traits;

use serde_json;
use std::fs::File;
use std::io::{Error as IoError, Read, Write};

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
