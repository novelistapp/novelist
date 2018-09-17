//! Root window UI context

use crate::ui::Component;
use gtk::{self, GtkWindowExt, HeaderBar, HeaderBarExt, Widget, WidgetExt, Window, WindowType};

use ui::widgets::headermenu::HeaderMenu;

pub struct RootWindow {
    inner: Window,
    header_menu: HeaderMenu,
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
            header_menu: HeaderMenu::new(),
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

        self.header_menu.init();
        self.inner.set_titlebar(self.header_menu.get_inner_ref());
        self.inner.set_default_size(800, 600);
        self.inner.show_all();
    }
}
