use crate::ui::Component;
use gtk::{
    self, Image, HeaderBar, HeaderBarExt, MenuToolButton, MenuToolButtonExt, MenuButton, ToolButton, ToolButtonExt,
};

pub struct HeaderMenu {
    inner: HeaderBar,

    /* From the left */
    add_object: ToolButton,
    del_object: ToolButton,
    formatting: ToolButton,
    alignment: ToolButton,
    write_mode: ToolButton,

    /* From the right */
    global_menu: ToolButton,
    save_as: ToolButton,
    save: ToolButton,
}

impl HeaderMenu {
    #[allow(deprecated)]
    pub fn new() -> Self {
        Self {
            inner: HeaderBar::new(),
            add_object: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Add"),
            del_object: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Delete"),
            formatting: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Formatting"),
            alignment: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Text Align"), 
            write_mode: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Write Mode"),
            global_menu: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Global Menu"),
            save_as: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Save As"),   
            save: ToolButton::new(&Image::new_from_icon_name("gtk-add", 32), "Save"),      
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
        self.inner.set_subtitle("<No Project>");

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
