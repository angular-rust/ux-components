#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ProgressBarFill {
    pub parent: Widget,
    pub height: u32,
}

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ProgressBar {
    pub fill: Option<clutter::Actor>,
    pub progress: f64,
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::progress_bar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ProgressBar {}
impl Is<ProgressBar> for ProgressBar {}

impl AsRef<ProgressBar> for ProgressBar {
    fn as_ref(&self) -> &ProgressBar {
        self
    }
}

pub const NONE_PROGRESS_BAR: Option<&ProgressBar> = None;

pub trait ProgressBarExt: 'static {
    /// get_progress:
    /// @bar: A #ProgressBar
    ///
    /// Get the progress of the progress bar
    ///
    /// Returns: A value between 0.0 and 1.0
    ///
    fn get_progress(&self) -> f64;

    /// set_progress:
    /// @bar: A #ProgressBar
    /// @progress: A value between 0.0 and 1.0
    ///
    /// Set the progress of the progress bar
    ///
    fn set_progress(&self, progress: f64);

    fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ProgressBar>> ProgressBarExt for O {
    /// get_progress:
    /// @bar: A #ProgressBar
    ///
    /// Get the progress of the progress bar
    ///
    /// Returns: A value between 0.0 and 1.0
    ///
    fn get_progress(&self) -> f64 {
        let progressbar = self.as_ref();
        progressbar.progress
    }

    /// set_progress:
    /// @bar: A #ProgressBar
    /// @progress: A value between 0.0 and 1.0
    ///
    /// Set the progress of the progress bar
    ///
    fn set_progress(&self, progress: f64) {
        let progressbar = self.as_ref();
        
        if progressbar.progress != progress {
            // progressbar.progress = progress;
            // allocate_fill(bar, NULL, 0);
            // clutter_actor_queue_redraw(CLUTTER_ACTOR(bar));
            // g_object_notify(G_OBJECT(bar), "progress");
        }
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
