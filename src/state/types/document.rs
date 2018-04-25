use super::text::Paragraph;
use std::fmt::*;

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

impl Document {
    /// Utility function to check if this chapter has a certain name
    pub fn is_named(&self, name: &String) -> bool {
        return *&self.name == *name;
    }
}


// impl Display for Document {
// }