use super::super::io::{FileContainer, NovelData, Storable};
use super::{Chapter, Universe};
use std::io::Error as IoError;

/// A novel project is a collection of chapters and other metadata
pub(crate) struct Novel {
    pub name: String,
    pub author: String,
    pub version: u8,
    chapters: Vec<Chapter>,

    container: FileContainer<NovelData>,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}

impl Novel {
    /// Try to create a new novel with a path.
    ///
    /// Will return None if a novel of the same name already
    /// existed in the directory.
    pub fn new(name: String, author: String, path: &str) -> Result<Self, IoError> {
        let on_disk = NovelData::create(name.clone(), author.clone(), path)?;
        let internal_universe = Universe::new(name.clone(), "Default Universe for your story");
        let container = FileContainer::new(path, on_disk);

        return Ok(Self {
            name,
            author,
            container,
            internal_universe,
            external_universe: None,
            chapters: Vec::new(),
            version: 0,
        });
    }

    /// Load an existing novel from disk
    pub fn load(path: &str) -> Result<Self, IoError> {
        let on_disk = NovelData::load(path)?;
        let NovelData {
            name,
            author,
            version,
            ..
        } = on_disk.clone();

        let internal_universe = Universe::new(name.clone(), "Default Universe for your story");
        let container = FileContainer::new(path, on_disk);

        return Ok(Self {
            name,
            author,
            version,
            container,
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
