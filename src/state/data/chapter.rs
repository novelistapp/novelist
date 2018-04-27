use super::super::io::traits::MetadataStore;
use super::super::io::{self, chapter::ChapterData, DocumentData, FileContainer};
use super::document::Document;
use utils::InteratorResultExt;

use std::io::Error as IoError;

/// A chapter is a collection of scenes
#[derive(Debug)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize,
    on_disk: FileContainer<ChapterData>,
    dirty: bool,
}

impl Chapter {
    /// Create a new chapter metadata file and folder
    pub fn create(name: String, description: String, dir: &str) -> Result<Chapter, IoError> {
        let on_disk = FileContainer::new(
            dir,
            ChapterData::create(name.clone(), description.clone(), dir)?,
        );
        let path = io::path_append(dir, &[&format!("{}", name)]);
        io::create_dir(path)?;

        return Ok(Chapter {
            name,
            description,
            scenes: Vec::new(),
            word_count: 0,
            on_disk,
            dirty: false,
        });
    }

    /// Load a chapter from an existing ChapterData object
    pub fn load(data: ChapterData, base: &str) -> Result<Self, IoError> {
        let ChapterData {
            name, description, ..
        } = data.clone();

        /* Pull in all documents */
        let docs: Vec<Document> = data.pull(&base)?
            .into_iter()
            .map(|dd| Document::load(dd, base))
            .collect();

        Ok(Self {
            name,
            description,
            scenes: docs,
            word_count: 0,
            on_disk: FileContainer::new("", data),
            dirty: false,
        })
    }

    pub fn add_scene(&mut self, name: String, descr: String) -> &mut Document{
        self.scenes.push(
            Document::create(
                name,
                descr,
                &io::path_append(&self.on_disk.path, &[&self.name]),
            ).unwrap(),
        );
        self.dirty = true;
        self.scenes.last_mut().unwrap()
    }

    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }

    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

    /// Get a reference list of chapters
    pub fn get_documents(&self) -> &Vec<Document> {
        return &self.scenes;
    }

    /// Get a mutable reference to a chapter to work with it
    pub fn get_document_mut(&mut self, name: String) -> Option<&mut Document> {
        return self.scenes.iter_mut().filter(|i| i.is_named(&name)).next();
    }

    pub fn save(&mut self) -> Result<(), Vec<IoError>> {
        self.scenes
            .iter_mut()
            .filter(|x| x.is_dirty())
            .map(|x| x.save())
            .fold_errs()
    }
}

// I hear you Pascal...I hear you :/
// #[test]
// fn foo() {
//     let c = Chapter {
//         name: "fo".into(),
//         description: "bar".into(),
//         scenes: vec![Document {
//             name: "foo".into(),
//             description: "foo".into(),
//             word_count: 42,
//             text: vec![],
//             on_disk: FileContainer::new("llll", DocumentData {
//                 name: "foo".into(),
//                 description: "foo".into(),
//                 text: vec![],
//             }),
//             dirty: false,
//         }],
//         word_count: 42,
//         on_disk: FileContainer::new("llll", ChapterData {
//                 name: "foo".into(),
//                 description: "foo".into(),
//                 scenes: vec![],
//             }),
//         dirty: true,
//     };

//     c.save().unwrap();
// }
