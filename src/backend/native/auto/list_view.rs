// use clutter;
// use glib;
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
// use BoxLayout;
// use Widget;

// glib_wrapper! {
//     pub struct ListView(Object<ffi::MxListView, ffi::MxListViewClass, ListViewClass>) @extends BoxLayout, Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_list_view_get_type(),
//     }
// }

pub struct ListView {

}

impl ListView {
    pub fn new() -> ListView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_list_view_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LIST_VIEW: Option<&ListView> = None;

// pub trait ListViewExt: 'static {
//     fn add_attribute(&self, attribute: &str, column: i32);

//     fn freeze(&self);

//     //fn get_factory(&self) -> /*Ignored*/Option<ItemFactory>;

//     fn get_item_type(&self) -> glib::types::Type;

//     fn get_model(&self) -> Option<clutter::Model>;

//     //fn set_factory(&self, factory: /*Ignored*/Option<&ItemFactory>);

//     fn set_item_type(&self, item_type: glib::types::Type);

//     fn set_model<P: IsA<clutter::Model>>(&self, model: &P);

//     fn thaw(&self);

//     fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<ListView>> ListViewExt for O {
//     fn add_attribute(&self, attribute: &str, column: i32) {
//         unsafe {
//             ffi::mx_list_view_add_attribute(
//                 self.as_ref().to_glib_none().0,
//                 attribute.to_glib_none().0,
//                 column,
//             );
//         }
//     }

//     fn freeze(&self) {
//         unsafe {
//             ffi::mx_list_view_freeze(self.as_ref().to_glib_none().0);
//         }
//     }

//     //fn get_factory(&self) -> /*Ignored*/Option<ItemFactory> {
//     //    unsafe { TODO: call ffi:mx_list_view_get_factory() }
//     //}

//     fn get_item_type(&self) -> glib::types::Type {
//         unsafe {
//             from_glib(ffi::mx_list_view_get_item_type(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_model(&self) -> Option<clutter::Model> {
//         unsafe {
//             from_glib_none(ffi::mx_list_view_get_model(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn set_factory(&self, factory: /*Ignored*/Option<&ItemFactory>) {
//     //    unsafe { TODO: call ffi:mx_list_view_set_factory() }
//     //}

//     fn set_item_type(&self, item_type: glib::types::Type) {
//         unsafe {
//             ffi::mx_list_view_set_item_type(self.as_ref().to_glib_none().0, item_type.to_glib());
//         }
//     }

//     fn set_model<P: IsA<clutter::Model>>(&self, model: &P) {
//         unsafe {
//             ffi::mx_list_view_set_model(
//                 self.as_ref().to_glib_none().0,
//                 model.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn thaw(&self) {
//         unsafe {
//             ffi::mx_list_view_thaw(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxListView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ListView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::factory\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_factory_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_item_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_item_type_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxListView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ListView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::item-type\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_item_type_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxListView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ListView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ListView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::model\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_model_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for ListView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListView")
    }
}
