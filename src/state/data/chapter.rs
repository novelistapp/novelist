use super::super::io::{self, chapter::ChapterData, FileContainer};
use super::document::Document;
use std::io::Error as IoError;

/// A chapter is a collection of scenes
#[derive(Debug)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize,
    on_disk: FileContainer<ChapterData>,
    dirty: bool,
}

impl Chapter {
    /// Create a new chapter metadata file and folder
    pub fn create(name: String, description: String, dir: &str) -> Result<Chapter, IoError> {
        let on_disk = FileContainer::new(
            dir,
            ChapterData::create(name.clone(), description.clone(), dir)?,
        );
        let path = io::path_append(dir, &[&format!("{}", name)]);
        io::create_dir(path)?;

        return Ok(Chapter {
            name,
            description,
            scenes: Vec::new(),
            word_count: 0,
            on_disk,
            dirty: false,
        });
    }

    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }

    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    /// Get a reference list of chapters
    pub fn get_documents(&self) -> &Vec<Document> {
        return &self.scenes;
    }

    /// Get a mutable reference to a chapter to work with it
    pub fn get_document_mut(&mut self, name: String) -> Option<&mut Document> {
        return self.scenes.iter_mut().filter(|i| i.is_named(&name)).next();
    }

    pub fn save(&mut self) -> Result<(), IoError> {
        return self.scenes
            .iter_mut()
            .filter(|x| x.is_dirty())
            .for_each(|x| x.save()?)
            .collect();
    }
}
