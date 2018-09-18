extern crate clap;
extern crate novelist_core as core;
extern crate novelist_ui as ui;

use core::logging;

fn main() {
    logging::setup_logger().expect("Failed to initialise stdout logger!");
    ui::start_ui();
}
