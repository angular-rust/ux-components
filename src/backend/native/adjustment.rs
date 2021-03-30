#![allow(unused_variables)]
#![allow(dead_code)]

// use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Adjustment {}

impl Adjustment {
    pub fn new() -> Adjustment {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::adjustment_new()) }
        unimplemented!()
    }

    pub fn with_values(
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    ) -> Adjustment {
        // assert_initialized_main_thread!();
        // unsafe {
        //     from_glib_full(ffi::adjustment_new_with_values(
        //         value,
        //         lower,
        //         upper,
        //         step_increment,
        //         page_increment,
        //         page_size,
        //     ))
        // }
        unimplemented!()
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ADJUSTMENT: Option<&Adjustment> = None;

pub trait AdjustmentExt: 'static {
    fn get_clamp_value(&self) -> bool;

    fn get_elastic(&self) -> bool;

    fn get_lower(&self) -> f64;

    fn get_page_increment(&self) -> f64;

    fn get_page_size(&self) -> f64;

    fn get_step_increment(&self) -> f64;

    fn get_upper(&self) -> f64;

    fn get_value(&self) -> f64;

    fn get_values(&self) -> (f64, f64, f64, f64, f64, f64);

    // fn interpolate(&self, value: f64, duration: u32, mode: libc::c_ulong);

    // fn interpolate_relative(&self, offset: f64, duration: u32, mode: libc::c_ulong);

    fn set_clamp_value(&self, clamp: bool);

    fn set_elastic(&self, elastic: bool);

    fn set_lower(&self, lower: f64);

    fn set_page_increment(&self, increment: f64);

    fn set_page_size(&self, page_size: f64);

    fn set_step_increment(&self, increment: f64);

    fn set_upper(&self, upper: f64);

    fn set_value(&self, value: f64);

    fn set_values(
        &self,
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    );

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_changed_immediate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_interpolation_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clamp_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_elastic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<Adjustment>> AdjustmentExt for O {
//     fn get_clamp_value(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::adjustment_get_clamp_value(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_elastic(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::adjustment_get_elastic(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_lower(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_lower(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_page_increment(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_page_increment(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_page_size(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_page_size(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_step_increment(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_step_increment(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_upper(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_upper(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_value(&self) -> f64 {
//         // unsafe { ffi::adjustment_get_value(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_values(&self) -> (f64, f64, f64, f64, f64, f64) {
//         // unsafe {
//         //     let mut value = mem::MaybeUninit::uninit();
//         //     let mut lower = mem::MaybeUninit::uninit();
//         //     let mut upper = mem::MaybeUninit::uninit();
//         //     let mut step_increment = mem::MaybeUninit::uninit();
//         //     let mut page_increment = mem::MaybeUninit::uninit();
//         //     let mut page_size = mem::MaybeUninit::uninit();
//         //     ffi::adjustment_get_values(
//         //         self.as_ref().to_glib_none().0,
//         //         value.as_mut_ptr(),
//         //         lower.as_mut_ptr(),
//         //         upper.as_mut_ptr(),
//         //         step_increment.as_mut_ptr(),
//         //         page_increment.as_mut_ptr(),
//         //         page_size.as_mut_ptr(),
//         //     );
//         //     let value = value.assume_init();
//         //     let lower = lower.assume_init();
//         //     let upper = upper.assume_init();
//         //     let step_increment = step_increment.assume_init();
//         //     let page_increment = page_increment.assume_init();
//         //     let page_size = page_size.assume_init();
//         //     (
//         //         value,
//         //         lower,
//         //         upper,
//         //         step_increment,
//         //         page_increment,
//         //         page_size,
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn interpolate(&self, value: f64, duration: u32, mode: libc::c_ulong) {
//         // unsafe {
//         //     ffi::adjustment_interpolate(
//         //         self.as_ref().to_glib_none().0,
//         //         value,
//         //         duration,
//         //         mode,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn interpolate_relative(&self, offset: f64, duration: u32, mode: libc::c_ulong) {
//         // unsafe {
//         //     ffi::adjustment_interpolate_relative(
//         //         self.as_ref().to_glib_none().0,
//         //         offset,
//         //         duration,
//         //         mode,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_clamp_value(&self, clamp: bool) {
//         // unsafe {
//         //     ffi::adjustment_set_clamp_value(self.as_ref().to_glib_none().0, clamp.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_elastic(&self, elastic: bool) {
//         // unsafe {
//         //     ffi::adjustment_set_elastic(self.as_ref().to_glib_none().0, elastic.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_lower(&self, lower: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_lower(self.as_ref().to_glib_none().0, lower);
//         // }
//         unimplemented!()
//     }

//     fn set_page_increment(&self, increment: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_page_increment(self.as_ref().to_glib_none().0, increment);
//         // }
//         unimplemented!()
//     }

//     fn set_page_size(&self, page_size: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_page_size(self.as_ref().to_glib_none().0, page_size);
//         // }
//         unimplemented!()
//     }

//     fn set_step_increment(&self, increment: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_step_increment(self.as_ref().to_glib_none().0, increment);
//         // }
//         unimplemented!()
//     }

//     fn set_upper(&self, upper: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_upper(self.as_ref().to_glib_none().0, upper);
//         // }
//         unimplemented!()
//     }

//     fn set_value(&self, value: f64) {
//         // unsafe {
//         //     ffi::adjustment_set_value(self.as_ref().to_glib_none().0, value);
//         // }
//         unimplemented!()
//     }

//     fn set_values(
//         &self,
//         value: f64,
//         lower: f64,
//         upper: f64,
//         step_increment: f64,
//         page_increment: f64,
//         page_size: f64,
//     ) {
//         // unsafe {
//         //     ffi::adjustment_set_values(
//         //         self.as_ref().to_glib_none().0,
//         //         value,
//         //         lower,
//         //         upper,
//         //         step_increment,
//         //         page_increment,
//         //         page_size,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"changed\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             changed_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_changed_immediate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn changed_immediate_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"changed-immediate\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             changed_immediate_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_interpolation_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn interpolation_completed_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"interpolation-completed\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             interpolation_completed_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_clamp_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_clamp_value_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::clamp-value\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_clamp_value_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_elastic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_elastic_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::elastic\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_elastic_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_lower_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::lower\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_lower_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_page_increment_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::page-increment\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_page_increment_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_page_size_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::page-size\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_page_size_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_step_increment_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::step-increment\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_step_increment_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_upper_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::upper\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_upper_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Adjustment,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Adjustment>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::value\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_value_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Adjustment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adjustment")
    }
}
