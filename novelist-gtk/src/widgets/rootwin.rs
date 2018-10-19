use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonExt, GtkWindowExt, HeaderBarExt, Inhibit, LabelExt, OrientableExt, TextBufferExt,
    TextViewExt, WidgetExt,
};
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;

// use super::headerbar::Event::Add as HeaderBarAdd;
// use super::headerbar::Event::Alignment as HeaderBarAlignment;
// use super::headerbar::Event::Delete as HeaderBarDelete;
// use super::headerbar::Event::Formatting as HeaderBarFormatting;
// use super::headerbar::Event::GlobalMenu as HeaderBarGlobalMenu;
// use super::headerbar::Event::Save as HeaderBarSave;
// use super::headerbar::Event::SaveAs as HeaderBarSaveAs;
// use super::headerbar::Event::WriteMode as HeaderBarWriteMode;
use super::headerbar::{self, HeaderBar, Event as HeaderEvent};

use super::workspace::{self, Workspace};

pub struct Model {/* to be determined */}

#[derive(Msg)]
pub enum Event {
    Quit,
    MappingSaveToShow,
}

#[widget]
impl Widget for RootWindow {
    fn init_view(&mut self) {
        self.window.set_titlebar(self.titlebar.widget());
        self.window.set_default_size(800, 600);
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, e: Event) {
        match e {
            Event::MappingSaveToShow => self.workspace.emit(workspace::Event::ShowOther),
            Event::Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "Novelist",
            #[name = "workspace"]
            Workspace {},
            #[name="titlebar"]
            HeaderBar {
                HeaderEvent::Add => Event::MappingSaveToShow,
            },
            delete_event(_, _) => (Event::Quit, Inhibit(false)),
        },

    }
}
