#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Viewport {}

impl Viewport {
    pub fn new() -> Viewport {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::viewport_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Viewport {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::viewport_new()) }
    // }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Viewport {}
impl Is<Viewport> for Viewport {}

impl AsRef<Viewport> for Viewport {
    fn as_ref(&self) -> &Viewport {
        self
    }
}

pub const NONE_VIEWPORT: Option<&Viewport> = None;

pub trait ViewportExt: 'static {
    fn get_origin(&self) -> (f32, f32, f32);

    fn get_sync_adjustments(&self) -> bool;

    fn set_origin(&self, x: f32, y: f32, z: f32);

    fn set_sync_adjustments(&self, sync: bool);

    fn get_property_x_origin(&self) -> f32;

    fn set_property_x_origin(&self, x_origin: f32);

    fn get_property_y_origin(&self) -> f32;

    fn set_property_y_origin(&self, y_origin: f32);

    fn get_property_z_origin(&self) -> f32;

    fn set_property_z_origin(&self, z_origin: f32);

    fn connect_property_sync_adjustments_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_x_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_z_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Viewport>> ViewportExt for O {
    fn get_origin(&self) -> (f32, f32, f32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     let mut z = mem::MaybeUninit::uninit();
        //     ffi::viewport_get_origin(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //         z.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     let z = z.assume_init();
        //     (x, y, z)
        // }
        unimplemented!()
    }

    fn get_sync_adjustments(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::viewport_get_sync_adjustments(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_origin(&self, x: f32, y: f32, z: f32) {
        // unsafe {
        //     ffi::viewport_set_origin(self.as_ref().to_glib_none().0, x, y, z);
        // }
        unimplemented!()
    }

    fn set_sync_adjustments(&self, sync: bool) {
        // unsafe {
        //     ffi::viewport_set_sync_adjustments(
        //         self.as_ref().to_glib_none().0,
        //         sync.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn get_property_x_origin(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-origin\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `x-origin` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_x_origin(&self, x_origin: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-origin\0".as_ptr() as *const _,
        //         Value::from(&x_origin).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_y_origin(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-origin\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `y-origin` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_y_origin(&self, y_origin: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-origin\0".as_ptr() as *const _,
        //         Value::from(&y_origin).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_z_origin(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"z-origin\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `z-origin` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_z_origin(&self, z_origin: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"z-origin\0".as_ptr() as *const _,
        //         Value::from(&z_origin).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_sync_adjustments_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_sync_adjustments_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Viewport,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Viewport>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Viewport::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::sync-adjustments\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_sync_adjustments_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_x_origin_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Viewport,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Viewport>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Viewport::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-origin\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_origin_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_y_origin_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Viewport,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Viewport>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Viewport::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-origin\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_origin_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_z_origin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_z_origin_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Viewport,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Viewport>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Viewport::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::z-origin\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_z_origin_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Viewport")
    }
}
