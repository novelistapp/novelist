use gtk::Image;
use std::collections::HashMap;

/// Icon utility wrapper struct
///
/// This acts as a general icon store that are loaded from
/// resource directories at runtime to provide easier icon
/// path handling than doing it all manually in the
/// components.
///
/// Each icon has an idea of the form `n-<component>-<action>`
/// which should be followed as closely as possible.
pub struct Icons;

impl Icons {
    /// A dump wrapper around Image::new_from_icon_name
    pub fn get(id: &str, size: u32) -> Image {
        Image::new_from_icon_name(id, size as i32)
    }
}
