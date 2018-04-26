use super::super::io::{traits::Storable, DocumentData, FileContainer};
use state::text::{Paragraph, Sentence};

/// A document in a novel or universe
///
/// This can either be a scene, a note, a template or an
/// "implementation" of templates such as character sheets, etc
#[derive(Debug, Clone)]
pub(crate) struct Document {
    name: String,
    description: String,
    word_count: usize,
    text: Vec<Paragraph>,
    on_disk: FileContainer<DocumentData>,
    dirty: bool,
}

impl Document {
    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }

    /// Return whether this file has changes
    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    /// Get the paragraph which contains the current cursor index.
    pub fn get_paragraph(&mut self, index: usize) -> Option<&mut Paragraph> {
        let mut acc = 0;
        return self.text
            .iter_mut()
            .take_while(|x| {
                acc += x.wordcount();
                acc < index
            })
            .last();
    }

    /// Get the underlying sentence which contains the current
    /// cursor index.
    pub fn get_sentence(&mut self, index: usize) -> Option<&mut Sentence> {
        let mut acc = 0;
        return self.text
            .iter_mut()
            .map(|x| x.snippets())
            .flat_map(|x| x.iter_mut())
            .take_while(|x| {
                acc += x.wordcount();
                acc < index
            })
            .last();
    }

    /// Write down a document to disk
    pub fn save(&mut self) {
        let cpy: DocumentData = self.clone().into();
        match cpy.save(&self.on_disk.path) {
            Err(_) => eprintln!("Failed to save file!"),
            _ => {}
        }
    }
}

impl Into<DocumentData> for Document {
    fn into(self) -> DocumentData {
        return DocumentData {
            name: self.name,
            description: self.description,
            text: self.text,
        };
    }
}
