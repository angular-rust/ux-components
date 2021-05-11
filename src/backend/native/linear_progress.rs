#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct LinearProgressFill {
    pub parent: Widget,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct LinearProgressProps {
    pub fill: Option<Actor>,
    pub progress: f64,
}

#[derive(Clone, Debug)]
pub struct LinearProgress {
    props: RefCell<LinearProgressProps>,
    widget: Widget,
}

impl LinearProgress {
    pub fn new() -> LinearProgress {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::progress_bar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for LinearProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for LinearProgress {}
impl Is<LinearProgress> for LinearProgress {}

impl AsRef<LinearProgress> for LinearProgress {
    fn as_ref(&self) -> &LinearProgress {
        self
    }
}

impl Is<Widget> for LinearProgress {}

impl AsRef<Widget> for LinearProgress {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for LinearProgress {}

impl AsRef<Actor> for LinearProgress {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait LinearProgressExt: 'static {
    /// get_progress:
    /// @bar: A #LinearProgress
    ///
    /// Get the progress of the progress bar
    ///
    /// Returns: A value between 0.0 and 1.0
    ///
    fn get_progress(&self) -> f64;

    /// set_progress:
    /// @bar: A #LinearProgress
    /// @progress: A value between 0.0 and 1.0
    ///
    /// Set the progress of the progress bar
    ///
    fn set_progress(&self, progress: f64);

    fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<LinearProgress>> LinearProgressExt for O {
    /// get_progress:
    /// @bar: A #LinearProgress
    ///
    /// Get the progress of the progress bar
    ///
    /// Returns: A value between 0.0 and 1.0
    ///
    fn get_progress(&self) -> f64 {
        let progressbar = self.as_ref();
        let props = progressbar.props.borrow();

        props.progress
    }

    /// set_progress:
    /// @bar: A #LinearProgress
    /// @progress: A value between 0.0 and 1.0
    ///
    /// Set the progress of the progress bar
    ///
    fn set_progress(&self, progress: f64) {
        let progressbar = self.as_ref();
        let mut props = progressbar.props.borrow_mut();

        if props.progress != progress {
            props.progress = progress;
            // allocate_fill(bar, None, 0);
            // actor_queue_redraw(CLUTTER_ACTOR(bar));
            // g_object_notify(G_OBJECT(bar), "progress");
        }
    }

    fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_progress_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::LinearProgress,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<LinearProgress>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&LinearProgress::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for LinearProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LinearProgress")
    }
}
