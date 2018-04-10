//! I/O module that handles serialising and deserialising projects
//! 
//! 


use serde_json;
use super::Novel;

impl Novel {
    
    /// Load an existing novel project from disk
    pub fn load(path: &str) -> Novel {
        serde_json::from_str(path).unwrap()
    }
}
