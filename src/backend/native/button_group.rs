#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Button;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ButtonGroup {}

impl ButtonGroup {
    pub fn new() -> ButtonGroup {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::button_group_new()) }
        unimplemented!()
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for ButtonGroup {}
impl Is<ButtonGroup> for ButtonGroup {}

impl AsRef<ButtonGroup> for ButtonGroup {
    fn as_ref(&self) -> &ButtonGroup {
        unimplemented!()
    }
}

pub const NONE_BUTTON_GROUP: Option<&ButtonGroup> = None;

pub trait ButtonGroupExt: 'static {
    fn add<P: Is<Button>>(&self, button: &P);

    fn foreach<P: FnMut(&clutter::Actor)>(&self, callback: P);

    fn get_active_button(&self) -> Option<Button>;

    fn get_allow_no_active(&self) -> bool;

    fn get_buttons(&self) -> Vec<Button>;

    fn remove<P: Is<Button>>(&self, button: &P);

    fn set_active_button<P: Is<Button>>(&self, button: Option<&P>);

    fn set_allow_no_active(&self, allow_no_active: bool);

    fn connect_property_active_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_allow_no_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<ButtonGroup>> ButtonGroupExt for O {
    fn add<P: Is<Button>>(&self, button: &P) {
        // unsafe {
        //     ffi::button_group_add(
        //         self.as_ref().to_glib_none().0,
        //         button.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn foreach<P: FnMut(&clutter::Actor)>(&self, callback: P) {
        // let callback_data: P = callback;
        // unsafe extern "C" fn callback_func<P: FnMut(&clutter::Actor)>(
        //     actor: *mut clutter_sys::ClutterActor,
        //     data: glib_sys::gpointer,
        // ) {
        //     let actor = from_glib_borrow(actor);
        //     let callback: *mut P = data as *const _ as usize as *mut P;
        //     (*callback)(&actor);
        // }
        // let callback = Some(callback_func::<P> as _);
        // let super_callback0: &P = &callback_data;
        // unsafe {
        //     ffi::button_group_foreach(
        //         self.as_ref().to_glib_none().0,
        //         callback,
        //         super_callback0 as *const _ as usize as *mut _,
        //     );
        // }
        unimplemented!()
    }

    fn get_active_button(&self) -> Option<Button> {
        // unsafe {
        //     from_glib_none(ffi::button_group_get_active_button(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_allow_no_active(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::button_group_get_allow_no_active(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_buttons(&self) -> Vec<Button> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_none(ffi::button_group_get_buttons(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn remove<P: Is<Button>>(&self, button: &P) {
        // unsafe {
        //     ffi::button_group_remove(
        //         self.as_ref().to_glib_none().0,
        //         button.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_active_button<P: Is<Button>>(&self, button: Option<&P>) {
        // unsafe {
        //     ffi::button_group_set_active_button(
        //         self.as_ref().to_glib_none().0,
        //         button.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_allow_no_active(&self, allow_no_active: bool) {
        // unsafe {
        //     ffi::button_group_set_allow_no_active(
        //         self.as_ref().to_glib_none().0,
        //         allow_no_active.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_active_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_active_button_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ButtonGroup,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ButtonGroup>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ButtonGroup::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-button\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_button_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_allow_no_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_allow_no_active_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ButtonGroup,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ButtonGroup>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ButtonGroup::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::allow-no-active\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_allow_no_active_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ButtonGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ButtonGroup")
    }
}
