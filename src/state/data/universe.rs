use super::super::io::{self, FileContainer, UniverseData};
use super::document::Document;
use std::io::Error as IoError;

/// A universe that has planning documents attached
#[derive(Debug)]
pub struct Universe {
    name: String,
    description: String,
    on_disk: FileContainer<UniverseData>,
    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}

impl Universe {
    pub fn new(name: String, description: String, dir: &str) -> Result<Self, IoError> {
        let path = &io::path_append(&dir, &[""]);
        let on_disk = UniverseData::create(name.clone(), description.clone(), path)?;

        return Ok(Self {
            name: name.into(),
            description: description.into(),
            on_disk: FileContainer::new(&path, on_disk),
            templates: Vec::new(),
            characters: Vec::new(),
            places: Vec::new(),
        });
    }
}
