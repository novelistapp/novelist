extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate gio;
extern crate gtk;

use std::env::args;
use gtk::{ApplicationWindow, Builder, Menu, MenuItem, MenuItemExt};
use gio::prelude::*;
use gtk::prelude::*;

mod state;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("../main.ui");
    let builder = Builder::new_from_string(ui_src);

    let window: ApplicationWindow = builder.get_object("mainwindow").expect("Couldn't get Window");

    window.set_application(application);
    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let quit: MenuItem = builder.get_object("mainwindow-menu-file-quit").expect("Couldn't get Quit option");

    quit.connect_activate(clone!(window => move |_| {
        window.destroy();
    }));

    window.show_all();
}

fn main() {
    //Novel::create(".", "Starlike", "Katharina Ariane").unwrap();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let application =  
        gtk::Application::new("org.novelist", gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(move |app| { build_ui(app); });
    application.connect_activate(|_| {});

    /* Run our app */
    application.run(&args().collect::<Vec<_>>());
}
