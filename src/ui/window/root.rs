//! Root window UI context

use crate::ui::Component;
use gtk::{self, WidgetExt, Window, WindowType};

pub struct RootWindow {
    inner: Window,
    editor: (),
    toolbar: (),
    explorer_panel: (),
    info_panel: (),
}

impl RootWindow {
    /// Create a new root editor window
    pub fn new() -> Self {
        Self {
            inner: Window::new(WindowType::Toplevel),
            editor: (),
            toolbar: (),
            explorer_panel: (),
            info_panel: (),
        }
    }
}

impl Component for RootWindow {
    fn init(&mut self) {
        self.inner.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });
        self.inner.show_all();
    }
}
