use super::document::Document;


/// A universe that has planning documents attached
pub(crate) struct Universe {
    name: String,
    description: String,

    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}
