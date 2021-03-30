#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Spinner {}

impl Spinner {
    pub fn new() -> Spinner {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::spinner_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Spinner {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::spinner_new()) }
    // }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for Spinner {}
impl Is<Spinner> for Spinner {}

impl AsRef<Spinner> for Spinner {
    fn as_ref(&self) -> &Spinner {
        unimplemented!()
    }
}

pub const NONE_SPINNER: Option<&Spinner> = None;

pub trait SpinnerExt: 'static {
    fn get_animating(&self) -> bool;

    fn set_animating(&self, animating: bool);

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Spinner>> SpinnerExt for O {
    fn get_animating(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::spinner_get_animating(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_animating(&self, animating: bool) {
        // unsafe {
        //     ffi::spinner_set_animating(self.as_ref().to_glib_none().0, animating.to_glib());
        // }
        unimplemented!()
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
