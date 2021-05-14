#![allow(unused_variables)]

use crate::prelude::*;
use crate::{ActorMeta, Effect, HandlerId, OffscreenEffect};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct FadeEffectProps {
    pub x: i32,
    pub y: i32,
    pub bounds_width: u32,
    pub bounds_height: u32,

    pub border: [u32; 4],
    pub color: Color,
    pub width: f32,
    pub height: f32,

    pub vbo: dx::Handle,
    pub indices: dx::Handle,
    pub n_quads: u32,
    pub old_material: dx::Material,
    pub blocked_id: u64,

    pub x_offset: f32,
    pub y_offset: f32,

    pub update_vbo: bool,
    pub freeze_update: bool,
}

#[derive(Debug)]
pub struct FadeEffect {
    props: RefCell<FadeEffectProps>,
}

impl FadeEffect {
    pub fn new() -> FadeEffect {
        // assert_initialized_main_thread!();
        // unsafe { Effect::from_glib_none(ffi::fade_effect_new()).unsafe_cast() }
        unimplemented!()
    }
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

impl Is<OffscreenEffect> for FadeEffect {}

impl AsRef<OffscreenEffect> for FadeEffect {
    fn as_ref(&self) -> &OffscreenEffect {
        // &self.widget
        unimplemented!()
    }
}

impl Is<Effect> for FadeEffect {}

impl AsRef<Effect> for FadeEffect {
    fn as_ref(&self) -> &Effect {
        // &self.widget
        unimplemented!()
    }
}

impl Is<ActorMeta> for FadeEffect {}

impl AsRef<ActorMeta> for FadeEffect {
    fn as_ref(&self) -> &ActorMeta {
        // &self.widget
        unimplemented!()
    }
}

pub trait FadeEffectExt: 'static {
    /// get_border:
    /// @effect: A #FadeEffect
    /// @top: (out) (allow-none): The upper border, in pixels
    /// @right: (out) (allow-none): The right border, in pixels
    /// @bottom: (out) (allow-none): The lower border, in pixels
    /// @left: (out) (allow-none): The left border, in pixels
    ///
    /// Retrieves the border values for @effect.
    ///
    fn get_border(&self) -> (u32, u32, u32, u32);

    /// get_bounds:
    /// @effect: A #FadeEffect
    /// @x: (out) (allow-none): The x value of the effect bounds, in pixels
    /// @y: (out) (allow-none): The y value of the effect bounds, in pixels
    /// @width: (out) (allow-none): The width of the effect bounds, in pixels, or %0
    /// @height: (out) (allow-none): The height of the effect bounds, in pixels, or %0
    ///
    /// Retrieves the bounding box of the effect.
    ///
    fn get_bounds(&self) -> (i32, i32, u32, u32);

    /// get_color:
    /// @effect: A #FadeEffect
    /// @color: (out): A #Color to store the color in
    ///
    /// Retrieves the color used for the fade effect.
    ///
    fn get_color(&self) -> Color;

    /// set_border:
    /// @effect: A #FadeEffect
    /// @top: The upper border, in pixels
    /// @right: The right border, in pixels
    /// @bottom: The lower border, in pixels
    /// @left: The left border, in pixels
    ///
    /// Sets the border to be used for the fading effect. This is the number of
    /// pixels on each side of the effect that should be used to fade.
    ///
    fn set_border(&self, top: u32, right: u32, bottom: u32, left: u32);

