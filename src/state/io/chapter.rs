use super::traits::*;
use std::io::Error as IoError;


/// Metadata file for a chapter
#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterData {
    name: String,
    description: String,
}

/* Auto-implement store functions */
impl Storable for ChapterData {}

impl ChapterData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(ChapterData { name, description }.create(path)?);
    }
}
