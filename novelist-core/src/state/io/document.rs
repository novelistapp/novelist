use super::super::text::*;

use super::super::io;
use super::traits::Storable;

use std::io::Error as IoError;

/// Describes a document on disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    pub name: String,
    pub description: String,
    pub text: Vec<Paragraph>,
}

/* Auto-implement store functions */
impl Storable for DocumentData {}

impl DocumentData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(DocumentData {
            name: name.clone(),
            description,
            text: Vec::new(),
        }.create(&io::path_append(path, &[&format!("{}.scene", &name)]))?);
    }
}
