#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends clutter::OffscreenEffect, clutter::Effect, clutter::ActorMeta;
#[derive(Clone, Debug)]
pub struct FadeEffect {}

impl FadeEffect {
    pub fn new() -> FadeEffect {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Effect::from_glib_none(ffi::fade_effect_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> FadeEffect {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::fade_effect_new()) }
    // }
}

impl Default for FadeEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for FadeEffect {}
impl Is<FadeEffect> for FadeEffect {}

impl AsRef<FadeEffect> for FadeEffect {
    fn as_ref(&self) -> &FadeEffect {
        self
    }
}

pub const NONE_FADE_EFFECT: Option<&FadeEffect> = None;

pub trait FadeEffectExt: 'static {
    fn get_border(&self) -> (u32, u32, u32, u32);

    fn get_bounds(&self) -> (i32, i32, u32, u32);

    fn get_color(&self) -> clutter::Color;

    fn set_border(&self, top: u32, right: u32, bottom: u32, left: u32);

    fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32);

    fn set_color(&self, color: &clutter::Color);

    fn get_property_border_bottom(&self) -> u32;

    fn set_property_border_bottom(&self, border_bottom: u32);

    fn get_property_border_left(&self) -> u32;

    fn set_property_border_left(&self, border_left: u32);

    fn get_property_border_right(&self) -> u32;

    fn set_property_border_right(&self, border_right: u32);

    fn get_property_border_top(&self) -> u32;

    fn set_property_border_top(&self, border_top: u32);

    fn get_property_bounds_height(&self) -> u32;

    fn set_property_bounds_height(&self, bounds_height: u32);

    fn get_property_bounds_width(&self) -> u32;

    fn set_property_bounds_width(&self, bounds_width: u32);

    fn get_property_bounds_x(&self) -> i32;

    fn set_property_bounds_x(&self, bounds_x: i32);

    fn get_property_bounds_y(&self) -> i32;

    fn set_property_bounds_y(&self, bounds_y: i32);

    fn get_property_freeze_update(&self) -> bool;

    fn set_property_freeze_update(&self, freeze_update: bool);

    fn connect_property_border_bottom_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_border_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_border_right_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_border_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bounds_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_bounds_width_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_bounds_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bounds_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_freeze_update_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<FadeEffect>> FadeEffectExt for O {
    fn get_border(&self) -> (u32, u32, u32, u32) {
        // unsafe {
        //     let mut top = mem::MaybeUninit::uninit();
        //     let mut right = mem::MaybeUninit::uninit();
        //     let mut bottom = mem::MaybeUninit::uninit();
        //     let mut left = mem::MaybeUninit::uninit();
        //     ffi::fade_effect_get_border(
        //         self.as_ref().to_glib_none().0,
        //         top.as_mut_ptr(),
        //         right.as_mut_ptr(),
        //         bottom.as_mut_ptr(),
        //         left.as_mut_ptr(),
        //     );
        //     let top = top.assume_init();
        //     let right = right.assume_init();
        //     let bottom = bottom.assume_init();
        //     let left = left.assume_init();
        //     (top, right, bottom, left)
        // }
        unimplemented!()
    }

    fn get_bounds(&self) -> (i32, i32, u32, u32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     let mut width = mem::MaybeUninit::uninit();
        //     let mut height = mem::MaybeUninit::uninit();
        //     ffi::fade_effect_get_bounds(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //         width.as_mut_ptr(),
        //         height.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     let width = width.assume_init();
        //     let height = height.assume_init();
        //     (x, y, width, height)
        // }
        unimplemented!()
    }

    fn get_color(&self) -> clutter::Color {
        // unsafe {
        //     let mut color = clutter::Color::uninitialized();
        //     ffi::fade_effect_get_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none_mut().0,
        //     );
        //     color
        // }
        unimplemented!()
    }

    fn set_border(&self, top: u32, right: u32, bottom: u32, left: u32) {
        // unsafe {
        //     ffi::fade_effect_set_border(
        //         self.as_ref().to_glib_none().0,
        //         top,
        //         right,
        //         bottom,
        //         left,
        //     );
        // }
        unimplemented!()
    }

    fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32) {
        // unsafe {
        //     ffi::fade_effect_set_bounds(self.as_ref().to_glib_none().0, x, y, width, height);
        // }
        unimplemented!()
    }

    fn set_color(&self, color: &clutter::Color) {
        // unsafe {
        //     ffi::fade_effect_set_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_border_bottom(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-bottom\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-bottom` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_border_bottom(&self, border_bottom: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-bottom\0".as_ptr() as *const _,
        //         Value::from(&border_bottom).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_border_left(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-left\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-left` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_border_left(&self, border_left: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-left\0".as_ptr() as *const _,
        //         Value::from(&border_left).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_border_right(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-right\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-right` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_border_right(&self, border_right: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-right\0".as_ptr() as *const _,
        //         Value::from(&border_right).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_border_top(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-top\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-top` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_border_top(&self, border_top: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-top\0".as_ptr() as *const _,
        //         Value::from(&border_top).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_bounds_height(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-height\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `bounds-height` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_bounds_height(&self, bounds_height: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-height\0".as_ptr() as *const _,
        //         Value::from(&bounds_height).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_bounds_width(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `bounds-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_bounds_width(&self, bounds_width: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-width\0".as_ptr() as *const _,
        //         Value::from(&bounds_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_bounds_x(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `bounds-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_bounds_x(&self, bounds_x: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-x\0".as_ptr() as *const _,
        //         Value::from(&bounds_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_bounds_y(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `bounds-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_bounds_y(&self, bounds_y: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"bounds-y\0".as_ptr() as *const _,
        //         Value::from(&bounds_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_freeze_update(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"freeze-update\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `freeze-update` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_freeze_update(&self, freeze_update: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"freeze-update\0".as_ptr() as *const _,
        //         Value::from(&freeze_update).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_border_bottom_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_border_bottom_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::border-bottom\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_border_bottom_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_border_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_border_left_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::border-left\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_border_left_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_border_right_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_border_right_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::border-right\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_border_right_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_border_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_border_top_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::border-top\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_border_top_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_bounds_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_bounds_height_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::bounds-height\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_bounds_height_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_bounds_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_bounds_width_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::bounds-width\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_bounds_width_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_bounds_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_bounds_x_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::bounds-x\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_bounds_x_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_bounds_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_bounds_y_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::bounds-y\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_bounds_y_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_color_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::color\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_color_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_freeze_update_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_freeze_update_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FadeEffect,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FadeEffect>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FadeEffect::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::freeze-update\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_freeze_update_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for FadeEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FadeEffect")
    }
}
