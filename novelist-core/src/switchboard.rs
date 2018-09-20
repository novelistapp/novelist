//! See `Switchboard` type docs for detail!

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{
    mpsc::{self, channel, Receiver, Sender},
    RwLock,
};
use std::thread;

/// The switchboard communicates between UI and memory state
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
    slots: RwLock<HashMap<String, Sender<Event>>>,
}

/// A slot is simply a function we call with an `Event`
pub type Slot = Fn(Event);

impl Switchboard {
    /// Create a new instance
    pub fn new() -> Self {
        Self {
            slots: RwLock::new(HashMap::new()),
        }
    }

    // /// **Not Implemented** – **Do not call**
    // ///
    // /// Generate MPSC endpoints and fork
    // pub fn spawn(&self) {
    //     thread::spawn(|| {});
    //     unimplemented!()
    // }

    /// Adds a new slot to an ID, removing the previous one
    ///
    /// This will shortly aquire write-access to the slot list
    /// which will block all other reads that are currently
    /// trying to happen.
    ///
    /// Avoid this expensive call as often as possible.
    pub fn add_slot(&self, id: &str) -> Receiver<Event> {
        let (s, r) = channel();

        let mut w = self.slots.write().unwrap();
        w.insert(String::from(id), s);
        drop(w);

        r
    }

    /// Removes a slot again, warning if none existed
    ///
    /// This will shortly aquire write-access to the slot list
    /// which will block all other reads that are currently
    /// trying to happen.
    ///
    /// Avoid this expensive call as often as possible.
    pub fn del_slot(&self, id: &str) {
        let mut w = self.slots.write().unwrap();
        if let None = w.remove(id) {
            warn!("Tried to remove `None`-slot '{}'", id);
        }
    }

    /// Send a signal to a slot
    ///
    /// Receive a send handle for a slot
    pub fn signal(&self, id: &str, event: Event) -> Option<Sender<Event>> {
        match self.slots.read().unwrap().get(id) {
            Some(s) => Some(s.clone()),
            None => {
                warn!("Unknown slot '{}' notified", id);
                None
            }
        }
    }
}

/// Message type created by signals
pub enum Event {
    /// Simply notify a slot
    Notify,
    /// Notify a slot with a lazy text payload
    Payload(&'static str),
}

pub enum NewEvent<'outer, T> {
    Notify(&'outer T),
    NotifyMut(&'outer mut T),
}