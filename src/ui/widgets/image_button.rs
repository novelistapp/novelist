use crate::ui::{Icons, traits::Component};
use gtk::{self, *};

/// A wrapper for a button with an image
pub struct ImageButton {
    inner: ToolButton,
    image: Image,
}

impl ImageButton {
    pub fn new(icon_id: &str, btn_label: &str, size: u32) -> Self {
        let image = Icons::get(icon_id, size);
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
