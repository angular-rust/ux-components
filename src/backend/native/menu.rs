#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Action, FloatingWidget, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends FloatingWidget, Widget, clutter::Actor
#[derive(Clone, Debug)]
pub struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::menu_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Menu {}
impl Is<Menu> for Menu {}

impl AsRef<Menu> for Menu {
    fn as_ref(&self) -> &Menu {
        self
    }
}

pub const NONE_MENU: Option<&Menu> = None;

pub trait MenuExt: 'static {
    fn add_action<P: Is<Action>>(&self, action: &P);

    fn remove_action<P: Is<Action>>(&self, action: &P);

    fn remove_all(&self);

    fn show_with_position(&self, x: f32, y: f32);

    fn connect_action_activated<F: Fn(&Self, &Action) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Menu>> MenuExt for O {
    fn add_action<P: Is<Action>>(&self, action: &P) {
        // unsafe {
        //     ffi::menu_add_action(
        //         self.as_ref().to_glib_none().0,
        //         action.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_action<P: Is<Action>>(&self, action: &P) {
        // unsafe {
        //     ffi::menu_remove_action(
        //         self.as_ref().to_glib_none().0,
        //         action.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_all(&self) {
        // unsafe {
        //     ffi::menu_remove_all(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn show_with_position(&self, x: f32, y: f32) {
        // unsafe {
        //     ffi::menu_show_with_position(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn connect_action_activated<F: Fn(&Self, &Action) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn action_activated_trampoline<P, F: Fn(&P, &Action) + 'static>(
        //     this: *mut ffi::Menu,
        //     object: *mut ffi::Action,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Menu>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(
        //         &Menu::from_glib_borrow(this).unsafe_cast_ref(),
        //         &from_glib_borrow(object),
        //     )
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"action-activated\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             action_activated_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
