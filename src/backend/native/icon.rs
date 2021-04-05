#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Icon {}

impl Icon {
    pub fn new() -> Icon {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::icon_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Icon {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::icon_new()) }
    // }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Icon {}
impl Is<Icon> for Icon {}

impl AsRef<Icon> for Icon {
    fn as_ref(&self) -> &Icon {
        self
    }
}

pub const NONE_ICON: Option<&Icon> = None;

pub trait IconExt: 'static {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_size(&self) -> i32;

    fn set_icon_name(&self, icon_name: &str);

    fn set_icon_size(&self, size: i32);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Icon>> IconExt for O {
    fn get_icon_name(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::icon_get_icon_name(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_icon_size(&self) -> i32 {
        // unsafe { ffi::icon_get_icon_size(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn set_icon_name(&self, icon_name: &str) {
        // unsafe {
        //     ffi::icon_set_icon_name(
        //         self.as_ref().to_glib_none().0,
        //         icon_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_icon_size(&self, size: i32) {
        // unsafe {
        //     ffi::icon_set_icon_size(self.as_ref().to_glib_none().0, size);
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Icon,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Icon>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Icon,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Icon>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-size\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_size_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Icon")
    }
}
