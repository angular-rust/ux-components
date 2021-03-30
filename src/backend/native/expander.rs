// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Expander {}

impl Expander {
    pub fn new() -> Expander {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::expander_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Expander {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::expander_new()) }
    // }
}

impl Default for Expander {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

pub trait ExpanderExt: 'static {
    fn get_expanded(&self) -> bool;

    fn set_expanded(&self, expanded: bool);

    fn set_label(&self, label: &str);

    fn get_property_label(&self) -> Option<String>;

    fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<Expander>> ExpanderExt for O {
//     fn get_expanded(&self) -> bool {
//         unsafe {
//             from_glib(ffi::expander_get_expanded(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn set_expanded(&self, expanded: bool) {
//         unsafe {
//             ffi::expander_set_expanded(self.as_ref().to_glib_none().0, expanded.to_glib());
//         }
//     }

//     fn set_label(&self, label: &str) {
//         unsafe {
//             ffi::expander_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
//         }
//     }

//     fn get_property_label(&self) -> Option<String> {
//         unsafe {
//             let mut value = Value::from_type(<String as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"label\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `label` getter")
//         }
//     }

//     fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn expand_complete_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Expander,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"expand-complete\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     expand_complete_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Expander,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::expanded\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_expanded_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Expander,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::label\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_label_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expander")
    }
}
