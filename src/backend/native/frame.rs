#![allow(unused_variables)]

use super::Widget;
use crate::prelude::*;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Frame {
    pub child: Option<clutter::Actor>,
    widget: Widget,
}

impl Frame {
    pub fn new() -> Frame {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::frame_new()).unsafe_cast() }
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

impl Is<clutter::Actor> for Frame {}

impl AsRef<clutter::Actor> for Frame {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_FRAME: Option<&Frame> = None;

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame")
    }
}
