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

use super::headerbar::{self, HeaderBar};

pub struct Model {
    /* to be determined */
}

#[derive(Msg)]
pub enum Event {
}

#[widget]
impl Widget for RootWindow {
    fn init_view(&mut self) {
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model { }
    }

    fn update(&mut self, e: Event) {}

    view! {
        #[name="window"]
        gtk::Window {
            title: "Novelist"
        },
        HeaderBar {}
    }
}