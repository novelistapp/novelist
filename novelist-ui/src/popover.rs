//! A pop-over is like a pop-up, which disappears if mouse context is changed

mod create;
mod formatting;
mod preferences;
mod project;

pub use self::create::*;
pub use self::formatting::*;
pub use self::preferences::*;
pub use self::project::*;
