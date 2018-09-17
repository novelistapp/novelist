//! Root window UI context

use gtk::{Window, WindowType};

struct RootWindow {
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
