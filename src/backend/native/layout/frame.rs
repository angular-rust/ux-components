#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Widget};
use std::fmt;

#[derive(Debug)]
pub struct Frame {
    pub child: Option<Actor>,
    widget: Widget,
}

impl Frame {
    pub fn new() -> Frame {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::frame_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Frame {}
impl Is<Frame> for Frame {}

impl AsRef<Frame> for Frame {
    fn as_ref(&self) -> &Frame {
        self
    }
}

impl Is<Widget> for Frame {}

impl AsRef<Widget> for Frame {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Frame {}

impl AsRef<Actor> for Frame {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame")
    }
}
