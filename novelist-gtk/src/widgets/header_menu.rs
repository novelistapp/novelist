use crate::{
    popover::{CreatePopover, FormattingPopover},
    traits::Component,
    widgets::{ImageButton, ImageMenuButton},
};
use gtk::{self, *};

/// The main menu at the top of a Novelist window
pub struct HeaderMenu {
    inner: HeaderBar,

    /* From the left */
    add_object: ImageMenuButton,
    del_object: ImageButton,
    formatting: ImageButton,
    alignment: ImageButton,
    write_mode: ImageButton,

    /* From the right */
    global_menu: ImageButton,
    save_as: ImageMenuButton,
    save: ImageButton,

    /* Spawnable menu types */
    formatting_menu: FormattingPopover,
    create_menu: CreatePopover,
}

impl HeaderMenu {
    pub fn new() -> Self {
        Self {
            inner: HeaderBar::new(),
            add_object: ImageMenuButton::new("gtk-new", "Add", 32),
            del_object: ImageButton::new("gtk-delete", "Delete", 32),
            formatting: ImageButton::new("gtk-italic", "Formatting", 32),
            alignment: ImageButton::new("gtk-justify-left", "Text Aligment", 32),
            write_mode: ImageButton::new("gtk-edit", "Write Mode", 32),
            global_menu: ImageButton::new("fuck knows", "Global Menu", 32),
            save_as: ImageMenuButton::new("gtk-save-as", "Save As", 32),
            save: ImageButton::new("gtk-save", "Save", 32),
            formatting_menu: FormattingPopover::new(),
            create_menu: CreatePopover::new(),
        }
    }
}

impl Component for HeaderMenu {
    type WrappedType = HeaderBar;

    fn init(&mut self) {
        self.inner.set_show_close_button(true);
        self.inner.set_title("Novelist");
        self.inner.set_subtitle("<No Project>");

        /* Pack left */
        self.inner.pack_start(self.add_object.as_ref());
        self.inner.pack_start(self.del_object.as_ref());
        self.inner.pack_start(self.formatting.as_ref());
        self.inner.pack_start(self.alignment.as_ref());
        self.inner.pack_start(self.write_mode.as_ref());

        /* Pack right */
        self.inner.pack_end(self.global_menu.as_ref());
        self.inner.pack_end(self.save_as.as_ref());
        self.inner.pack_end(self.save.as_ref());

        /* Map popovers */
        self.formatting_menu.init(self.formatting.as_ref());
        self.create_menu.init(self.add_object.as_ref());

        /* Map events */
        // TODO: Create events :)
    }

    fn as_ref(&self) -> &Self::WrappedType {
        &self.inner
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        &mut self.inner
    }
}
