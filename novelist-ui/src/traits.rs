use gtk::{self, IsA, Widget};

/// Define some shared characteristics of components
pub trait Component {
    type WrappedType: IsA<Widget>;

    /// A function that is called on *all* components after initialisation
    ///
    /// This function should do things like set-up callbacks,
    /// events and actions that can be triggered by the
    /// object outside of it's creation process.
    fn init(&mut self);

    /// Simply return a reference to the inner wrap type
    fn as_ref(&self) -> &Self::WrappedType;

    /// Return a mutable reference to the inner wrap type
    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType;
}

/// A button that can spawn a Popover
pub trait PopoverButton {
    type Spawns: NovelistPopover;

    fn set_popover(&mut self, impl NovelistPopover);
}

/// A zero-sized marker trait
pub trait NovelistPopover {}
