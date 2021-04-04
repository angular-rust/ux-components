#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct FocusManager {}

impl FocusManager {
    //pub fn get_for_stage(stage: /*Ignored*/&clutter::Stage) -> Option<FocusManager> {
    //    unsafe { TODO: call ffi:focus_manager_get_for_stage() }
    //}
}

impl Object for FocusManager {}
impl Is<FocusManager> for FocusManager {}

impl AsRef<FocusManager> for FocusManager {
    fn as_ref(&self) -> &FocusManager {
        self
    }
}

pub const NONE_FOCUS_MANAGER: Option<&FocusManager> = None;

pub trait FocusManagerExt: 'static {
    //fn get_focused(&self) -> /*Ignored*/Option<Focusable>;

    fn get_stage(&self) -> Option<clutter::Stage>;

    //fn move_focus(&self, direction: /*Ignored*/FocusDirection);

    //fn push_focus(&self, focusable: /*Ignored*/&Focusable);

    //fn push_focus_with_hint(&self, focusable: /*Ignored*/&Focusable, hint: /*Ignored*/FocusHint);

    fn connect_property_focused_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<FocusManager>> FocusManagerExt for O {
    //fn get_focused(&self) -> /*Ignored*/Option<Focusable> {
    //    unsafe { TODO: call ffi:focus_manager_get_focused() }
    //}

    fn get_stage(&self) -> Option<clutter::Stage> {
        //    unsafe { TODO: call ffi:focus_manager_get_stage() }
        unimplemented!()
    }

    //fn move_focus(&self, direction: /*Ignored*/FocusDirection) {
    //    unsafe { TODO: call ffi:focus_manager_move_focus() }
    //}

    //fn push_focus(&self, focusable: /*Ignored*/&Focusable) {
    //    unsafe { TODO: call ffi:focus_manager_push_focus() }
    //}

    //fn push_focus_with_hint(&self, focusable: /*Ignored*/&Focusable, hint: /*Ignored*/FocusHint) {
    //    unsafe { TODO: call ffi:focus_manager_push_focus_with_hint() }
    //}

    fn connect_property_focused_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_focused_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FocusManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FocusManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FocusManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::focused\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_focused_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_stage_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FocusManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FocusManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FocusManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::stage\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_stage_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for FocusManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FocusManager")
    }
}
