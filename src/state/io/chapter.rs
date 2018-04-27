use super::super::io;
use super::traits::*;
use std::io::Error as IoError;

/// Metadata file for a chapter
#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterData {
    name: String,
    description: String,
    scenes: Vec<String>,
}

impl ChapterData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(ChapterData {
            name: name.clone(),
            description,
            scenes: Vec::new()
        }.create(&io::path_append(path, &[&format!("{}.chapter", &name)]))?);
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
