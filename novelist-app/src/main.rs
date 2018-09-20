#![allow(unused)]

extern crate clap;
extern crate novelist_core as core;
extern crate novelist_gtk as ui;

use crate::core::logger;
use crate::core::switchboard::{Switchboard, Event, Slot};

fn main() {
    logger::setup().expect("Failed to initialise stdout logger!");

    let switch = Switchboard::new();
    ui::start_ui(&switch);
}
