//! A pop-over is like a pop-up, which disappears if mouse context is changed

mod create;
mod formatting;
mod preferences;
mod project;

pub(crate) use self::create::*;
pub(crate) use self::formatting::*;
pub(crate) use self::preferences::*;
pub(crate) use self::project::*;
