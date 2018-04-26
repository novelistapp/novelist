use super::document::Document;
use super::super::io::chapter::ChapterData;

/// A chapter is a collection of scenes
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize,
    on_disk: ChapterData,
    path: String,
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
