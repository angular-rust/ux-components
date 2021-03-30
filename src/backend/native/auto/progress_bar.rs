use crate::prelude::*;
// use glib::object::Cast;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;


// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Widget;

// glib_wrapper! {
//     pub struct ProgressBar(Object<ffi::ProgressBar, ffi::ProgressBarClass, ProgressBarClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::progress_bar_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct ProgressBar {

}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::progress_bar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PROGRESS_BAR: Option<&ProgressBar> = None;

// pub trait ProgressBarExt: 'static {
//     fn get_progress(&self) -> f64;

//     fn set_progress(&self, progress: f64);

//     fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<ProgressBar>> ProgressBarExt for O {
//     fn get_progress(&self) -> f64 {
//         unsafe { ffi::progress_bar_get_progress(self.as_ref().to_glib_none().0) }
//     }

//     fn set_progress(&self, progress: f64) {
//         unsafe {
//             ffi::progress_bar_set_progress(self.as_ref().to_glib_none().0, progress);
//         }
//     }

//     fn connect_property_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_progress_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ProgressBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ProgressBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ProgressBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::progress\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_progress_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProgressBar")
    }
}
