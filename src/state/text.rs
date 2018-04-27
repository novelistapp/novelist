/// Common text types shared between the data and I/O layers

/// A complete text Paragraph that ends with a "." (normally)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    snippets: Vec<Sentence>,
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Paragraph { snippets: Vec::new() }
    }

    pub fn wordcount(&self) -> usize {
        self.snippets.iter().map(|x| x.wordcount()).sum()
    }

    pub fn snippets(&mut self) -> &mut Vec<Sentence> {
        &mut self.snippets
    }

    /// Utility to append a new sentence to this paragraph
    pub fn append(&mut self) -> &mut Sentence {
        self.snippets.push(Sentence {
            snippets: Vec::new(),
            terminator: ' ',
        });
        self.snippets.last_mut().unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
    snippets: Vec<TextSnippet>,
    terminator: char,
}

impl Sentence {
    pub fn wordcount(&self) -> usize {
        self.snippets.iter().map(|x| x.wordcount()).sum()
    }

    pub fn push_text(&mut self, text: &str, style: TextStyle) {
        self.snippets.push(TextSnippet { text: text.to_string(), style });
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSnippet {
    text: String,
    style: TextStyle,
}

impl TextSnippet {
    pub fn new(text: String, style: TextStyle) -> Self {
        Self { text, style }
    }

    pub fn wordcount(&self) -> usize {
        self.text.len()
    }
}

/// Describes a style of text
#[derive(Debug, Clone, Serialize, Deserialize)]
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
