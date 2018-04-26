use super::document::Document;

/// A universe that has planning documents attached
pub struct Universe {
    name: String,
    description: String,

    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}

impl Universe {
    pub fn new<S: Into<String>, T: Into<String>>(name: S, decription: T) -> Self {
        return Self {
            name: name.into(),
            description: decription.into(),
            templates: Vec::new(),
            characters: Vec::new(),
            places: Vec::new()
        };
    }
}