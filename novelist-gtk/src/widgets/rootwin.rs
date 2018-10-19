use gtk::{
    ButtonExt,
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
    GtkWindowExt,
    HeaderBarExt,
    TextBufferExt,
    TextViewExt,
};
use gtk::prelude::*;
use gtk::Orientation::{Vertical,Horizontal};
use relm::{Relm, Widget, timeout};
use relm_attributes::widget;

use super::headerbar::{self, HeaderBar, Event::Add as HeaderBarAdd};
use super::workspace::{self, Workspace};

pub struct Model {
    /* to be determined */
}

#[derive(Msg)]
pub enum Event {
    Quit,
    MappingSaveToShow
}

#[widget]
impl Widget for RootWindow {
    fn init_view(&mut self) {
        self.window.set_titlebar(self.titlebar.widget());
        self.window.set_default_size(800, 600);
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { }
    }

    fn update(&mut self, e: Event) {
        println!("Receiving root windo mapping event");
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
                // Save => workspace::Event::ShowOther
                HeaderBarAdd => Event::MappingSaveToShow,
            },
            delete_event(_, _) => (Event::Quit, Inhibit(false)),
        },

    }
}