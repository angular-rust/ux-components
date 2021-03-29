// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib_sys;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Widget;

// glib_wrapper! {
//     pub struct Toggle(Object<ffi::MxToggle, ffi::MxToggleClass, ToggleClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_toggle_get_type(),
//     }
// }

pub struct Toggle {

}

impl Toggle {
    pub fn new() -> Toggle {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_toggle_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TOGGLE: Option<&Toggle> = None;

// pub trait ToggleExt: 'static {
//     fn get_active(&self) -> bool;

//     fn set_active(&self, active: bool);

//     fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Toggle>> ToggleExt for O {
//     fn get_active(&self) -> bool {
//         unsafe { from_glib(ffi::mx_toggle_get_active(self.as_ref().to_glib_none().0)) }
//     }

//     fn set_active(&self, active: bool) {
//         unsafe {
//             ffi::mx_toggle_set_active(self.as_ref().to_glib_none().0, active.to_glib());
//         }
//     }

//     fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxToggle,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Toggle>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Toggle::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::active\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_active_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toggle")
    }
}