    /// set_bounds:
    /// @effect: A #FadeEffect
    /// @x: The x value of the effect bounds, in pixels
    /// @y: The y value of the effect bounds, in pixels
    /// @width: The width of the effect bounds, in pixels, or %0
    /// @height: The height of the effect bounds, in pixels, or %0
    ///
    /// Sets the bounding box of the effect. The effect will essentially treat
    /// this box as a clipping rectangle. Setting width or height to %0 will
    /// use the width or height of the #Actor the effect is attached to.
    ///
    /// <note><para>
    /// The effect border will apply to the bounds, and not to the un-altered
    /// rectangle, so an effect with an %x of %5 and a %left-border of %5 will
    /// have a gap of 5 blank pixels to the left, with a fade length of 5 pixels.
    /// </para></note>
    ///
    fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32);

    /// set_color:
    /// @effect: A #FadeEffect
    /// @color: A #Color
    ///
    /// Sets the color of the fade effect. The effect will fade out towards
    /// the set border to this color.
    ///
    fn set_color(&self, color: &Color);

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

    fn connect_property_border_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_border_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_border_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_border_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_bounds_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_bounds_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_bounds_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_bounds_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_freeze_update_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<FadeEffect>> FadeEffectExt for O {
    /// get_border:
    /// @effect: A #FadeEffect
    /// @top: (out) (allow-none): The upper border, in pixels
    /// @right: (out) (allow-none): The right border, in pixels
    /// @bottom: (out) (allow-none): The lower border, in pixels
    /// @left: (out) (allow-none): The left border, in pixels
    ///
    /// Retrieves the border values for @effect.
    ///
    fn get_border(&self) -> (u32, u32, u32, u32) {
        let fadeeffect = self.as_ref();
        let props = fadeeffect.props.borrow();

        let top = props.border[0];
        let right = props.border[1];
        let bottom = props.border[2];
        let left = props.border[3];

        (top, right, bottom, left)
    }

    /// get_bounds:
    /// @effect: A #FadeEffect
    /// @x: (out) (allow-none): The x value of the effect bounds, in pixels
    /// @y: (out) (allow-none): The y value of the effect bounds, in pixels
    /// @width: (out) (allow-none): The width of the effect bounds, in pixels, or %0
    /// @height: (out) (allow-none): The height of the effect bounds, in pixels, or %0
    ///
    /// Retrieves the bounding box of the effect.
    ///
    fn get_bounds(&self) -> (i32, i32, u32, u32) {
        let fadeeffect = self.as_ref();
        let props = fadeeffect.props.borrow();

        let x = props.x;
        let y = props.y;
        let width = props.bounds_width;
        let height = props.bounds_height;

        (x, y, width, height)
    }

    /// get_color:
    /// @effect: A #FadeEffect
    /// @color: (out): A #Color to store the color in
    ///
    /// Retrieves the color used for the fade effect.
    ///
    fn get_color(&self) -> Color {
        let fadeeffect = self.as_ref();
        let props = fadeeffect.props.borrow();

        props.color
    }

    /// set_border:
    /// @effect: A #FadeEffect
    /// @top: The upper border, in pixels
    /// @right: The right border, in pixels
    /// @bottom: The lower border, in pixels
    /// @left: The left border, in pixels
    ///
    /// Sets the border to be used for the fading effect. This is the number of
    /// pixels on each side of the effect that should be used to fade.
    ///
    fn set_border(&self, top: u32, right: u32, bottom: u32, left: u32) {
        let fadeeffect = self.as_ref();
        let mut props = fadeeffect.props.borrow_mut();

        // g_object_freeze_notify(G_OBJECT(effect));

        if props.border[0] != top {
            props.border[0] = top;
            //     g_object_notify(G_OBJECT(effect), "border-top");
        }

        if props.border[1] != right {
            props.border[1] = right;
            //     g_object_notify(G_OBJECT(effect), "border-right");
        }

        if props.border[2] != bottom {
            props.border[2] = bottom;
            //     g_object_notify(G_OBJECT(effect), "border-bottom");
        }

        if props.border[3] != left {
            props.border[3] = left;
            //     g_object_notify(G_OBJECT(effect), "border-left");
        }

        props.update_vbo = true;

        // g_object_thaw_notify(G_OBJECT(effect));
    }

    /// set_bounds:
    /// @effect: A #FadeEffect
    /// @x: The x value of the effect bounds, in pixels
    /// @y: The y value of the effect bounds, in pixels
    /// @width: The width of the effect bounds, in pixels, or %0
    /// @height: The height of the effect bounds, in pixels, or %0
    ///
    /// Sets the bounding box of the effect. The effect will essentially treat
    /// this box as a clipping rectangle. Setting width or height to %0 will
    /// use the width or height of the #Actor the effect is attached to.
    ///
    /// <note><para>
    /// The effect border will apply to the bounds, and not to the un-altered
    /// rectangle, so an effect with an %x of %5 and a %left-border of %5 will
    /// have a gap of 5 blank pixels to the left, with a fade length of 5 pixels.
    /// </para></note>
    ///
    fn set_bounds(&self, x: i32, y: i32, width: u32, height: u32) {
        let fadeeffect = self.as_ref();
        let mut props = fadeeffect.props.borrow_mut();

        // g_object_freeze_notify(G_OBJECT(effect));

        if props.x != x {
            props.x = x;
            //     g_object_notify(G_OBJECT(effect), "bounds-x");
        }

        if props.y != y {
            props.y = y;
            //     g_object_notify(G_OBJECT(effect), "bounds-y");
        }

        if props.bounds_width != width {
            props.bounds_width = width;
            //     g_object_notify(G_OBJECT(effect), "bounds-width");
        }

        if props.bounds_height != height {
            props.bounds_height = height;
            //     g_object_notify(G_OBJECT(effect), "bounds-height");
        }

        props.update_vbo = true;

        // g_object_thaw_notify(G_OBJECT(effect));
    }

    /// set_color:
    /// @effect: A #FadeEffect
    /// @color: A #Color
    ///
    /// Sets the color of the fade effect. The effect will fade out towards
    /// the set border to this color.
    ///
    fn set_color(&self, color: &Color) {
        let fadeeffect = self.as_ref();
        // let mut props = fadeeffect.props.borrow_mut();

        // if !color_equal(&fadeeffect.color, color) {
        //     props.color = color;
        //     props.update_vbo = true;
        //     g_object_notify(G_OBJECT(effect), "color");
        // }
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

    fn connect_property_border_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_border_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_border_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_border_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_bounds_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_bounds_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_bounds_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_bounds_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_freeze_update_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
