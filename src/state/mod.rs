//! All the models used in Novelist

mod io;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A novel project is a collection of chapters and other metadata
pub(crate) struct Novel {
    name: String,
    author: String,
    version: u8,
    chapters: Vec<Chapter>,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}

/// A universe that has planning documents attached
pub(crate) struct Universe {
    name: String,
    description: String,

    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}

/// A chapter is a collection of scenes
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Chapter {
    name: String,
    description: String,
    scenes: Vec<Document>,
    word_count: usize
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


/// A document in a novel or universe
/// 
/// This can either be a scene, a note, a template or an
/// "implementation" of templates such as character sheets, etc
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Document {
    name: String,
    description: String,    
    word_count: usize,
    text: Vec<Paragraph>
}

/// A complete text Paragraph that ends with a "." (normally)
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Paragraph {
    snippets: Vec<Sentence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Sentence {
    snippets: Vec<TextSnippet>,
    terminator: char
}

/// A piece of text with a formatting style attached to it
/// 
/// ## Example
/// 
/// The simplest example would be
/// 
/// ```norun
/// TextSnippet::new("Just some Text", TextStyle::Plain);
/// ```
/// 
/// A more complicated example would immediately boil down to a 
/// list of textsnippets, to make up a sentence:
/// 
/// ```norun
/// let mut sentence = Vec::new();
/// sentence.push(TextSnippet::new("Some", TextStyle::Plain));
/// sentence.push(TextSnippet::new("amazing", TextStyle::Italics));
/// sentence.push(TextSnippet::new("text!!!", TextStyle::Bold));
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TextSnippet {
    text: String,
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
