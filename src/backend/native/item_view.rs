#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Grid, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Grid, Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ItemView {}

impl ItemView {
    pub fn new() -> ItemView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::item_view_new()).unsafe_cast() }
        unimplemented!()
    }
    // pub fn new() -> ItemView {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::item_view_new()) }
    // }
}

impl Default for ItemView {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ItemView {}
impl Is<ItemView> for ItemView {}

impl AsRef<ItemView> for ItemView {
    fn as_ref(&self) -> &ItemView {
        self
    }
}

pub const NONE_ITEM_VIEW: Option<&ItemView> = None;

pub trait ItemViewExt: 'static {
    fn add_attribute(&self, attribute: &str, column: i32);

    fn freeze(&self);

    //fn get_factory(&self) -> /*Ignored*/Option<ItemFactory>;

    fn get_item_type(&self) -> glib::types::Type;

    fn get_model(&self) -> Option<clutter::Model>;

    //fn set_factory(&self, factory: /*Ignored*/Option<&ItemFactory>);

    fn set_item_type(&self, item_type: glib::types::Type);

    // fn set_model<P: Is<clutter::Model>>(&self, model: &P);

    fn thaw(&self);

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ItemView>> ItemViewExt for O {
    fn add_attribute(&self, attribute: &str, column: i32) {
        // unsafe {
        //     ffi::item_view_add_attribute(
        //         self.as_ref().to_glib_none().0,
        //         attribute.to_glib_none().0,
        //         column,
        //     );
        // }
        unimplemented!()
    }

    fn freeze(&self) {
        // unsafe {
        //     ffi::item_view_freeze(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    //fn get_factory(&self) -> /*Ignored*/Option<ItemFactory> {
    //    unsafe { TODO: call ffi:item_view_get_factory() }
    //}

    fn get_item_type(&self) -> glib::types::Type {
        // unsafe {
        //     from_glib(ffi::item_view_get_item_type(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_model(&self) -> Option<clutter::Model> {
        // unsafe {
        //     from_glib_none(ffi::item_view_get_model(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    //fn set_factory(&self, factory: /*Ignored*/Option<&ItemFactory>) {
    //    unsafe { TODO: call ffi:item_view_set_factory() }
    //}

    fn set_item_type(&self, item_type: glib::types::Type) {
        // unsafe {
        //     ffi::item_view_set_item_type(self.as_ref().to_glib_none().0, item_type.to_glib());
        // }
        unimplemented!()
    }

    // fn set_model<P: Is<clutter::Model>>(&self, model: &P) {
    //     unsafe {
    //         ffi::item_view_set_model(
    //             self.as_ref().to_glib_none().0,
    //             model.as_ref().to_glib_none().0,
    //         );
    //     }
    // }

    fn thaw(&self) {
        // unsafe {
        //     ffi::item_view_thaw(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::factory\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_factory_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_item_type_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::item-type\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_item_type_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::model\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_model_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ItemView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ItemView")
    }
}
