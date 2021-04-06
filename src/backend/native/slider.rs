#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Slider {
    pub trough_bg: Option<clutter::Actor>,
    pub fill: Option<clutter::Actor>,
    pub trough: Option<clutter::Actor>,
    pub handle: Option<clutter::Actor>,
    pub buffer: Option<clutter::Actor>,

    pub capture_handler: u64,
    pub x_origin: f32,

    // the middle of the handle can wander on the axis between start and end
    pub handle_middle_start: f32,
    pub handle_middle_end: f32,

    // keep those around for ::alocate_fill() */
    pub trough_box_y1: f32,
    pub trough_box_y2: f32,
    pub trough_height: i32,
    pub handle_width: u32,
    pub handle_height: u32,

    pub value: f64,
    pub buffer_value: f64,
}

impl Slider {
    pub fn new() -> Slider {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::slider_new()).unsafe_cast() }
        unimplemented!()
    }
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
    /// get_buffer_value:
    /// @slider: A #Slider
    ///
    /// Get the value of the #Slider:buffer-value property.
    ///
    /// Returns: The current value of the "buffer-value" property.
    ///
    fn get_buffer_value(&self) -> f64;

    /// get_value:
    /// @bar: A #Slider
    ///
    /// Retrieve the current value of the media bar
    ///
    /// Returns: gdouble
    ///
    fn get_value(&self) -> f64;

    /// set_buffer_value:
    /// @slider: A #Slider
    /// @value: the new buffer value of the slider
    ///
    /// Set the value of the #Slider:buffer-value property.
    ///
    fn set_buffer_value(&self, value: f64);

    /// set_value:
    /// @bar: A #Slider
    /// @value: A value between 0.0 and 1.0
    ///
    /// Set the value of the slider
    ///
    fn set_value(&self, value: f64);

    fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Slider>> SliderExt for O {
    /// get_buffer_value:
    /// @slider: A #Slider
    ///
    /// Get the value of the #Slider:buffer-value property.
    ///
    /// Returns: The current value of the "buffer-value" property.
    ///
    fn get_buffer_value(&self) -> f64 {
        let slider = self.as_ref();
        slider.buffer_value
    }

    /// get_value:
    /// @bar: A #Slider
    ///
    /// Retrieve the current value of the media bar
    ///
    /// Returns: gdouble
    ///
    fn get_value(&self) -> f64 {
        let slider = self.as_ref();
        slider.value
    }

    /// set_buffer_value:
    /// @slider: A #Slider
    /// @value: the new buffer value of the slider
    ///
    /// Set the value of the #Slider:buffer-value property.
    ///
    fn set_buffer_value(&self, value: f64) {
        let slider = self.as_ref();

        // if slider.buffer_value == value {
        //     return;
        // }

        // slider.buffer_value = value;
        // clutter_actor_queue_relayout(CLUTTER_ACTOR(slider));
        // g_object_notify(G_OBJECT(slider), "buffer-value");
    }

    /// set_value:
    /// @bar: A #Slider
    /// @value: A value between 0.0 and 1.0
    ///
    /// Set the value of the slider
    ///
    fn set_value(&self, value: f64) {
        let slider = self.as_ref();

        if slider.value == value {
            return;
        }

        // if G_UNLIKELY((value < 0.0) || (value > 1.0)) {
        //     g_warning("Slider:value must be a number between 0.0 and 1.0");
        //     return;
        // }

        // slider.value = value;

        // if !slider.capture_handler {
        //     slider_allocate_fill_handle(slider, NULL, 0);
        //     clutter_actor_queue_redraw(CLUTTER_ACTOR(slider));
        // }

        // g_object_notify(G_OBJECT(slider), "value");
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
