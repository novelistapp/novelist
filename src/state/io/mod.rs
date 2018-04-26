//! Serailisation specific structures

pub mod chapter;
pub mod document;
pub mod novel;
pub mod traits;
pub mod universe;

pub use self::chapter::ChapterData;
pub use self::document::DocumentData;
pub use self::novel::NovelData;
pub use self::universe::UniverseData;
pub use self::traits::Storable;


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
