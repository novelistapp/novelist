use crate::{traits::Component, Icons};
use crate::core::switchboard::Switchboard;
use gtk::{self, *};

/// A wrapper for a menu button with an image
pub struct ImageMenuButton {
    inner: MenuToolButton,
    image: Image,
}

impl ImageMenuButton {
    pub fn new(icon_id: &str, btn_label: &str, size: u32) -> Self {
        let image = Icons::get(icon_id, size);
        let inner = MenuToolButton::new(&image, btn_label);
        Self { inner, image }
    }
}

impl Component for ImageMenuButton {
    type WrappedType = MenuToolButton;

    fn init(&mut self, _: &Switchboard) {}

    fn as_ref(&self) -> &Self::WrappedType {
        &self.inner
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        &mut self.inner
    }
}
