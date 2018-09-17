use crate::ui::Component;
use gtk::{self, GtkWindowExt, HeaderBar, HeaderBarExt, WidgetExt, Window, WindowType};

pub struct HeaderMenu {
    inner: HeaderBar,
}

impl HeaderMenu {
    pub fn new() -> Self {
        Self {
            inner: HeaderBar::new(),
        }
    }

    pub fn get_inner_ref(&self) -> &HeaderBar {
        &self.inner
    }
}

impl Component for HeaderMenu {
    fn init(&mut self) {
        self.inner.set_show_close_button(true);
        self.inner.set_title("Novelist");
    }
}
