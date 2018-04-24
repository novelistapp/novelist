use super::super::io::NovelData;
use super::{Chapter, Universe};

/// A novel project is a collection of chapters and other metadata
pub(crate) struct Novel {
    name: String,
    author: String,
    version: u8,
    chapters: Vec<Chapter>,
    on_disk: NovelData,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}

impl Novel {
    /// Create a new Novel on disk, at a path
    pub fn new(name: String, author: String, path: &str) -> Self {
        return Self {
            name: name.clone(),
            author: author.clone(),
            version: 0,
            chapters: Vec::new(),
            on_disk: NovelData::create(name.clone(), author, path).unwrap(),
            external_universe: None,
            internal_universe: Universe::new(name, "Default Universe for your story"),
        };
    }

    /// Load an existing novel from disk
    pub fn load(path: &str) -> Self {
        let nd = NovelData::load(path).unwrap();
        return Self {
            name: nd.name.clone(),
            author: nd.author.clone(),
            version: nd.version,
            chapters: Vec::new(),
            on_disk: nd.clone(),
            external_universe: None,
            internal_universe: Universe::new(nd.name, "Default Universe for your story"),
        };
    }
}
