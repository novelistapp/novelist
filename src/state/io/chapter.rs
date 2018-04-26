use super::traits::*;
use std::io::Error as IoError;
use std::path::Path;


/// Metadata file for a chapter
#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterData {
    name: String,
    description: String,
}

impl ChapterData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(ChapterData { name, description }.create(path)?);
    }
}

/* Auto-implement store functions */
impl Storable for ChapterData {}

/* Implement Document fetch manually */
impl<T: Storable> MetadataStore<T> for ChapterData {
    fn fetch(&self, path: &str) -> Vec<T> {
        
        
        return Vec::new();
    }
}