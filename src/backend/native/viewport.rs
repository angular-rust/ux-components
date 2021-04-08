#![allow(unused_variables)]

// use std::mem;
// use std::mem::transmute;
use super::{Adjustment, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hadjustment: Adjustment,
    pub vadjustment: Adjustment,
    pub sync_adjustments: bool,
    pub child: Option<clutter::Actor>,
    widget: Widget,
}

impl Viewport {
    pub fn new() -> Viewport {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::viewport_new()).unsafe_cast() }
        unimplemented!()
    }
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

impl Is<Widget> for Viewport {}

impl AsRef<Widget> for Viewport {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Viewport {}

impl AsRef<clutter::Actor> for Viewport {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_VIEWPORT: Option<&Viewport> = None;

pub trait ViewportExt: 'static {
    fn get_origin(&self) -> (f32, f32, f32);

    fn get_sync_adjustments(&self) -> bool;

    fn set_origin(&self, x: f32, y: f32, z: f32);

    fn set_sync_adjustments(&self, sync_adjustments: bool);

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
        let viewport = self.as_ref();
        (viewport.x, viewport.y, viewport.z)
    }

    fn get_sync_adjustments(&self) -> bool {
        let viewport = self.as_ref();
        viewport.sync_adjustments
    }

    fn set_origin(&self, x: f32, y: f32, z: f32) {
        let viewport = self.as_ref();

        // g_object_freeze_notify(G_OBJECT(viewport));

        // if x != viewport.x {
        //     viewport.x = x;
        //     g_object_notify(G_OBJECT(viewport), "x-origin");

        //     if viewport.hadjustment {
        //         mx_adjustment_set_value(viewport.hadjustment, (float)(x));
        //     }
        // }

        // if y != viewport.y {
        //     viewport.y = y;
        //     g_object_notify(G_OBJECT(viewport), "y-origin");

        //     if viewport.vadjustment {
        //         mx_adjustment_set_value(viewport.vadjustment, (float)(y));
        //     }
        // }

        // if z != viewport.z {
        //     viewport.z = z;
        //     g_object_notify(G_OBJECT(viewport), "z-origin");
        // }

        // g_object_thaw_notify(G_OBJECT(viewport));
        // clutter_actor_queue_redraw(CLUTTER_ACTOR(viewport));
    }

    fn set_sync_adjustments(&self, sync_adjustments: bool) {
        let viewport = self.as_ref();

        if viewport.sync_adjustments != sync_adjustments {
            // viewport.sync_adjustments = sync_adjustments;
            // g_object_notify(G_OBJECT(viewport), "sync-adjustments");
        }
    }

    fn get_property_x_origin(&self) -> f32 {
        let viewport = self.as_ref();
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
        let viewport = self.as_ref();
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
        let viewport = self.as_ref();
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
        let viewport = self.as_ref();
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
        let viewport = self.as_ref();
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
        let viewport = self.as_ref();
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
