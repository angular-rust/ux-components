#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Adjustment, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ScrollBar {}

impl ScrollBar {
    pub fn new() -> ScrollBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::scroll_bar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn with_adjustment<P: Is<Adjustment>>(adjustment: &P) -> ScrollBar {
    //     // skip_assert_initialized!();
    //     // unsafe {
    //     //     clutter::Actor::from_glib_none(ffi::scroll_bar_new_with_adjustment(
    //     //         adjustment.as_ref().to_glib_none().0,
    //     //     ))
    //     //     .unsafe_cast()
    //     // }
    //     unimplemented!()
    // }

    // pub fn new() -> ScrollBar {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::scroll_bar_new()) }
    // }

    // pub fn with_adjustment<P: Is<Adjustment>>(adjustment: &P) -> ScrollBar {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::scroll_bar_new_with_adjustment()) }
    // }
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for ScrollBar {}
impl Is<ScrollBar> for ScrollBar {}

impl AsRef<ScrollBar> for ScrollBar {
    fn as_ref(&self) -> &ScrollBar {
        unimplemented!()
    }
}

pub const NONE_SCROLL_BAR: Option<&ScrollBar> = None;

pub trait ScrollBarExt: 'static {
    fn get_adjustment(&self) -> Option<Adjustment>;

    //fn get_orientation(&self) -> /*Ignored*/Orientation;

    // fn set_adjustment<P: Is<Adjustment>>(&self, adjustment: &P);

    //fn set_orientation(&self, orientation: /*Ignored*/Orientation);

    fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ScrollBar>> ScrollBarExt for O {
    fn get_adjustment(&self) -> Option<Adjustment> {
        // unsafe {
        //     from_glib_none(ffi::scroll_bar_get_adjustment(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    //fn get_orientation(&self) -> /*Ignored*/Orientation {
    //    unsafe { TODO: call ffi:scroll_bar_get_orientation() }
    //}

    // fn set_adjustment<P: Is<Adjustment>>(&self, adjustment: &P) {
    //     unsafe {
    //         ffi::scroll_bar_set_adjustment(
    //             self.as_ref().to_glib_none().0,
    //             adjustment.as_ref().to_glib_none().0,
    //         );
    //     }
    // }

    //fn set_orientation(&self, orientation: /*Ignored*/Orientation) {
    //    unsafe { TODO: call ffi:scroll_bar_set_orientation() }
    //}

    fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn scroll_start_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"scroll-start\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             scroll_start_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn scroll_stop_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"scroll-stop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             scroll_stop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::adjustment\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_adjustment_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::orientation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_orientation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ScrollBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollBar")
    }
}
