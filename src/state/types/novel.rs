use super::super::io::NovelData;
use super::{Chapter, Universe};

/// A novel project is a collection of chapters and other metadata
pub(crate) struct Novel {
    pub name: String,
    pub author: String,
    pub version: u8,
    chapters: Vec<Chapter>,
    on_disk: NovelData,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}

impl Novel {
    /// Try to create a new novel with a path.
    ///
    /// Will return None if a novel of the same name already
    /// existed in the directory.
    pub fn new(name: String, author: String, path: &str) -> Option<Self> {
        let on_disk = match NovelData::create(name.clone(), author.clone(), path) {
            Some(nd) => nd,
            None => return None,
        };

        let internal_universe = Universe::new(name.clone(), "Default Universe for your story");

        return Some(Self {
            name,
            author,
            on_disk,
            internal_universe,
            external_universe: None,
            chapters: Vec::new(),
            version: 0,
        });
    }

    /// Load an existing novel from disk
    pub fn load(path: &str) -> Option<Self> {
        let nd = match NovelData::load(path) {
            Some(nd) => nd,
            None => return None,
        };

        let on_disk = nd.clone();
        let (name, author, version) = (nd.name, nd.author, nd.version);
        let internal_universe = Universe::new(name.clone(), "Default Universe for your story");

        return Some(Self {
            name,
            author,
            version,
            on_disk,
            internal_universe,
            chapters: Vec::new(),
            external_universe: None,
        });
    }

    /// Get a reference list of chapters
    pub fn get_chapters(&self) -> &Vec<Chapter> {
        return &self.chapters;
    }

    /// Get a mutable reference to a chapter to work with it
    pub fn get_chapter_mut(&mut self, name: String) -> Option<&mut Chapter> {
        return self.chapters
            .iter_mut()
            .filter(|i| i.is_named(&name))
            .next();
    }
}
