use super::traits::*;
use std::io::Error as IoError;
use std::path::Path;

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
    pub fn create(name: String, description: String, dir: &str) -> Result<Self, IoError> {
        let path = Path::new(dir).join(&format!("{}.universe", &name));
        return Ok(UniverseData { name, description }.create(path.to_str().unwrap())?);
    }
}
