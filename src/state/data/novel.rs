use super::super::io::{self, FileContainer, NovelData, Storable};
use super::{Chapter, Universe};

use std::io::Error as IoError;
use utils::InteratorResultExt;

/// A novel project is a collection of chapters and other metadata
#[derive(Debug)]
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
    /// Create a new novel in memory and on disk
    ///
    /// The `directory` parameter refers to the **containing** directory,
    /// not the novel directory itself.
    ///
    /// This is persistent across the novel-file API, unless otherwise stated.
    ///
    /// **All joining of `<directory>/<name>` is done internally!**
    ///
    /// # Example
    ///
    /// ```norun
    /// Novel::new("Game of Thrones", "George RR Martin", "/home/george/books/");
    /// ```
    pub fn new(name: String, author: String, directory: &str) -> Result<Self, IoError> {
        /* Scaffold the directory structure */
        io::create_scaffold(directory, &name)?;

        /* Shift the base directory to make working more easily */
        let base = io::path_append(directory, &[&name]);

        /* Create a new empty Universe */
        let internal_universe = Universe::new(
            name.clone(),
            String::from("Default Universe for your story"),
            &io::path_append(&base, &["Universe"]),
        )?;

        /* Store novel metadat file in a wrapper */
        let container = FileContainer::new(
            &base,
            NovelData::create(name.clone(), author.clone(), &base)?,
        );

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

    /// Load an existing novel from disk.
    ///
    /// The `path` parameter in this case is the absolute path to
    /// the `.novel` index file.
    ///
    /// # Example
    ///
    /// ```norun
    /// Novel::load("/home/george/books/Where Everybody Dies/Where Everybody Dies.novel");
    /// ``'
    pub fn load(path: &str) -> Result<Self, IoError> {
        let on_disk = NovelData::load(path)?;
        let base = io::path_pop(path, 1);

        let NovelData {
            name,
            author,
            version,
            ..
        } = on_disk.clone();

        let internal_universe = Universe::new(
            name.clone(),
            String::from("Default Universe for your story"),
            &io::path_append(&base, &["Universe"]),
        )?;
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

    pub fn add_chapter(&mut self, name: &str, descr: &str) {
        self.chapters.push(
            Chapter::create(
                name.to_owned(),
                descr.to_owned(),
                &io::path_append(&self.container.path, &["Novel", "Chapters"]),
            ).unwrap(),
        );
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

    /// Save this novel and all modified files to disk
    pub fn save(&mut self) -> Result<(), Vec<IoError>> {
        if let Err(es) = self.chapters
            .iter_mut()
            .filter(|x| x.is_dirty())
            .map(|x| x.save())
            .fold_errs()
        {
            return Err(es.into_iter().flatten().collect());
        } else {
            Ok(())
        }
    }
}
