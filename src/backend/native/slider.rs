#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Actor;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Slider {}

impl Slider {
    pub fn new() -> Slider {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::slider_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Slider {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::slider_new()) }
    // }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Slider {}
impl Is<Slider> for Slider {}

impl AsRef<Slider> for Slider {
    fn as_ref(&self) -> &Slider {
        self
    }
}

pub const NONE_SLIDER: Option<&Slider> = None;

pub trait SliderExt: 'static {
    fn get_buffer_value(&self) -> f64;

    fn get_value(&self) -> f64;

    fn set_buffer_value(&self, value: f64);

    fn set_value(&self, value: f64);

    fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Slider>> SliderExt for O {
    fn get_buffer_value(&self) -> f64 {
        // unsafe { ffi::slider_get_buffer_value(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_value(&self) -> f64 {
        // unsafe { ffi::slider_get_value(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn set_buffer_value(&self, value: f64) {
        // unsafe {
        //     ffi::slider_set_buffer_value(self.as_ref().to_glib_none().0, value);
        // }
        unimplemented!()
    }

    fn set_value(&self, value: f64) {
        // unsafe {
        //     ffi::slider_set_value(self.as_ref().to_glib_none().0, value);
        // }
        unimplemented!()
    }

    fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn slide_start_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Slider,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Slider>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"slide-start\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             slide_start_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn slide_stop_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Slider,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Slider>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"slide-stop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             slide_stop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_buffer_value_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Slider,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Slider>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::buffer-value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_buffer_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Slider,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Slider>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Slider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slider")
    }
}
