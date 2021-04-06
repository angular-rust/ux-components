#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Spinner {
    // pub texture: cogl::Handle,
    // pub material: cogl::Handle,
    pub frames: u32,
    pub anim_duration: u32,
    pub current_frame: u32,
    pub update_id: u32,
    pub animating: bool,
}

impl Spinner {
    pub fn new() -> Spinner {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::spinner_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Spinner {}
impl Is<Spinner> for Spinner {}

impl AsRef<Spinner> for Spinner {
    fn as_ref(&self) -> &Spinner {
        self
    }
}

pub const NONE_SPINNER: Option<&Spinner> = None;

pub trait SpinnerExt: 'static {
    /// get_animating:
    /// @spinner: A #Spinner widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %TRUE if the spinner is animating, %FALSE otherwise
    ///
    fn get_animating(&self) -> bool;

    /// set_animating:
    /// @spinner: A #Spinner widget
    /// @animating: %TRUE to enable animation, %FALSE to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool);

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Spinner>> SpinnerExt for O {
    /// get_animating:
    /// @spinner: A #Spinner widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %TRUE if the spinner is animating, %FALSE otherwise
    ///
    fn get_animating(&self) -> bool {
        let spinner = self.as_ref();
        spinner.animating
    }

    /// set_animating:
    /// @spinner: A #Spinner widget
    /// @animating: %TRUE to enable animation, %FALSE to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool) {
        let spinner = self.as_ref();
        
        if spinner.animating != animating {
            spinner.animating = animating;
            update_timeout(spinner);
            g_object_notify(G_OBJECT(spinner), "animating");
        }
    }

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn looped_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Spinner,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Spinner>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Spinner::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"looped\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             looped_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_animating_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Spinner,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Spinner>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Spinner::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::animating\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_animating_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spinner")
    }
}
