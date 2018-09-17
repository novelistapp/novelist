// Make clippy shut up about a whole bunch
//    of stuff we don't care about right now...

#![allow(unused)]
#![feature(crate_in_paths)]

#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate serde;
extern crate serde_json;

extern crate gio;
extern crate gtk;

use gio::ApplicationExtManual;
use std::env::args;

pub mod gui;
mod state;
mod ui;
mod utils;

fn main() {
    ui::start_ui();

    // if gtk::init().is_err() {
    //     println!("Failed to initialize GTK.");
    //     return;
    // }
    // let application = gui::get_application();
    // application.run(&args().collect::<Vec<_>>());
}
