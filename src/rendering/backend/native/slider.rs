#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Widget};
use std::{cell::RefCell, fmt};

#[derive(Default, Debug, Clone)]
pub struct SliderProps {
    pub trough_bg: Option<Actor>,
    pub fill: Option<Actor>,
    pub trough: Option<Actor>,
    pub handle: Option<Actor>,
    pub buffer: Option<Actor>,

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

#[derive(Debug)]
pub struct Slider {
    props: RefCell<SliderProps>,
    inner: Widget,
}

impl Slider {
    pub fn new() -> Slider {
        let props = SliderProps::default();

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    fn init(&self) {}
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

impl Is<Widget> for Slider {}

impl AsRef<Widget> for Slider {
    fn as_ref(&self) -> &Widget {
        &self.inner
    }
}

impl Is<Actor> for Slider {}

impl AsRef<Actor> for Slider {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.inner.as_ref();
        actor
    }
}

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

    fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
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
        let props = slider.props.borrow();

        props.buffer_value
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
        let props = slider.props.borrow();

        props.value
    }

    /// set_buffer_value:
    /// @slider: A #Slider
    /// @value: the new buffer value of the slider
    ///
    /// Set the value of the #Slider:buffer-value property.
    ///
    fn set_buffer_value(&self, value: f64) {
        let slider = self.as_ref();
        let mut props = slider.props.borrow_mut();

        if props.buffer_value == value {
            return;
        }

        props.buffer_value = value;
        // actor_queue_relayout(CLUTTER_ACTOR(slider));
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
        let mut props = slider.props.borrow_mut();

        if props.value == value {
            return;
        }

        // if G_UNLIKELY((value < 0.0) || (value > 1.0)) {
        //     g_warning("Slider:value must be a number between 0.0 and 1.0");
        //     return;
        // }

        props.value = value;

        // if !props.capture_handler {
        //     slider_allocate_fill_handle(slider, None, 0);
        //     actor_queue_redraw(CLUTTER_ACTOR(slider));
        // }

        // g_object_notify(G_OBJECT(slider), "value");
    }

    fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"slide-start\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             slide_start_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"slide-stop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             slide_stop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::buffer-value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_buffer_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
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
