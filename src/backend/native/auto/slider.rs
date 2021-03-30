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
//     pub struct Slider(Object<ffi::Slider, ffi::SliderClass, SliderClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::slider_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Slider {

}

impl Slider {
    pub fn new() -> Slider {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::slider_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SLIDER: Option<&Slider> = None;

// pub trait SliderExt: 'static {
//     fn get_buffer_value(&self) -> f64;

//     fn get_value(&self) -> f64;

//     fn set_buffer_value(&self, value: f64);

//     fn set_value(&self, value: f64);

//     fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Slider>> SliderExt for O {
//     fn get_buffer_value(&self) -> f64 {
//         unsafe { ffi::slider_get_buffer_value(self.as_ref().to_glib_none().0) }
//     }

//     fn get_value(&self) -> f64 {
//         unsafe { ffi::slider_get_value(self.as_ref().to_glib_none().0) }
//     }

//     fn set_buffer_value(&self, value: f64) {
//         unsafe {
//             ffi::slider_set_buffer_value(self.as_ref().to_glib_none().0, value);
//         }
//     }

//     fn set_value(&self, value: f64) {
//         unsafe {
//             ffi::slider_set_value(self.as_ref().to_glib_none().0, value);
//         }
//     }

//     fn connect_slide_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn slide_start_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Slider,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Slider>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"slide-start\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     slide_start_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_slide_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn slide_stop_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Slider,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Slider>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"slide-stop\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     slide_stop_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_buffer_value_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_buffer_value_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Slider,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Slider>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::buffer-value\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_buffer_value_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Slider,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Slider>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Slider::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::value\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_value_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Slider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slider")
    }
}
