/*
 * Copyright (c) 2018 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
// #[macro_use]
// extern crate gtk_test;

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

use self::Msg::*;

// Define the structure of the model.
pub struct Model {
    text: String,
}

// The messages that can be sent to the update function.
#[derive(Msg)]
pub enum Msg {
    Quit,
    Save,
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        self.window.set_titlebar(&self.titlebar);
        self.window.set_default_size(800, 600);
        self.text_view.get_buffer().expect("Why no buffer?").set_text(self.model.text.as_str());
    }

    // The initial model.
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            text: String::from("Hello"),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            Save => {
                let buffer = self.text_view.get_buffer().expect("why no buffer");
                &self.model.text.clear();
                // &self.model.text = 
                println!("{}",buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), false).unwrap());
            },
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: "Meine Buttons",

            gtk::Box {
                // Set the orientation property of the Box.
                orientation: Vertical,
                // THE TEXTFIELD
                #[name="text_view"]
                gtk::TextView {

                }
            },
            #[name="titlebar"]
            gtk::HeaderBar {
                show_close_button: true,
                title: "Novelist",
                subtitle: "<No Project>",
                gtk::ToolButton {
                    icon_name: "document-new",
                    label: "Add",
                },
                gtk::ToolButton {
                    icon_name: "edit-delete",
                    label: "Delete",
                },
                gtk::ToolButton {
                    icon_name: "format-text-italic",
                    label: "Formatting",
                },
                gtk::ToolButton {
                    icon_name: "format-justify-left",
                    label: "Text Aligment",
                },
                gtk::ToolButton {
                    icon_name: "insert-text",
                    label: "Write Mode",
                },
                gtk::ToolButton {
                    icon_name: "preferences-system",
                    label: "Global Menu",
                },
                gtk::ToolButton {
                    icon_name: "document-save-as",
                    label: "Save As",
                },
                gtk::ToolButton {
                    icon_name: "document-save",
                    label: "Save",
                    clicked => Save
                },


                gtk::SearchEntry {
                    child: {
                        pack_type: ::gtk::PackType::End,
                    },
                    // search_changed(entry) => Msg::Search(entry.get_text().unwrap().to_string()),
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

// #[cfg(test)]
// mod tests {
//     use gtk::LabelExt;

//     use relm;
//     use gtk_test::click;

//     use Win;

//     #[test]
//     fn label_change() {
//         let (_component, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
//         let dec_button = &widgets.dec_button;
//         let label = &widgets.label;

//         click(dec_button);
//         assert_text!(label, -1);
//         click(dec_button);
//         assert_text!(label, -2);
//     }
// }
