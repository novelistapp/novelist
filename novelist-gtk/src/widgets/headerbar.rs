use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonExt, GtkWindowExt, HeaderBarExt, Inhibit, LabelExt, OrientableExt, TextBufferExt,
    TextViewExt, WidgetExt,
};
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;

pub struct Model {/* to be determined */}

#[derive(Msg, Debug)]
pub enum Event {
    Add,
    Delete,
    ToggleExplorer,
    ToggleInfoBar,
    Formatting,
    Alignment,
    WriteMode,
    GlobalMenu,
    SaveAs,
    Save,
}

#[widget]
impl Widget for HeaderBar {
    fn init_view(&mut self) {
        // self.headerbar.set_subtitle("<Project loaded>");
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, e: Event) {
        debug!("Getting event: {:?}", e);
    }

    view! {
        #[name="headerbar"]
        gtk::HeaderBar {
            show_close_button: true,
            title: "Novelist",
            subtitle: "<No Project>",
            gtk::ToolButton {
                icon_name: "document-new",
                label: "Add",
                clicked => Event::Add
            },
            gtk::ToolButton {
                icon_name: "edit-delete",
                label: "Delete",
                clicked => Event::Delete
            },

            gtk::Separator { },

            gtk::ToolButton {
                icon_name: "gtk-dnd-multiple",
                label: "Explorer",
                clicked => Event::ToggleExplorer
            },
            gtk::ToolButton {
                icon_name: "dialog-information",
                label: "Info Panel",
                clicked => Event::ToggleInfoBar
            },

            gtk::Separator { },

            gtk::ToolButton {
                icon_name: "format-text-italic",
                label: "Formatting",
                clicked => Event::Formatting
            },
            gtk::ToolButton {
                icon_name: "format-justify-left",
                label: "Text Aligment",
                clicked => Event::Alignment
            },
            gtk::ToolButton {
                icon_name: "insert-text",
                label: "Write Mode",
                clicked => Event::WriteMode
            },

            gtk::ToolButton {
                 child: {
                    pack_type: ::gtk::PackType::End,
                },

                icon_name: "preferences-system",
                label: "Global Menu",
                clicked => Event::GlobalMenu
            },
            gtk::ToolButton {
                 child: {
                    pack_type: ::gtk::PackType::End,
                },

                icon_name: "document-save-as",
                label: "Save As",
                clicked => Event::SaveAs
            },
            gtk::ToolButton {
                 child: {
                    pack_type: ::gtk::PackType::End,
                },

                icon_name: "document-save",
                label: "Save",
                clicked => Event::Save
            },

            gtk::SearchEntry {
                child: {
                    pack_type: ::gtk::PackType::End,
                },
            },
        },
    }
}
