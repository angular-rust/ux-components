// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Toolbar {}

impl Toolbar {
    pub fn new() -> Toolbar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::toolbar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Toolbar {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::toolbar_new()) }
    // }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl UxComponent for Toolbar {}

pub const NONE_TOOLBAR: Option<&Toolbar> = None;

pub trait ToolbarExt: 'static {
    fn get_has_close_button(&self) -> bool;

    fn set_has_close_button(&self, has_close_button: bool);

    fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

// impl<O: Is<Toolbar>> ToolbarExt for O {
//     fn get_has_close_button(&self) -> bool {
//         unsafe {
//             from_glib(ffi::toolbar_get_has_close_button(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn set_has_close_button(&self, has_close_button: bool) {
//         unsafe {
//             ffi::toolbar_set_has_close_button(
//                 self.as_ref().to_glib_none().0,
//                 has_close_button.to_glib(),
//             );
//         }
//     }

//     fn connect_close_button_clicked<F: Fn(&Self) -> bool + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn close_button_clicked_trampoline<P, F: Fn(&P) -> bool + 'static>(
//             this: *mut ffi::Toolbar,
//             f: glib_sys::gpointer,
//         ) -> glib_sys::gboolean
//         where
//             P: Is<Toolbar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Toolbar::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"close-button-clicked\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     close_button_clicked_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_has_close_button_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_has_close_button_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Toolbar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Toolbar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Toolbar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::has-close-button\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_has_close_button_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Toolbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toolbar")
    }
}
