use super::document::Document;


/// A chapter is a collection of scenes
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize
}
