#![allow(unused_variables)]

// use std::mem::transmute;
use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct ExpanderProps {
    pub label: Option<clutter::Actor>,
    pub arrow: Option<clutter::Actor>,
    pub spacing: f64,
    pub timeline: Option<clutter::Timeline>,
    pub progress: u64,
    pub expanded: bool,
    pub child: Option<clutter::Actor>,
}

#[derive(Clone, Debug)]
pub struct Expander {
    props: RefCell<ExpanderProps>,
    widget: Widget,
}

impl Expander {
    pub fn new() -> Expander {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::expander_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Expander {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Expander {}
impl Is<Expander> for Expander {}

impl AsRef<Expander> for Expander {
    fn as_ref(&self) -> &Expander {
        self
    }
}

impl Is<Widget> for Expander {}

impl AsRef<Widget> for Expander {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Expander {}

impl AsRef<clutter::Actor> for Expander {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

pub trait ExpanderExt: 'static {
    /// get_expanded:
    /// @expander: a #Expander
    ///
    /// Get the current state of the expander (the value of #Expander:expanded)
    ///
    /// Returns: #true if the expander is open, #false if it is closed
    ///
    fn get_expanded(&self) -> bool;

    /// set_expanded:
    /// @expander: A #Expander
    /// @expanded: the state of the expander to set
    ///
    /// Set the state (the #Expander:expanded property) of the expander.
    /// This will cause the expander to open or close.
    ///
    fn set_expanded(&self, expanded: bool);

    /// set_label:
    /// @expander: A #Expander
    /// @label: string to set as the expander label
    ///
    /// Sets the text displayed as the title of the expander
    ///
    fn set_label(&self, label: &str);

    fn get_property_label(&self) -> Option<String>;

    fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Expander>> ExpanderExt for O {
    /// get_expanded:
    /// @expander: a #Expander
    ///
    /// Get the current state of the expander (the value of #Expander:expanded)
    ///
    /// Returns: #true if the expander is open, #false if it is closed
    ///
    fn get_expanded(&self) -> bool {
        let expander = self.as_ref();
        let props = expander.props.borrow();

        props.expanded
    }

    /// set_expanded:
    /// @expander: A #Expander
    /// @expanded: the state of the expander to set
    ///
    /// Set the state (the #Expander:expanded property) of the expander.
    /// This will cause the expander to open or close.
    ///
    fn set_expanded(&self, expanded: bool) {
        let expander = self.as_ref();
        let mut props = expander.props.borrow_mut();

        if props.expanded != expanded {
            props.expanded = expanded;

            // expander.update();
            // g_object_notify (G_OBJECT (expander), "expanded");
        }
    }

    /// set_label:
    /// @expander: A #Expander
    /// @label: string to set as the expander label
    ///
    /// Sets the text displayed as the title of the expander
    ///
    fn set_label(&self, label: &str) {
        let expander = self.as_ref();
        // clutter_text_set_text (CLUTTER_TEXT (expander.label), label);
    }

    fn get_property_label(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"label\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `label` getter")
        // }
        unimplemented!()
    }

    fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn expand_complete_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Expander,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Expander>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"expand-complete\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             expand_complete_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Expander,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Expander>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::expanded\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_expanded_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Expander,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Expander>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::label\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_label_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expander")
    }
}
