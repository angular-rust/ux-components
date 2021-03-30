#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Entry, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct PathBar {}

impl PathBar {
    pub fn new() -> PathBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::path_bar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> PathBar {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::path_bar_new()) }
    // }
}

impl Default for PathBar {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for PathBar {}
impl Is<PathBar> for PathBar {}

impl AsRef<PathBar> for PathBar {
    fn as_ref(&self) -> &PathBar {
        unimplemented!()
    }
}

pub const NONE_PATH_BAR: Option<&PathBar> = None;

pub trait PathBarExt: 'static {
    fn clear(&self);

    fn get_clear_on_change(&self) -> bool;

    fn get_editable(&self) -> bool;

    fn get_entry(&self) -> Option<Entry>;

    fn get_label(&self, level: i32) -> Option<String>;

    fn get_level(&self) -> i32;

    fn get_text(&self) -> Option<String>;

    fn pop(&self) -> i32;

    fn push(&self, name: &str) -> i32;

    fn set_clear_on_change(&self, clear_on_change: bool);

    fn set_editable(&self, editable: bool);

    fn set_label(&self, level: i32, label: &str);

    fn set_text(&self, text: &str);

    fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<PathBar>> PathBarExt for O {
    fn clear(&self) {
        // unsafe {
        //     ffi::path_bar_clear(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn get_clear_on_change(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::path_bar_get_clear_on_change(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_editable(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::path_bar_get_editable(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_entry(&self) -> Option<Entry> {
        // unsafe {
        //     from_glib_none(ffi::path_bar_get_entry(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_label(&self, level: i32) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::path_bar_get_label(
        //         self.as_ref().to_glib_none().0,
        //         level,
        //     ))
        // }
        unimplemented!()
    }

    fn get_level(&self) -> i32 {
        // unsafe { ffi::path_bar_get_level(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_text(&self) -> Option<String> {
        // unsafe { from_glib_none(ffi::path_bar_get_text(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn pop(&self) -> i32 {
        // unsafe { ffi::path_bar_pop(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn push(&self, name: &str) -> i32 {
        // unsafe { ffi::path_bar_push(self.as_ref().to_glib_none().0, name.to_glib_none().0) }
        unimplemented!()
    }

    fn set_clear_on_change(&self, clear_on_change: bool) {
        // unsafe {
        //     ffi::path_bar_set_clear_on_change(
        //         self.as_ref().to_glib_none().0,
        //         clear_on_change.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_editable(&self, editable: bool) {
        // unsafe {
        //     ffi::path_bar_set_editable(self.as_ref().to_glib_none().0, editable.to_glib());
        // }
        unimplemented!()
    }

    fn set_label(&self, level: i32, label: &str) {
        // unsafe {
        //     ffi::path_bar_set_label(
        //         self.as_ref().to_glib_none().0,
        //         level,
        //         label.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_text(&self, text: &str) {
        // unsafe {
        //     ffi::path_bar_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clear_on_change_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clear-on-change\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clear_on_change_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::editable\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_editable_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_entry_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::entry\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_entry_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_level_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::level\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_level_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for PathBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathBar")
    }
}
