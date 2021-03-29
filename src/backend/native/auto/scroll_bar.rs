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
// use Adjustment;
// use Widget;

// glib_wrapper! {
//     pub struct ScrollBar(Object<ffi::MxScrollBar, ffi::MxScrollBarClass, ScrollBarClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_scroll_bar_get_type(),
//     }
// }

pub struct ScrollBar {

}

impl ScrollBar {
    pub fn new() -> ScrollBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_scroll_bar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn with_adjustment<P: IsA<Adjustment>>(adjustment: &P) -> ScrollBar {
    //     // skip_assert_initialized!();
    //     // unsafe {
    //     //     clutter::Actor::from_glib_none(ffi::mx_scroll_bar_new_with_adjustment(
    //     //         adjustment.as_ref().to_glib_none().0,
    //     //     ))
    //     //     .unsafe_cast()
    //     // }
    //     unimplemented!()
    // }
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SCROLL_BAR: Option<&ScrollBar> = None;

// pub trait ScrollBarExt: 'static {
//     fn get_adjustment(&self) -> Option<Adjustment>;

//     //fn get_orientation(&self) -> /*Ignored*/Orientation;

//     fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation);

//     fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<ScrollBar>> ScrollBarExt for O {
//     fn get_adjustment(&self) -> Option<Adjustment> {
//         unsafe {
//             from_glib_none(ffi::mx_scroll_bar_get_adjustment(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn get_orientation(&self) -> /*Ignored*/Orientation {
//     //    unsafe { TODO: call ffi:mx_scroll_bar_get_orientation() }
//     //}

//     fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
//         unsafe {
//             ffi::mx_scroll_bar_set_adjustment(
//                 self.as_ref().to_glib_none().0,
//                 adjustment.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation) {
//     //    unsafe { TODO: call ffi:mx_scroll_bar_set_orientation() }
//     //}

//     fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn scroll_start_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxScrollBar,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"scroll-start\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     scroll_start_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn scroll_stop_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxScrollBar,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"scroll-stop\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     scroll_stop_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxScrollBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::adjustment\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_adjustment_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxScrollBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::orientation\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_orientation_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for ScrollBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollBar")
    }
}
