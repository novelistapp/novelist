//! All the models used in Novelist

mod io;
pub(crate) mod data;
mod text;

use self::data::Novel;

/// A list of global, app-level errors
///
/// These will end in messages to the user!
pub(crate) enum AppErrors {
    OpenProject,
}

/// A structure that contains active application state
///
/// It keeps track of open projects, unsaved work
/// and the likes
pub(crate) struct AppState {
    pub active_novel: Option<Novel>,
}

impl AppState {

    /// Try to open a new project, only works if no project is already
    /// open. A bit of a utility function.
    /// 
    /// (Not sure how useful 😅)
    pub fn open_novel(&mut self, n: Novel) -> Result<(), AppErrors> {
        return match self.active_novel {
            Some(_) => Err(AppErrors::OpenProject),
            None => {
                self.active_novel = Some(n);
                Ok(())
            }
        };
    }
}
