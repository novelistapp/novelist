use super::chapter::ChapterData;
use super::path_append;
use super::traits::*;

use rayon::prelude::*;

use std::fmt::Debug;
use std::io::{Error as IoError, Read};
use std::{
    fs::{self, File},
    path::Path,
};

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

/// MetadataStore is used to pull chilf-metadata objects in
impl<T: Storable + Send> MetadataStore<T> for NovelData {
    fn pull(&self, base: &str) -> Result<Vec<T>, IoError> {
        let paths = self.fetch(&path_append(&base, &["Novel", "Chapters"]), "chapter")?;
        Ok(paths
            .into_par_iter()
            .filter_map(|p| T::load(&p).ok())
            .collect())
    }
}

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

impl Indexable for NovelData {
    fn index(&self) -> Vec<String> {
        self.chapters.clone()
    }
}
