use crate::ui::Component;
use gtk::{Image, ToolButton, ToolButtonExt};

/// A wrapper for a button with an image
pub struct ImageButton {
    inner: ToolButton,
    image: Image,
}

impl ImageButton {
    pub fn new(icon_id: &str, btn_label: &str, size: u32) -> Self {
        let image = Image::new_from_icon_name(icon_id, size as i32);
        let inner = ToolButton::new(&image, btn_label);
        Self { inner, image }
    }
}

impl Component for ImageButton {
    type WrappedType = ToolButton;

    fn init(&mut self) {}

    fn as_ref(&self) -> &Self::WrappedType {
        &self.inner
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        &mut self.inner
    }
}
