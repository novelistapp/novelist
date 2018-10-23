use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonExt, GtkWindowExt, HeaderBarExt, Inhibit, LabelExt, OrientableExt, TextBufferExt,
    TextViewExt, WidgetExt
};
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;

use super::explorer::{self, ProjectExplorer};
use super::textview::{self, TextView};
use super::info_bar::{self, InfoPanel};

pub struct Model {/* to be determined */}

#[derive(Msg, Debug)]
pub enum Event {
    ToggleExplorer,
    ToggleInfoPanel,
}

#[widget]
impl Widget for Workspace {
    fn init_view(&mut self) {
        // self.headerbar.set_subtitle("<Project loaded>");
        // self.text_view.get_buffer().expect("Why no buffer?").set_text("Hallo Welt!");
        // self.other_text.get_buffer().expect("Why no buffer?").set_text("Hello World!");
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, e: Event) {
        debug!("Getting event: {:?}", e);
        match e {
            Event::ToggleExplorer => self.explorer.emit(explorer::Event::ToggleVisibility),
            Event::ToggleInfoPanel => self.info_panel.emit(info_bar::Event::ToggleVisibility),
        }
    }

    view! {
        #[name="workspace"]
        gtk::Box {
            orientation: Horizontal,
            #[name="explorer"]
            ProjectExplorer {},
            #[name="text_view"]
            TextView {},
            #[name="info_panel"]
            InfoPanel {},
        },
    }
}
