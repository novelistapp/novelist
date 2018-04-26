use super::traits::*;
use std::io::Error as IoError;

/// Represents the universe config on disk
///
/// This is a file with the `.universe` extention
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniverseData {
    pub name: String,
    pub description: String,
}

/* Auto-implement store functions */
impl Storable for UniverseData {}

impl UniverseData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        return Ok(UniverseData { name, description }.create(path)?);
    }
}
