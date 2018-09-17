use crate::ui::Component;
use gtk::{self, HeaderBar, HeaderBarExt, MenuToolButton, MenuToolButtonExt};

pub struct HeaderMenu {
    inner: HeaderBar,

    /* From the left */
    add_object: MenuToolButton,
    del_object: MenuToolButton,
    formatting: MenuToolButton,
    alignment: MenuToolButton,
    write_mode: MenuToolButton,

    /* From the right */
    global_menu: MenuToolButton,
    save_as: MenuToolButton,
    save: MenuToolButton,
}

impl HeaderMenu {
    #[allow(deprecated)]
    pub fn new() -> Self {
        Self {
            inner: HeaderBar::new(),
            add_object: MenuToolButton::new_from_stock("gtk-new"),
            del_object: MenuToolButton::new_from_stock("gtk-new"),
            formatting: MenuToolButton::new_from_stock("gtk-new"),
            alignment: MenuToolButton::new_from_stock("gtk-new"),
            write_mode: MenuToolButton::new_from_stock("gtk-new"),
            global_menu: MenuToolButton::new_from_stock("gtk-new"),
            save_as: MenuToolButton::new_from_stock("gtk-new"),
            save: MenuToolButton::new_from_stock("gtk-new"),
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

        /* Pack left */
        self.inner.pack_start(&self.add_object);
        self.inner.pack_start(&self.del_object);
        self.inner.pack_start(&self.formatting);
        self.inner.pack_start(&self.alignment);
        self.inner.pack_start(&self.write_mode);

        /* Pack right */
        self.inner.pack_end(&self.global_menu);
        self.inner.pack_end(&self.save_as);
        self.inner.pack_end(&self.save);
    }
}
