//! File interactions with project folder structures
//!
//! This provides easy to use utility functions to create, load
//! and organise novelist projects.
//!
//! ### The folder structure is as follows
//!
//! - ProjectRoot
//!   - ProjecName.novel (metadata file)
//!   - Novel
//!     - Chapters (multiple folders)
//!       - Scene.bla (multiple files per folder)
//!   - Universe (Internal)
//!     - Internal.universe (metadata file)
//!     - Assets
//!     - ...
//!   - Universe (Linked)
//!     - ...

use serde_json;
use std::error::Error;
use std::io::Write;
use std::{fs::{self, File},
          path::Path};

use super::*;

impl Novel {
    /// Creates a new skeleton project at a specific location
    ///
    /// If the path is `/path/to/folder` and the provided name is
    /// `MyProject` then the resulting folder will be
    /// `/path/to/folder/MyProject`.
    ///
    /// See the module documentation for described folder structure
    ///
    pub fn create(path: &str, name: &str, author: &str) -> Result<Novel, Box<Error>> {
        let path = Path::new(path);

        /* Start with the root dir */
        fs::create_dir_all(path.join(name))?;
        fs::create_dir(path.join(name).join("Novel"))?;
        fs::create_dir(path.join(name).join("Universe_Internal"))?;

        let universe = Universe {
            name: String::from(name),
            description: String::new(),
            templates: Vec::new(),
            characters: Vec::new(),
            places: Vec::new(),
        };

        let novel = Novel {
            name: String::from(name),
            author: String::from(author),
            version: 0,
            chapters: Vec::new(),
            external_universe: None,
            internal_universe: LinkedUniverse::External(format!("{}", universe)),
        };

        let novel_cereal = serde_json::to_string(&novel)?;
        let mut novel_meta = File::create(path.join(name).join(&format!("{}.novel", name)))?;
        novel_meta.write_all(novel_cereal.as_bytes())?;

        let uni_cereal = serde_json::to_string(&universe)?;
        let mut novel_meta = File::create(
            path.join(name).join("Universe_Internal")
                .join(&format!("{}.universe", name)),
        )?;
        novel_meta.write_all(uni_cereal.as_bytes())?;

        /* Return an object that represents the new folders */
        return Ok(novel);
    }
}
