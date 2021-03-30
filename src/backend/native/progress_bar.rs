#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ProgressBar {}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::progress_bar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> ProgressBar {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::progress_bar_new()) }
    // }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for ProgressBar {}
impl Is<ProgressBar> for ProgressBar {}

impl AsRef<ProgressBar> for ProgressBar {
    fn as_ref(&self) -> &ProgressBar {
        unimplemented!()
    }
}

pub const NONE_PROGRESS_BAR: Option<&ProgressBar> = None;

pub trait ProgressBarExt: 'static {
    fn get_progress(&self) -> f64;

    fn set_progress(&self, progress: f64);

    fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ProgressBar>> ProgressBarExt for O {
    fn get_progress(&self) -> f64 {
        // unsafe { ffi::progress_bar_get_progress(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn set_progress(&self, progress: f64) {
        // unsafe {
        //     ffi::progress_bar_set_progress(self.as_ref().to_glib_none().0, progress);
        // }
        unimplemented!()
    }

    fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_progress_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ProgressBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ProgressBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ProgressBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::progress\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_progress_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProgressBar")
    }
}
