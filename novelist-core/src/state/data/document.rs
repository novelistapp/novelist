use crate::state::io::{self, traits::Storable, DocumentData, FileContainer};
use crate::state::text::{Paragraph, Sentence, TextStyle};
use std::io::Error as IoError;

use rayon::prelude::*;

/// A document in a novel or universe
///
/// This can either be a scene, a note, a template or an
/// "implementation" of templates such as character sheets, etc
#[derive(Debug, Clone)]
pub struct Document {
    name: String,
    description: String,
    word_count: usize,
    text: Vec<Paragraph>,
    on_disk: FileContainer<DocumentData>,
    dirty: bool,
}

impl Document {
    /// Create a new chapter metadata file and folder
    pub fn create(name: String, description: String, dir: &str) -> Result<Self, IoError> {
        let on_disk = FileContainer::new(
            dir,
            DocumentData::create(name.clone(), description.clone(), dir)?,
        );

        Ok(Self {
            name,
            description,
            word_count: 0,
            text: Vec::new(),
            on_disk,
            dirty: false,
        })
    }

    pub fn load(data: DocumentData, base: &str) -> Self {
        let c = data.clone();
        Self {
            name: c.name,
            description: c.description,
            word_count: 0,
            text: c.text,
            on_disk: FileContainer::new(base, data),
            dirty: false,
        }
    }

    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        *&self.name == *name
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// whether this file has changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the paragraph which contains the current cursor index.
    pub fn get_paragraph(&mut self, index: usize) -> Option<&mut Paragraph> {
        let mut acc = 0;
        self
            .text
            .iter_mut()
            .take_while(|x| {
                acc += x.wordcount();
                acc < index
            }).last()
    }

    /// Get the underlying sentence which contains the current
    /// cursor index.
    pub fn get_sentence(&mut self, index: usize) -> Option<&mut Sentence> {
        let mut acc = 0;
        self
            .text
            .iter_mut()
            .map(|x| x.snippets())
            .flat_map(|x| x.iter_mut())
            .take_while(|x| {
                acc += x.wordcount();
                acc < index
            }).last()
    }

    /// Append a string into the latest paragraph
    ///
    /// FOR TESTING ONLY PLEASE
    pub fn append(&mut self, word: &str) {
        self.text.push(Paragraph::new());
        self.text
            .last_mut()
            .unwrap()
            .append()
            .push_text(word, TextStyle::Plain);
        self.dirty = true;
    }

    /// Utility function which makes all sections re-count themselves
    pub fn refresh_word_count(&mut self) -> usize {
        self.word_count = self.text.par_iter_mut().map(|p| p.wordcount()).sum();
        self.word_count
    }

    /// Write down a document to disk
    pub fn save(&mut self) -> Result<(), IoError> {
        let cpy: DocumentData = self.clone().into();
        cpy.save(&io::path_append(
            &self.on_disk.path,
            &[&format!("{}.scene", &self.name)],
        ))?;
        Ok(())
    }
}

impl Into<DocumentData> for Document {
    fn into(self) -> DocumentData {
        DocumentData {
            name: self.name,
            description: self.description,
            text: self.text,
        }
    }
}
