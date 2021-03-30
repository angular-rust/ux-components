#![allow(unused_variables)]

use super::Widget;
use crate::prelude::*;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Frame {}

impl Frame {
    pub fn new() -> Frame {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::frame_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Frame {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::frame_new()) }
    // }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for Frame {}
impl Is<Frame> for Frame {}

impl AsRef<Frame> for Frame {
    fn as_ref(&self) -> &Frame {
        unimplemented!()
    }
}

pub const NONE_FRAME: Option<&Frame> = None;

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame")
    }
}
