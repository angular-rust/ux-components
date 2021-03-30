#![allow(unused_variables)]
#![allow(dead_code)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Entry {}

impl Entry {
    pub fn new() -> Entry {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::entry_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_text(text: &str) -> Entry {
        // assert_initialized_main_thread!();
        // unsafe {
        //     clutter::Actor::from_glib_none(ffi::entry_new_with_text(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }

    // pub fn new() -> Entry {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::entry_new()) }
    // }

    // pub fn with_text(text: &str) -> Entry {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::entry_new_with_text()) }
    // }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ENTRY: Option<&Entry> = None;

pub trait EntryExt: 'static {
    fn get_clutter_text(&self) -> Option<clutter::Actor>;

    fn get_icon_highlight_suffix(&self) -> Option<String>;

    fn get_password_char(&self) -> char;

    fn get_placeholder(&self) -> Option<String>;

    fn get_text(&self) -> Option<String>;

    fn set_icon_highlight_suffix(&self, suffix: &str);

    fn set_password_char(&self, password_char: char);

    fn set_placeholder(&self, text: &str);

    fn set_primary_icon_from_file(&self, filename: &str);

    fn set_primary_icon_tooltip_text(&self, text: &str);

    fn set_secondary_icon_from_file(&self, filename: &str);

    fn set_secondary_icon_tooltip_text(&self, text: &str);

    fn set_text(&self, text: &str);

    fn get_property_primary_icon_tooltip_text(&self) -> Option<String>;

    fn get_property_secondary_icon_tooltip_text(&self) -> Option<String>;

    fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<Entry>> EntryExt for O {
//     fn get_clutter_text(&self) -> Option<clutter::Actor> {
//         unsafe {
//             from_glib_none(ffi::entry_get_clutter_text(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_icon_highlight_suffix(&self) -> Option<String> {
//         unsafe {
//             from_glib_none(ffi::entry_get_icon_highlight_suffix(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_password_char(&self) -> char {
//         unsafe {
//             from_glib(ffi::entry_get_password_char(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_placeholder(&self) -> Option<String> {
//         unsafe {
//             from_glib_none(ffi::entry_get_placeholder(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_text(&self) -> Option<String> {
//         unsafe { from_glib_none(ffi::entry_get_text(self.as_ref().to_glib_none().0)) }
//     }

//     fn set_icon_highlight_suffix(&self, suffix: &str) {
//         unsafe {
//             ffi::entry_set_icon_highlight_suffix(
//                 self.as_ref().to_glib_none().0,
//                 suffix.to_glib_none().0,
//             );
//         }
//     }

//     fn set_password_char(&self, password_char: char) {
//         unsafe {
//             ffi::entry_set_password_char(
//                 self.as_ref().to_glib_none().0,
//                 password_char.to_glib(),
//             );
//         }
//     }

//     fn set_placeholder(&self, text: &str) {
//         unsafe {
//             ffi::entry_set_placeholder(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         }
//     }

//     fn set_primary_icon_from_file(&self, filename: &str) {
//         unsafe {
//             ffi::entry_set_primary_icon_from_file(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//             );
//         }
//     }

//     fn set_primary_icon_tooltip_text(&self, text: &str) {
//         unsafe {
//             ffi::entry_set_primary_icon_tooltip_text(
//                 self.as_ref().to_glib_none().0,
//                 text.to_glib_none().0,
//             );
//         }
//     }

//     fn set_secondary_icon_from_file(&self, filename: &str) {
//         unsafe {
//             ffi::entry_set_secondary_icon_from_file(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//             );
//         }
//     }

//     fn set_secondary_icon_tooltip_text(&self, text: &str) {
//         unsafe {
//             ffi::entry_set_secondary_icon_tooltip_text(
//                 self.as_ref().to_glib_none().0,
//                 text.to_glib_none().0,
//             );
//         }
//     }

//     fn set_text(&self, text: &str) {
//         unsafe {
//             ffi::entry_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         }
//     }

//     fn get_property_primary_icon_tooltip_text(&self) -> Option<String> {
//         unsafe {
//             let mut value = Value::from_type(<String as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"primary-icon-tooltip-text\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `primary-icon-tooltip-text` getter")
//         }
//     }

//     fn get_property_secondary_icon_tooltip_text(&self) -> Option<String> {
//         unsafe {
//             let mut value = Value::from_type(<String as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"secondary-icon-tooltip-text\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `secondary-icon-tooltip-text` getter")
//         }
//     }

//     fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn primary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"primary-icon-clicked\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     primary_icon_clicked_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn secondary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"secondary-icon-clicked\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     secondary_icon_clicked_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clutter_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::clutter-text\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_clutter_text_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_highlight_suffix_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-highlight-suffix\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_highlight_suffix_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_password_char_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::password-char\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_password_char_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_placeholder_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::placeholder\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_placeholder_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_primary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::primary-icon-tooltip-text\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_primary_icon_tooltip_text_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_secondary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::secondary-icon-tooltip-text\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_secondary_icon_tooltip_text_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Entry,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Entry>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::text\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_text_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry")
    }
}
