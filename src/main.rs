extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate gio;
extern crate gtk;

use std::env::args;
use gio::ApplicationExtManual;

mod utils;
mod state;
pub mod gui;

use state::data::Novel;

fn main() {
    let mut n = Novel::new(String::from("Starlike"), String::from("Katharina Ariane"), "/home/spacekookie/Desktop").unwrap();
    //let mut n = Novel::load("/home/spacekookie/Desktop/Starlike/Starlike.novel").unwrap();
    n.add_chapter("Prologue", "Kicking off the story and such");
    println!("{:#?}", n);

    //Novel::create(".", "Starlike", "Katharina Ariane").unwrap();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let application = gui::get_application();
    application.run(&args().collect::<Vec<_>>());
}
