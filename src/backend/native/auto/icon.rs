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
//     pub struct Icon(Object<ffi::Icon, ffi::IconClass, IconClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::icon_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Icon {

}

impl Icon {
    pub fn new() -> Icon {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::icon_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ICON: Option<&Icon> = None;

// pub trait IconExt: 'static {
//     fn get_icon_name(&self) -> Option<String>;

//     fn get_icon_size(&self) -> i32;

//     fn set_icon_name(&self, icon_name: &str);

//     fn set_icon_size(&self, size: i32);

//     fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Icon>> IconExt for O {
//     fn get_icon_name(&self) -> Option<String> {
//         unsafe {
//             from_glib_none(ffi::icon_get_icon_name(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_icon_size(&self) -> i32 {
//         unsafe { ffi::icon_get_icon_size(self.as_ref().to_glib_none().0) }
//     }

//     fn set_icon_name(&self, icon_name: &str) {
//         unsafe {
//             ffi::icon_set_icon_name(
//                 self.as_ref().to_glib_none().0,
//                 icon_name.to_glib_none().0,
//             );
//         }
//     }

//     fn set_icon_size(&self, size: i32) {
//         unsafe {
//             ffi::icon_set_icon_size(self.as_ref().to_glib_none().0, size);
//         }
//     }

//     fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Icon,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Icon>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-name\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_name_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Icon,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Icon>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-size\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_size_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Icon")
    }
}
