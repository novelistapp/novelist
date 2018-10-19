//! Novelist core library
//!
//! Handles all common functions, data mapping
//! and persistence as well as utilit code.
#![feature(crate_in_paths)]

#[macro_use]
pub extern crate log;

#[macro_use]
pub extern crate serde_derive;
pub extern crate chrono;
pub extern crate fern;
pub extern crate rayon;
pub extern crate serde;
pub extern crate serde_json;

pub mod logger;
pub mod state;

// Some novelist-core internal utils
mod utils;
