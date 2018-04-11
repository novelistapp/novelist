//! All the models used in Novelist

pub(crate) mod io;

use std::fmt::{Display, Formatter, Result as FmtResult};

/// A novel project is a collection of chapters and other metadata
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Novel {
    name: String,
    author: String,
    version: u8,
    chapters: Vec<Chapter>,

    external_universe: Option<LinkedUniverse>,
    internal_universe: LinkedUniverse,
}

/// This is a bit of a hack.
/// 
/// Basically what it allows us to do is have a proper Novel -> Universe
/// hierarchy in the structs while only saving references to the universe
/// as a string. This is pretty shitty
/// 
/// Instead what we want to do is implement our own serialisers that
/// take into account to only include a String of the name or path
/// of the universe.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum LinkedUniverse {
    Internal(Universe),
    External(String),
}

/// A chapter is a collection of scenes
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize
}

/// A document in a novel or universe
/// 
/// This can either be a scene, a note, a template or an
/// "implementation" of templates such as character sheets, etc
/// 
/// TODO: Add a schema type that allows to selectively allow
/// typing (see external links)
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Document {
    name: String,
    description: String,    
    word_count: usize,
    text: Vec<Sentence>
}

/// A complete text sentence that ends with a "." (normally)
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Sentence {
    snippets: Vec<TextSnippet>,
    word_count: usize,
    terminator: char
}

/// A piece of text with a formatting style attached to it
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TextSnippet {
    text: String,
    word_count: usize,
    style: TextStyle,
}

/// Describes a style of text
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum TextStyle {
    /// Normally render this text
    Plain,
    /// Make it *italics*
    Italics,
    /// Make it **bold**
    Bold,
    /// Make it _underline_
    Underline
}

/// The type of asset that is linked into a project
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AssetType {

    /// Contains the extention of the image
    Image(String),
}

/// A linked asset copied into a project
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Asset {
    _type: AssetType,
    name: String,
    blob: Vec<u8>
}

/// A universe project
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Universe {
    name: String,
    description: String,

    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name)
    }
}
