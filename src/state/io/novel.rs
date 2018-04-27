
use super::traits::*;
use std::io::Error as IoError;
use std::{fs, path::Path};

/// Represents the novel config on disk
///
/// This is a file with the `.novel` extention
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NovelData {
    pub name: String,
    pub author: String,
    pub version: u8,
    pub external_universe: Option<String>,
    pub chapters: Vec<String>,
}

/// Auto-implements Storable
impl Storable for NovelData {}

impl NovelData {

    /// Create a new novel metadata file
    pub fn create(name: String, author: String, dir: &str) -> Result<Self, IoError> {
        let p = Path::new(dir).join(&format!("{}.novel", &name));
        return Ok(NovelData {
            name,
            author,
            version: 0,
            external_universe: None,
            chapters: Vec::new(),
        }.create(p.to_str().unwrap())?);
    }
}
