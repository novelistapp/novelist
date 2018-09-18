use crate::ui::traits::{Component, NovelistPopover};
use gtk::{self, prelude::*, *};

/// Containts formatting settings
pub struct FormattingPopover {
    inner: Option<Popover>,
}

impl FormattingPopover {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn init<W: IsA<Widget>>(&mut self, parent: &W) {
        self.inner = Some(Popover::new(parent));

        /* Chain the std-init call on top */
        <Self as Component>::init(self);
    }
}

impl Component for FormattingPopover {
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

impl NovelistPopover for FormattingPopover {}
