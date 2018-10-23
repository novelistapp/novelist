use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonExt, GtkWindowExt, HeaderBarExt, Inhibit, LabelExt, OrientableExt, TextBufferExt,
    TextViewExt, WidgetExt,
};
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;

use super::headerbar::Event::ToggleExplorer as HeaderToggleExplorer;
use super::headerbar::Event::ToggleInfoBar as HeaderToggleInfoBar;
use super::headerbar::{self, HeaderBar};

use super::workspace::{self, Workspace};
use super::explorer::{self, ProjectExplorer};

use crate::novelist_core::state::AppState;

pub struct Model {
    app_state: Option<AppState>,
}

#[derive(Msg, Debug)]
pub enum Event {
    Quit,

    /* Mapping headerbar events */
    ToggleExplorer,
    ToggleInfoBar,
}

#[widget]
impl Widget for RootWindow {
    fn init_view(&mut self) {
        self.window.set_titlebar(self.titlebar.widget());
        self.window.set_default_size(800, 600);
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { app_state: None }
    }

    fn update(&mut self, e: Event) {
        debug!("Getting event: {:?}", e);
        match e {
            Event::ToggleExplorer => self.workspace.emit(workspace::Event::ToggleExplorer),
            Event::ToggleInfoBar => self.workspace.emit(workspace::Event::ToggleInfoPanel),
            Event::Quit => gtk::main_quit(),
            // _ => println!("Unknown event!")
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
                HeaderToggleExplorer => Event::ToggleExplorer,
                HeaderToggleInfoBar => Event::ToggleInfoBar,
            },
            delete_event(_, _) => (Event::Quit, Inhibit(false)),
        },
    }
}
