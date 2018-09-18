//! Core UI entry-point
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
//! Inter-component interactions are simply two Rust modules calling
//! functions on each other. Shared state is provided via their parent
//! window.
//!
//! The master window is parent to all components.

pub(crate) mod popover;
pub(crate) mod traits;
pub(crate) mod widgets;
pub(crate) mod window;

use gtk;

mod icons;
pub(crate) use self::icons::*;

/// **Blocking function** which initialises all UI elements
pub(crate) fn start_ui() {
    gtk::init().unwrap();

    let mut w = window::root::RootWindow::new();
    w.init();

    gtk::main();
}