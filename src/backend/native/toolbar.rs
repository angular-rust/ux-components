#![allow(unused_variables)]

// use std::mem::transmute;
use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Toolbar {
    pub has_close_button: bool,
    pub child_has_focus: bool,
    pub close_button: Option<clutter::Actor>,
    pub child: Option<clutter::Actor>,
    widget: Widget,
}

impl Toolbar {
    pub fn new() -> Toolbar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::toolbar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Toolbar {}
impl Is<Toolbar> for Toolbar {}

impl AsRef<Toolbar> for Toolbar {
    fn as_ref(&self) -> &Toolbar {
        self
    }
}

impl Is<Widget> for Toolbar {}

impl AsRef<Widget> for Toolbar {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Toolbar {}

impl AsRef<clutter::Actor> for Toolbar {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_TOOLBAR: Option<&Toolbar> = None;

pub trait ToolbarExt: 'static {
    /// set_has_close_button:
    /// @toolbar: A #Toolbar
    /// @has_close_button: #true if a close button should be displayed
    ///
    /// Set the #Toolbar:has-close-button property
    ///
    fn get_has_close_button(&self) -> bool;

    /// get_has_close_button:
    /// @toolbar: A #Toolbar
    ///
    /// Get the value of the #Toolbar:has-close-button property.
    ///
    /// Returns: the current value of the "hast-close-button" property.
    ///
    fn set_has_close_button(&self, has_close_button: bool);

    fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<Toolbar>> ToolbarExt for O {
    /// set_has_close_button:
    /// @toolbar: A #Toolbar
    /// @has_close_button: #true if a close button should be displayed
    ///
    /// Set the #Toolbar:has-close-button property
    ///
    fn get_has_close_button(&self) -> bool {
        let toolbar = self.as_ref();
        toolbar.has_close_button
    }

    /// get_has_close_button:
    /// @toolbar: A #Toolbar
    ///
    /// Get the value of the #Toolbar:has-close-button property.
    ///
    /// Returns: the current value of the "hast-close-button" property.
    ///
    fn set_has_close_button(&self, has_close_button: bool) {
        let toolbar = self.as_ref();

        if toolbar.has_close_button != has_close_button {
            // toolbar.has_close_button = has_close_button;

            // if !has_close_button {
            //     if toolbar.close_button {
            //         clutter_actor_destroy(toolbar.close_button);
            //         toolbar.close_button = None;
            //     }
            // } else {
            //     toolbar.close_button = button_new ();
            //     clutter_actor_set_name(toolbar.close_button, "close-button");
            //     clutter_actor_add_child(CLUTTER_ACTOR (toolbar), toolbar.close_button);
            //     g_signal_connect(toolbar.close_button, "clicked",
            //                         G_CALLBACK(close_button_click_cb), toolbar);
            //     stylable_style_changed(STYLABLE(toolbar.close_button),
            //                                 STYLE_CHANGED_FORCE);
            // }

            // clutter_actor_queue_relayout(CLUTTER_ACTOR(toolbar));
            // g_object_notify(G_OBJECT(toolbar), "has-close-button");
        }
    }

    fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn close_button_clicked_trampoline<P, F: Fn(&P) -> bool + 'static>(
        //     this: *mut ffi::Toolbar,
        //     f: glib_sys::gpointer,
        // ) -> glib_sys::gboolean
        // where
        //     P: Is<Toolbar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Toolbar::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"close-button-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             close_button_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_has_close_button_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Toolbar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Toolbar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Toolbar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::has-close-button\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_has_close_button_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Toolbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toolbar")
    }
}
