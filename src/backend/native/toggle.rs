#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Actor;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Toggle {}

impl Toggle {
    pub fn new() -> Toggle {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::toggle_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Toggle {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::toggle_new()) }
    // }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Toggle {}
impl Is<Toggle> for Toggle {}

impl AsRef<Toggle> for Toggle {
    fn as_ref(&self) -> &Toggle {
        self
    }
}

pub const NONE_TOGGLE: Option<&Toggle> = None;

pub trait ToggleExt: 'static {
    fn get_active(&self) -> bool;

    fn set_active(&self, active: bool);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Toggle>> ToggleExt for O {
    fn get_active(&self) -> bool {
        // unsafe { from_glib(ffi::toggle_get_active(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn set_active(&self, active: bool) {
        // unsafe {
        //     ffi::toggle_set_active(self.as_ref().to_glib_none().0, active.to_glib());
        // }
        unimplemented!()
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Toggle,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Toggle>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Toggle::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toggle")
    }
}
