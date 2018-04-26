//! Serailisation specific structures

pub mod chapter;
pub mod document;
pub mod novel;
pub mod traits;
pub mod universe;

pub use self::chapter::ChapterData;
pub use self::document::DocumentData;
pub use self::novel::NovelData;
pub use self::traits::Storable;
pub use self::universe::UniverseData;

use std::io::{Error as IoError, ErrorKind};
use std::{fs,
          path::{Path, PathBuf}};

/// Type to wrap a generic `on_disk` type with a path
pub struct FileContainer<T: Storable> {
    pub path: String,
    pub on_disk: T,
}

impl<T: Storable> FileContainer<T> {
    /// A utility function for lazy people
    pub fn new(path: &str, on_disk: T) -> Self {
        return FileContainer {
            path: String::from(path),
            on_disk,
        };
    }
}

/// Create a novel with a name at a location
///
/// The `dir` parameter refers to the **containing** directory,
/// not the novel directory itself. The structure is then as follows
///
/// - `dir` parameter
///   - `name` folder
///     - `{name}.novel` metadata file
///     - Novel
///       - Chapters
///     - Universe
///       - `{name}.universe`
///       - assets/
///     - Universe (Linked) – Left empty initially
///
/// ## Future improvements
///
/// - This function should be able to selectively "repair" novels
/// - Have a seperate function to easily initialise Universes
/// -
pub fn create_scaffold(dir: &str, name: &str) -> Result<(), IoError> {
    let path = Path::new(dir).join(name);
    if path.exists() {
        return Err(IoError::from(ErrorKind::AlreadyExists));
    }

    fs::create_dir_all(&path)?;
    fs::create_dir_all(&path.join("Novel").join("Chapters"))?;
    fs::create_dir_all(&path.join("Universe").join("Assets"))?;
    fs::create_dir_all(&path.join("Universe").join("Templates"))?;

    Ok(())
}

/// A convenience function to append path segments
pub fn path_append(base: &str, join: &[&str]) -> String {
    let mut path = PathBuf::new();
    path.push(base);
    join.iter().for_each(|x| path.push(x));
    return String::from(path.to_str().unwrap());
}

/// A convenience function to pop items from a path segment
pub fn path_pop(base: &str, levels: u64) -> String {
    let mut path = PathBuf::new();
    path.push(base);

    [0..levels].iter().for_each(|_| {
        path.pop();
    });

    return String::from(path.to_str().unwrap());
}
