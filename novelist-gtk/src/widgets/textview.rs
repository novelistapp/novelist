use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    BoxExt, BuildableExt, ButtonExt, ContainerExt, GtkWindowExt, HeaderBarExt, Inhibit, LabelExt,
    OrientableExt, ScrolledWindowExt, TextBufferExt, TextViewExt, WidgetExt,
};
use relm::{timeout, Relm, Widget};
use relm_attributes::widget;

pub struct Model {/* to be determined */}

#[derive(Msg)]
pub enum Event {}

#[widget]
impl Widget for TextView {
    fn init_view(&mut self) {}

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, e: Event) {}

    view! {
        #[name = "container"]
        gtk::Box {
            child: {
                fill: true,
                expand: true,
            },
            gtk::ScrolledWindow {
                shadow_type: gtk::ShadowType::In,
                property_width_request: 600,

                #[name = "textview"]
                gtk::TextView {
                    margin_left: 25,
                    margin_right: 25,
                    margin_top: 25,
                    margin_bottom: 25,
                    pixels_below_lines: 25,
                    pixels_inside_wrap: 15,
                    wrap_mode: gtk::WrapMode::Word,
                    hscroll_policy: gtk::ScrollablePolicy::Natural,
                    left_margin: 50,
                    right_margin: 50,
                    top_margin: 75,
                    bottom_margin: 75,
                    indent: 50,
                    accepts_tab: false,
                }
            }
        }
    }
}
