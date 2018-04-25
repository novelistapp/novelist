//! Text type handling module

/// A complete text Paragraph that ends with a "." (normally)
#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
    snippets: Vec<Sentence>,
}

impl Paragraph {
    pub fn wordcount(&self) -> u64 {
        5
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sentence {
    snippets: Vec<TextSnippet>,
    terminator: char,
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
pub struct TextSnippet {
    text: String,
    style: TextStyle,
}

impl TextSnippet {
    pub fn new(text: String, style: TextStyle) -> Self {
        return Self { text, style };
    }
}

/// Describes a style of text
#[derive(Debug, Serialize, Deserialize)]
pub enum TextStyle {
    /// Normally render this text
    Plain,
    /// Make it *italics*
    Italics,
    /// Make it **bold**
    Bold,
    /// Make it _underline_
    Underline,
}
