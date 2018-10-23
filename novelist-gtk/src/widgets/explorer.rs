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
    ToggleVisibility,
}

#[widget]
impl Widget for ProjectExplorer {
    fn init_view(&mut self) {}

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, e: Event) {
        debug!("Getting event: {:?}", e);
        match e {
            Event::ToggleVisibility => self.container.set_visible(!self.container.get_visible()),
        }
    }

    view! {
        #[name="container"]
        gtk::Box {
            property_width_request: 300,
            visible: false,
            orientation: Vertical,
        },
    }
}
