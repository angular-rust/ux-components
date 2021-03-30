// use glib::object::Cast;
// use glib::translate::*;

use std::fmt;
// use Widget;

// glib_wrapper! {
//     pub struct Frame(Object<ffi::Frame, ffi::FrameClass, FrameClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::frame_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Frame {

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

pub const NONE_FRAME: Option<&Frame> = None;

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frame")
    }
}
