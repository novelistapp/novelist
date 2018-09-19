use crate::traits::{Component, NovelistPopover};

use gtk::{self, prelude::*, *};

/// Displays a list of items to be created (drop-down menu-like)
pub struct CreatePopover {
    inner: Option<Popover>,
}

impl CreatePopover {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn init<W: IsA<Widget>>(&mut self, parent: &W) {
        self.inner = Some(Popover::new(parent));

        /* Chain the std-init call on top */
        <Self as Component>::init(self);
    }
}

impl Component for CreatePopover {
    type WrappedType = Popover;

    fn init(&mut self) {}

    fn as_ref(&self) -> &Self::WrappedType {
        if self.inner.is_none() {
            warn!("About to `unwrap()` None value `Popover`");
        }

        self.inner.as_ref().unwrap()
    }

    fn inner_mut_ref(&mut self) -> &mut Self::WrappedType {
        if self.inner.is_none() {
            warn!("About to `unwrap()` None value `Popover`");
        }

        self.inner.as_mut().unwrap()
    }
}

impl NovelistPopover for CreatePopover {}
