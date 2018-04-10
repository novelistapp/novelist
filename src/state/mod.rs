//! All the models used in Novelist

mod io;

/// A novel project is a collection of chapters and other metadata
#[derive(Debug, Serialize, Deserialize)]
struct Novel {
    name: String,
    author: String,
    version: u8,
    chapters: Vec<Chapter>,

    external_universe: Option<Universe>,
    internal_universe: Universe,
}

/// A chapter is a collection of scenes
#[derive(Debug, Serialize, Deserialize)]
struct Chapter {
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
struct Document {
    name: String,
    description: String,    
    word_count: usize,
    text: Vec<Sentence>
}

/// A complete text sentence that ends with a "." (normally)
#[derive(Debug, Serialize, Deserialize)]
struct Sentence {
    snippets: Vec<TextSnippet>,
    word_count: usize,
    terminator: char
}

/// A piece of text with a formatting style attached to it
#[derive(Debug, Serialize, Deserialize)]
struct TextSnippet {
    text: String,
    word_count: usize,
    style: TextStyle,
}

/// Describes a style of text
#[derive(Debug, Serialize, Deserialize)]
enum TextStyle {
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
enum AssetType {

    /// Contains the extention of the image
    Image(String),
}

/// A linked asset copied into a project
#[derive(Debug, Serialize, Deserialize)]
struct Asset {
    _type: AssetType,
    name: String,
    blob: Vec<u8>
}

/// A universe project
#[derive(Debug, Serialize, Deserialize)]
struct Universe {
    name: String,
    description: String,

    templates: Vec<Document>,
    characters: Vec<Document>,
    places: Vec<Document>,
}