// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Notebook {}

impl Notebook {
    pub fn new() -> Notebook {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::notebook_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Notebook {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::notebook_new()) }
    // }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_NOTEBOOK: Option<&Notebook> = None;

pub trait NotebookExt: 'static {
    fn get_current_page(&self) -> Option<clutter::Actor>;

    fn next_page(&self);

    fn previous_page(&self);

    fn set_current_page<P: Is<clutter::Actor>>(&self, page: &P);

    fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

// impl<O: Is<Notebook>> NotebookExt for O {
//     fn get_current_page(&self) -> Option<clutter::Actor> {
//         unsafe {
//             from_glib_none(ffi::notebook_get_current_page(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn next_page(&self) {
//         unsafe {
//             ffi::notebook_next_page(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn previous_page(&self) {
//         unsafe {
//             ffi::notebook_previous_page(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn set_current_page<P: Is<clutter::Actor>>(&self, page: &P) {
//         unsafe {
//             ffi::notebook_set_current_page(
//                 self.as_ref().to_glib_none().0,
//                 page.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_current_page_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Notebook,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Notebook>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Notebook::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::current-page\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_current_page_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notebook")
    }
}
