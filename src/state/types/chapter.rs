use super::document::Document;
use super::super::io::{Storable, MetadataStore};


/// A chapter is a collection of scenes
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize
}


impl Chapter {
    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }

    /// Get a reference list of chapters
    pub fn get_documents(&self) -> &Vec<Document> {
        return &self.scenes;
    }

    /// Get a mutable reference to a chapter to work with it
    pub fn get_document_mut(&mut self, name: String) -> Option<&mut Document> {
        return self.scenes
            .iter_mut()
            .filter(|i| i.is_named(&name))
            .next();
    }
}


/* Derive default store functions */
impl Storable for Chapter {}

/* Implement Document fetch manually */
impl<T: Storable> MetadataStore<T> for Chapter {
    fn fetch(&self) -> Vec<T> {
        return Vec::new();
    }
}