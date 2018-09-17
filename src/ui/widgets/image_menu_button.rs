use crate::ui::Component;
use gtk::{Image, MenuToolButton, MenuToolButtonExt};

/// A wrapper for a menu button with an image
pub struct ImageMenuButton {
    inner: MenuToolButton,
    image: Image,
}

impl ImageMenuButton {
    pub fn new(icon_id: &str, btn_label: &str, size: u32) -> Self {
        let image = Image::new_from_icon_name(icon_id, size as i32);
        let inner = MenuToolButton::new(&image, btn_label);
        Self { inner, image }
    }
}

impl Component for ImageMenuButton {
    type WrappedType = MenuToolButton;

    fn init(&mut self) {}

    fn as_ref(&self) -> &Self::WrappedType {
        &self.inner
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        &mut self.inner
    }
}
