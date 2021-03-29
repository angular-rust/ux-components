// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib::GString;
// use glib_sys;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Entry;
// use Widget;

// glib_wrapper! {
//     pub struct PathBar(Object<ffi::MxPathBar, ffi::MxPathBarClass, PathBarClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_path_bar_get_type(),
//     }
// }

pub struct PathBar {

}

impl PathBar {
    pub fn new() -> PathBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_path_bar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for PathBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PATH_BAR: Option<&PathBar> = None;

// pub trait PathBarExt: 'static {
//     fn clear(&self);

//     fn get_clear_on_change(&self) -> bool;

//     fn get_editable(&self) -> bool;

//     fn get_entry(&self) -> Option<Entry>;

//     fn get_label(&self, level: i32) -> Option<GString>;

//     fn get_level(&self) -> i32;

//     fn get_text(&self) -> Option<GString>;

//     fn pop(&self) -> i32;

//     fn push(&self, name: &str) -> i32;

//     fn set_clear_on_change(&self, clear_on_change: bool);

//     fn set_editable(&self, editable: bool);

//     fn set_label(&self, level: i32, label: &str);

//     fn set_text(&self, text: &str);

//     fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<PathBar>> PathBarExt for O {
//     fn clear(&self) {
//         unsafe {
//             ffi::mx_path_bar_clear(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn get_clear_on_change(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_path_bar_get_clear_on_change(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_editable(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_path_bar_get_editable(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_entry(&self) -> Option<Entry> {
//         unsafe {
//             from_glib_none(ffi::mx_path_bar_get_entry(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_label(&self, level: i32) -> Option<GString> {
//         unsafe {
//             from_glib_none(ffi::mx_path_bar_get_label(
//                 self.as_ref().to_glib_none().0,
//                 level,
//             ))
//         }
//     }

//     fn get_level(&self) -> i32 {
//         unsafe { ffi::mx_path_bar_get_level(self.as_ref().to_glib_none().0) }
//     }

//     fn get_text(&self) -> Option<GString> {
//         unsafe { from_glib_none(ffi::mx_path_bar_get_text(self.as_ref().to_glib_none().0)) }
//     }

//     fn pop(&self) -> i32 {
//         unsafe { ffi::mx_path_bar_pop(self.as_ref().to_glib_none().0) }
//     }

//     fn push(&self, name: &str) -> i32 {
//         unsafe { ffi::mx_path_bar_push(self.as_ref().to_glib_none().0, name.to_glib_none().0) }
//     }

//     fn set_clear_on_change(&self, clear_on_change: bool) {
//         unsafe {
//             ffi::mx_path_bar_set_clear_on_change(
//                 self.as_ref().to_glib_none().0,
//                 clear_on_change.to_glib(),
//             );
//         }
//     }

//     fn set_editable(&self, editable: bool) {
//         unsafe {
//             ffi::mx_path_bar_set_editable(self.as_ref().to_glib_none().0, editable.to_glib());
//         }
//     }

//     fn set_label(&self, level: i32, label: &str) {
//         unsafe {
//             ffi::mx_path_bar_set_label(
//                 self.as_ref().to_glib_none().0,
//                 level,
//                 label.to_glib_none().0,
//             );
//         }
//     }

//     fn set_text(&self, text: &str) {
//         unsafe {
//             ffi::mx_path_bar_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         }
//     }

//     fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clear_on_change_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxPathBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<PathBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::clear-on-change\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_clear_on_change_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxPathBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<PathBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::editable\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_editable_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_entry_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxPathBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<PathBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::entry\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_entry_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_level_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxPathBar,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<PathBar>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::level\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_level_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for PathBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathBar")
    }
}
