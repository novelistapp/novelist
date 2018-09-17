// Make clippy shut up about a whole bunch
//    of stuff we don't care about right now...
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(non_modrs_mods)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rayon;

extern crate gio;
extern crate gtk;

use gio::ApplicationExtManual;
use std::env::args;

pub mod gui;
mod state;
mod utils;
mod ui;

fn main() {
    

    // if gtk::init().is_err() {
    //     println!("Failed to initialize GTK.");
    //     return;
    // }
    // let application = gui::get_application();
    // application.run(&args().collect::<Vec<_>>());
}
