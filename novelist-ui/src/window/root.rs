//! Root window UI context

use crate::traits::Component;
use crate::widgets::HeaderMenu;

use gtk::{self, *};


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
    type WrappedType = Window;

    fn init(&mut self) {
        self.inner.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        self.header_menu.init();
        self.inner.set_titlebar(self.header_menu.as_ref());
        self.inner.set_default_size(800, 600);
        self.inner.show_all();
    }

    fn as_ref(&self) -> &Self::WrappedType {
        &self.inner
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        &mut self.inner
    }
}
