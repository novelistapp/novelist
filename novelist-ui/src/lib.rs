//! Novelist UI toolkit
//!
//! This is a wrapper around a lot of Gtk code and types that enforce
//! a good design for when Novelist has grown past a certain size.
//!
//! *When adding code here, read the existing documentation carefully
//! to figure out if something is the best way of handling things.*
//!
//! The UI is split along components to make dealing with UI and
//! code changes easier.
//! **What is a component?** – good thing you ask: a component is
//! some piece of UI element that is *atomic*, which means that it
//! makes no sense to break it down any further, without removing
//! the context of it's design.
//!
//! A pop-over is a component. The text input area is a component.
//! The project sidebar is a component. The master window
//! is a component. etc, you get the point 😉
//!
//! Components are clustered into broad categories to make
//! searching easier. In each category a component is unique.
//!
//! Inter-component interactions are done via Gtk events
//! (more on that...soon)
//!
//! The master window is parent to all components.

extern crate novelist_core;

mod icons;
mod popover;
mod traits;
mod widgets;
mod window;

use crate::traits::Component;
use gtk;

/// **Blocking function** which initialises all UI elements
pub fn start_ui() {
    gtk::init().unwrap();

    let mut w = window::root::RootWindow::new();

    debug!("Initialising Novelist root window");
    w.init();

    gtk::main();
}
