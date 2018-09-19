
use std::sync::mpsc::{self};

/// The switchboard communicates between UI and memory state
/// 
/// **Note** This is not yet implemented and only a 
/// draft of how this _could_ work. 
/// Please leave feedback on the repository.
/// 
/// At it's core it allows UI elements to register themselves and
/// their events via RX/TX constructs so that they can interact
/// with the application state without having to take ownership
/// or mutating references themselves.
/// 
/// ## What are events even
/// 
/// Let's assume we press a button. Where the button is defined
/// we get a closure that can call some code. We give it a producer
/// to a channel that is sent to the switchboard.
/// 
/// At this point some switching code comes into play. 
/// How does this work?
/// 
/// Well...let's just steal the way that Qt does it: signals and slots.
/// 
/// When a signal comes in, it has a metadata payload which specifies
/// what kind of signal it is. It is then mapped to a slot.
/// 
/// A slot is a function that is provided by some component that is
/// called when the apropriate signal is invoked.
/// 
/// `Signal` to `Slot` mapping is deterministic but not neccessarily 1-1.
/// 
/// ## UI internal example
/// 
/// Let's say we change text-size options and as such, we change formatting
/// options on the text buffer. This does not directly touch the application
/// state, mostly the UI state.
/// 
/// The button sends a signal to the switchboard, it patches it into the
/// `change_formatting(&Formats)`-slot on the text-buffer. The apropriate
/// function is called, if it exists.
/// 
/// ## Why though?
/// 
/// This design makes the Switchboard the most complicated piece of code
/// and it's very unfortunate that Gtk doesn't have something like this
/// build-in. But it also makes it a very flat piece of code.
/// 
/// It connects signal producers to signal consumers (slots) which can
/// ideally be done with one large `match` statement.
/// 
/// Registering events and so on requires mutable state on the `Switchboard`
/// but only during creation where it can be easily passed through init-chains.
/// 
/// After that all components can share data via channels.
pub struct Switchboard {

}