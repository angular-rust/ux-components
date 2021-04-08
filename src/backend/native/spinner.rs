#![allow(unused_variables)]

// use std::mem::transmute;
use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct SpinnerProps {
    // pub texture: cogl::Handle,
    // pub material: cogl::Handle,
    pub frames: u32,
    pub anim_duration: u32,
    pub current_frame: u32,
    pub update_id: u32,
    pub animating: bool,
}

#[derive(Clone, Debug)]
pub struct Spinner {
    props: RefCell<SpinnerProps>,
    widget: Widget,
}

impl Spinner {
    pub fn new() -> Spinner {
        let props = SpinnerProps {
            // texture: cogl::Handle,
            // material: cogl::Handle,
            frames: 0,
            anim_duration: 0,
            current_frame: 0,
            update_id: 0,
            animating: false,
        };

        Self{
            props: RefCell::new(props),
            widget: Widget::new(),
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Spinner {}
impl Is<Spinner> for Spinner {}

impl AsRef<Spinner> for Spinner {
    fn as_ref(&self) -> &Spinner {
        self
    }
}

impl Is<Widget> for Spinner {}

impl AsRef<Widget> for Spinner {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Spinner {}

impl AsRef<clutter::Actor> for Spinner {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_SPINNER: Option<&Spinner> = None;

pub trait SpinnerExt: 'static {
    /// get_animating:
    /// @spinner: A #Spinner widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %true if the spinner is animating, %FALSE otherwise
    ///
    fn get_animating(&self) -> bool;

    /// set_animating:
    /// @spinner: A #Spinner widget
    /// @animating: %true to enable animation, %FALSE to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool);

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Spinner>> SpinnerExt for O {
    /// get_animating:
    /// @spinner: A #Spinner widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %true if the spinner is animating, %FALSE otherwise
    ///
    fn get_animating(&self) -> bool {
        let spinner = self.as_ref();
        let props = spinner.props.borrow();
        props.animating
    }

    /// set_animating:
    /// @spinner: A #Spinner widget
    /// @animating: %true to enable animation, %FALSE to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool) {
        let spinner = self.as_ref();
        let mut props = spinner.props.borrow_mut();

        if props.animating != animating {
            props.animating = animating;
            // update_timeout(spinner);
            // g_object_notify(G_OBJECT(spinner), "animating");
        }
    }

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn looped_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Spinner,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Spinner>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Spinner::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"looped\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             looped_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_animating_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Spinner,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Spinner>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Spinner::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::animating\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_animating_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spinner")
    }
}
