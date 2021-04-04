#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Actor;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ComboBox {}

impl ComboBox {
    pub fn new() -> ComboBox {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::combo_box_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> ComboBox {
    //     unimplemented!(); // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::combo_box_new()) }
    // }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ComboBox {}
impl Is<ComboBox> for ComboBox {}

impl AsRef<ComboBox> for ComboBox {
    fn as_ref(&self) -> &ComboBox {
        self
    }
}

pub const NONE_COMBO_BOX: Option<&ComboBox> = None;

pub trait ComboBoxExt: 'static {
    fn append_text(&self, text: &str);

    fn get_active_icon_name(&self) -> Option<String>;

    fn get_active_text(&self) -> Option<String>;

    fn get_index(&self) -> i32;

    fn insert_text(&self, position: i32, text: &str);

    fn insert_text_with_icon(&self, position: i32, text: &str, icon: &str);

    fn prepend_text(&self, text: &str);

    fn remove_all(&self);

    fn remove_text(&self, position: i32);

    fn set_active_icon_name(&self, icon_name: Option<&str>);

    fn set_active_text(&self, text: &str);

    fn set_index(&self, index: i32);

    fn connect_property_active_icon_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_active_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ComboBox>> ComboBoxExt for O {
    fn append_text(&self, text: &str) {
        // unsafe {
        //     ffi::combo_box_append_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn get_active_icon_name(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::combo_box_get_active_icon_name(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_active_text(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::combo_box_get_active_text(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_index(&self) -> i32 {
        // unsafe { ffi::combo_box_get_index(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn insert_text(&self, position: i32, text: &str) {
        // unsafe {
        //     ffi::combo_box_insert_text(
        //         self.as_ref().to_glib_none().0,
        //         position,
        //         text.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn insert_text_with_icon(&self, position: i32, text: &str, icon: &str) {
        // unsafe {
        //     ffi::combo_box_insert_text_with_icon(
        //         self.as_ref().to_glib_none().0,
        //         position,
        //         text.to_glib_none().0,
        //         icon.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn prepend_text(&self, text: &str) {
        // unsafe {
        //     ffi::combo_box_prepend_text(
        //         self.as_ref().to_glib_none().0,
        //         text.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_all(&self) {
        // unsafe {
        //     ffi::combo_box_remove_all(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_text(&self, position: i32) {
        // unsafe {
        //     ffi::combo_box_remove_text(self.as_ref().to_glib_none().0, position);
        // }
        unimplemented!()
    }

    fn set_active_icon_name(&self, icon_name: Option<&str>) {
        // unsafe {
        //     ffi::combo_box_set_active_icon_name(
        //         self.as_ref().to_glib_none().0,
        //         icon_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_active_text(&self, text: &str) {
        // unsafe {
        //     ffi::combo_box_set_active_text(
        //         self.as_ref().to_glib_none().0,
        //         text.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_index(&self, index: i32) {
        // unsafe {
        //     ffi::combo_box_set_index(self.as_ref().to_glib_none().0, index);
        // }
        unimplemented!()
    }

    fn connect_property_active_icon_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_active_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_active_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_active_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_index_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::index\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_index_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ComboBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboBox")
    }
}
