#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Adjustment, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

const ERROR_MARGIN: f32 = f32::EPSILON;

#[derive(Clone, Debug)]
pub struct ViewportProps {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hadjustment: Option<Adjustment>,
    pub vadjustment: Option<Adjustment>,
    pub sync_adjustments: bool,
    pub child: Option<Actor>,
}

#[derive(Clone, Debug)]
pub struct Viewport {
    props: RefCell<ViewportProps>,
    widget: Widget,
}

impl Viewport {
    pub fn new() -> Viewport {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::viewport_new()).unsafe_cast() }
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

impl Is<Actor> for Viewport {}

impl AsRef<Actor> for Viewport {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

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
        let props = viewport.props.borrow();

        (props.x, props.y, props.z)
    }

    fn get_sync_adjustments(&self) -> bool {
        let viewport = self.as_ref();
        let props = viewport.props.borrow();

        props.sync_adjustments
    }

    fn set_origin(&self, x: f32, y: f32, z: f32) {
        let viewport = self.as_ref();
        let mut props = viewport.props.borrow_mut();

        // g_object_freeze_notify(G_OBJECT(viewport));

        if (x - props.x).abs() > ERROR_MARGIN {
            props.x = x;
            // g_object_notify(G_OBJECT(viewport), "x-origin");

            if props.hadjustment.is_some() {
                // adjustment_set_value(viewport.hadjustment, (float)(x));
            }
        }

        if (y - props.y).abs() > ERROR_MARGIN {
            props.y = y;
            // g_object_notify(G_OBJECT(viewport), "y-origin");

            if props.vadjustment.is_some() {
                // adjustment_set_value(viewport.vadjustment, (float)(y));
            }
        }

        if (z - props.z).abs() > ERROR_MARGIN {
            props.z = z;
            // g_object_notify(G_OBJECT(viewport), "z-origin");
        }

        // g_object_thaw_notify(G_OBJECT(viewport));
        // clutter_actor_queue_redraw(CLUTTER_ACTOR(viewport));
    }

    fn set_sync_adjustments(&self, sync_adjustments: bool) {
        let viewport = self.as_ref();
        let mut props = viewport.props.borrow_mut();

        if props.sync_adjustments != sync_adjustments {
            props.sync_adjustments = sync_adjustments;
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
