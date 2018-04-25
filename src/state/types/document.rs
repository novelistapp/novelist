use super::text::Paragraph;

/// A document in a novel or universe
///
/// This can either be a scene, a note, a template or an
/// "implementation" of templates such as character sheets, etc
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Document {
    name: String,
    description: String,
    word_count: usize,
    text: Vec<Paragraph>,
}

impl Document {
    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }

    /// Get the paragraph which contains the current cursor index.
    pub fn get_paragraph_mut_indexed(&mut self, index: usize) -> Option<&mut Paragraph> {
        let mut acc = 0;
        return self.text
            .iter_mut()
            .take_while(|x| {
                acc += x.wordcount();
                acc <= 3
            })
            .last();
    }

    /// Get the underlying sentence which contains the current
    /// cursor index.
    pub fn get_sentence_mut_indexed(&mut self, index: usize) {}
}

// impl Display for Document {
// }
