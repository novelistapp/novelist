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

    pub fn get_mut_bar(&mut self) -> &mut HeaderBar {
        &mut self.inner
    }
}

impl Component for HeaderMenu {
    fn init(&mut self) {}
}
