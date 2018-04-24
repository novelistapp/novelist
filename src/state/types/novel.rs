use super::{universe::Universe, chapter::Chapter};

/// A novel project is a collection of chapters and other metadata
pub(crate) struct Novel {
    name: String,
    author: String,
    version: u8,
    chapters: Vec<Chapter>,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}
