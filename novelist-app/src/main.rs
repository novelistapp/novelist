extern crate clap;
extern crate novelist_core as core;
extern crate novelist_ui as ui;

use crate::core::logger;

fn main() {
    logger::setup().expect("Failed to initialise stdout logger!");
    ui::start_ui();
}
