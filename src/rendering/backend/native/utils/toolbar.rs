#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Widget};
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct ToolbarProps {
    pub has_close_button: bool,
    pub child_has_focus: bool,
    pub close_button: Option<Actor>,
    pub child: Option<Actor>,
}

#[derive(Debug)]
pub struct Toolbar {
    props: RefCell<ToolbarProps>,
    widget: Widget,
}

impl Toolbar {
    pub fn new() -> Toolbar {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::toolbar_new()).unsafe_cast() }
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

impl Is<Actor> for Toolbar {}

impl AsRef<Actor> for Toolbar {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

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

    fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
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
        let props = toolbar.props.borrow();

        props.has_close_button
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
        let mut props = toolbar.props.borrow_mut();

        if props.has_close_button != has_close_button {
            props.has_close_button = has_close_button;

            if !has_close_button {
                if props.close_button.is_some() {
                    // clutter_actor_destroy(toolbar.close_button);
                    props.close_button = None;
                }
            } else {
                // props.close_button = button_new ();
                // clutter_actor_set_name(props.close_button, "close-button");
                // clutter_actor_add_child(CLUTTER_ACTOR (toolbar), props.close_button);
                // g_signal_connect(props.close_button, "clicked",
                //                     G_CALLBACK(close_button_click_cb), toolbar);
                // stylable_style_changed(STYLABLE(props.close_button),
                //                             STYLE_CHANGED_FORCE);
            }

            // clutter_actor_queue_relayout(CLUTTER_ACTOR(toolbar));
            // g_object_notify(G_OBJECT(toolbar), "has-close-button");
        }
    }

    fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"close-button-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             close_button_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::has-close-button\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_has_close_button_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
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
