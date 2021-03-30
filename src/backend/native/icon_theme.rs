// use std::boxed::Box as Box_;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IconTheme {}

impl IconTheme {
    pub fn new() -> IconTheme {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::icon_theme_new()) }
        unimplemented!()
    }

    pub fn get_default() -> Option<IconTheme> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::icon_theme_get_default()) }
        unimplemented!()
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ICON_THEME: Option<&IconTheme> = None;

pub trait IconThemeExt: 'static {
    fn get_search_paths(&self) -> Vec<String>;

    fn get_theme_name(&self) -> Option<String>;

    fn has_icon(&self, icon_name: &str) -> bool;

    //fn lookup(&self, icon_name: &str, size: i32) -> /*Ignored*/Option<cogl::Handle>;

    fn set_search_paths(&self, paths: &[&str]);

    fn set_theme_name(&self, theme_name: &str);

    fn connect_property_theme_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<IconTheme>> IconThemeExt for O {
//     fn get_search_paths(&self) -> Vec<String> {
//         unsafe {
//             FromGlibPtrContainer::from_glib_none(ffi::icon_theme_get_search_paths(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_theme_name(&self) -> Option<String> {
//         unsafe {
//             from_glib_none(ffi::icon_theme_get_theme_name(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn has_icon(&self, icon_name: &str) -> bool {
//         unsafe {
//             from_glib(ffi::icon_theme_has_icon(
//                 self.as_ref().to_glib_none().0,
//                 icon_name.to_glib_none().0,
//             ))
//         }
//     }

//     //fn lookup(&self, icon_name: &str, size: i32) -> /*Ignored*/Option<cogl::Handle> {
//     //    unsafe { TODO: call ffi:icon_theme_lookup() }
//     //}

//     fn set_search_paths(&self, paths: &[&str]) {
//         unsafe {
//             ffi::icon_theme_set_search_paths(
//                 self.as_ref().to_glib_none().0,
//                 paths.to_glib_none().0,
//             );
//         }
//     }

//     fn set_theme_name(&self, theme_name: &str) {
//         unsafe {
//             ffi::icon_theme_set_theme_name(
//                 self.as_ref().to_glib_none().0,
//                 theme_name.to_glib_none().0,
//             );
//         }
//     }

//     fn connect_property_theme_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_theme_name_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::IconTheme,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<IconTheme>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&IconTheme::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::theme-name\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_theme_name_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconTheme")
    }
}
