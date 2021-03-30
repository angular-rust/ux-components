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
pub struct Label {}

impl Label {
    pub fn new() -> Label {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::label_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_text(text: &str) -> Label {
        // assert_initialized_main_thread!();
        // unsafe {
        //     clutter::Actor::from_glib_none(ffi::label_new_with_text(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }

    // pub fn new() -> Label {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::label_new()) }
    // }

    // pub fn with_text(text: &str) -> Label {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::label_new_with_text()) }
    // }
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LABEL: Option<&Label> = None;

pub trait LabelExt: 'static {
    //fn get_alignment(&self) -> (/*Ignored*/Align, /*Ignored*/Align);

    fn get_clutter_text(&self) -> Option<clutter::Actor>;

    fn get_fade_out(&self) -> bool;

    fn get_line_wrap(&self) -> bool;

    fn get_show_tooltip(&self) -> bool;

    fn get_text(&self) -> Option<String>;

    fn get_use_markup(&self) -> bool;

    //fn get_x_align(&self) -> /*Ignored*/Align;

    //fn get_y_align(&self) -> /*Ignored*/Align;

    //fn set_alignment(&self, x_align: /*Ignored*/Align, y_align: /*Ignored*/Align);

    fn set_fade_out(&self, fade: bool);

    fn set_line_wrap(&self, line_wrap: bool);

    fn set_show_tooltip(&self, show_tooltip: bool);

    fn set_text(&self, text: &str);

    fn set_use_markup(&self, use_markup: bool);

    //fn set_x_align(&self, align: /*Ignored*/Align);

    //fn set_y_align(&self, align: /*Ignored*/Align);

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_fade_out_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<Label>> LabelExt for O {
//     //fn get_alignment(&self) -> (/*Ignored*/Align, /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:label_get_alignment() }
//     //}

//     fn get_clutter_text(&self) -> Option<clutter::Actor> {
//         unsafe {
//             from_glib_none(ffi::label_get_clutter_text(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_fade_out(&self) -> bool {
//         unsafe {
//             from_glib(ffi::label_get_fade_out(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_line_wrap(&self) -> bool {
//         unsafe {
//             from_glib(ffi::label_get_line_wrap(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_show_tooltip(&self) -> bool {
//         unsafe {
//             from_glib(ffi::label_get_show_tooltip(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_text(&self) -> Option<String> {
//         unsafe { from_glib_none(ffi::label_get_text(self.as_ref().to_glib_none().0)) }
//     }

//     fn get_use_markup(&self) -> bool {
//         unsafe {
//             from_glib(ffi::label_get_use_markup(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn get_x_align(&self) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:label_get_x_align() }
//     //}

//     //fn get_y_align(&self) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:label_get_y_align() }
//     //}

//     //fn set_alignment(&self, x_align: /*Ignored*/Align, y_align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:label_set_alignment() }
//     //}

//     fn set_fade_out(&self, fade: bool) {
//         unsafe {
//             ffi::label_set_fade_out(self.as_ref().to_glib_none().0, fade.to_glib());
//         }
//     }

//     fn set_line_wrap(&self, line_wrap: bool) {
//         unsafe {
//             ffi::label_set_line_wrap(self.as_ref().to_glib_none().0, line_wrap.to_glib());
//         }
//     }

//     fn set_show_tooltip(&self, show_tooltip: bool) {
//         unsafe {
//             ffi::label_set_show_tooltip(
//                 self.as_ref().to_glib_none().0,
//                 show_tooltip.to_glib(),
//             );
//         }
//     }

//     fn set_text(&self, text: &str) {
//         unsafe {
//             ffi::label_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         }
//     }

//     fn set_use_markup(&self, use_markup: bool) {
//         unsafe {
//             ffi::label_set_use_markup(self.as_ref().to_glib_none().0, use_markup.to_glib());
//         }
//     }

//     //fn set_x_align(&self, align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:label_set_x_align() }
//     //}

//     //fn set_y_align(&self, align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:label_set_y_align() }
//     //}

//     fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clutter_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
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

//     fn connect_property_fade_out_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_fade_out_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::fade-out\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_fade_out_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_line_wrap_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::line-wrap\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_line_wrap_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_show_tooltip_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_show_tooltip_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::show-tooltip\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_show_tooltip_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
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

//     fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::use-markup\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_use_markup_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::x-align\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_x_align_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Label,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Label>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Label::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::y-align\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_y_align_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label")
    }
}
