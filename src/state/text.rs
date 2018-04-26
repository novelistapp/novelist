/// Common text types shared between the data and I/O layers

/// A complete text Paragraph that ends with a "." (normally)
#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
    snippets: Vec<Sentence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sentence {
    snippets: Vec<TextSnippet>,
    terminator: char,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextSnippet {
    text: String,
    style: TextStyle,
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
