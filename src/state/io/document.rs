use super::super::text::*;
use super::traits::Storable;
use std::io::Error as IoError;

/// Describes a document on disk
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentData {
    name: String,
    description: String,
    text: Vec<Paragraph>,
}

/* Auto-implement store functions */
impl Storable for DocumentData {}

impl DocumentData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(DocumentData {
            name,
            description,
            text: Vec::new(),
        }.create(path)?);
    }
}
