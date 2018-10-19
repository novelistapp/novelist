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

#[derive(Msg)]
pub enum Event {
    ShowOther
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
        println!("Getting an update on Workspace");
        // self.other_box_text.set_visible(true);
    }

    view! {
        #[name="workspace"]
        gtk::Box {
            orientation: Horizontal,
            ProjectExplorer {},
            TextView {},
            InfoPanel {},
        },
    }
}
