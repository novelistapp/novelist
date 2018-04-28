use super::super::io;
use super::document::DocumentData;

use rayon::prelude::*;

use super::traits::*;
use std::fmt::Debug;
use std::io::Error as IoError;

/// Metadata file for a chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterData {
    pub name: String,
    pub description: String,
    pub scenes: Vec<String>,
}

impl ChapterData {
    /// Create a new novel metadata file on disk
    pub fn create(name: String, description: String, path: &str) -> Result<Self, IoError> {
        Ok(ChapterData {
            name: name.clone(),
            description,
            scenes: Vec::new(),
        }.create(&io::path_append(path, &[&format!("{}.chapter", &name)]))?)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/* Auto-implement store functions */
impl Storable for ChapterData {}

/// MetadataStore is used to pull child-metadata objects in
impl<T: Storable + Send> MetadataStore<T> for ChapterData {
    fn pull(&self, base: &str) -> Result<Vec<T>, IoError> {
        let paths = self.fetch(&base, "scene")?;
        Ok(paths
            .into_par_iter()
            .filter_map(|p| T::load(&p).ok())
            .collect())
    }
}

impl Indexable for ChapterData {
    fn index(&self) -> Vec<String> {
        self.scenes.clone()
    }
}
