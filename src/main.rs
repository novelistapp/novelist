#![feature(iterator_flatten)]
// Make clippy shut up about a whole bunch
//    of stuff we don't care about right now...
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate gio;
extern crate gtk;

use gio::ApplicationExtManual;
use std::env::args;

pub mod gui;
mod state;
mod utils;

use state::data::{Chapter, Document, Novel};

fn main() {
    let mut n = Novel::new(
        String::from("Starlike"),
        String::from("Katharina Ariane"),
        "/home/spacekookie/Desktop",
    ).unwrap();
    n.add_chapter("Prologue", "Kicking off the story and such")
        .add_scene("Prologue".to_owned(), "The story starts...".to_owned())
        .append(
            "It was a warm summer day. BLa bla bla bla bla bla bla \
             MOre text omg this is all one fucking god damn text snippet because \
             FUCK THIS FUCKING BULLSHIT AAAAAAAAAAAAH!!!!",
        );

    n.save().unwrap();

    // let mut n = Novel::load("/home/spacekookie/Desktop/Starlike/Starlike.novel").unwrap();
    // println!("{:#?}", n);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let application = gui::get_application();
    application.run(&args().collect::<Vec<_>>());
}
