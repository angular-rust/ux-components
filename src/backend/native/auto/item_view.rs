
// use glib::object::Cast;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;


// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Grid;
// use Widget;

// glib_wrapper! {
//     pub struct ItemView(Object<ffi::ItemView, ffi::ItemViewClass, ItemViewClass>) @extends Grid, Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::item_view_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct ItemView {

}

impl ItemView {
    pub fn new() -> ItemView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::item_view_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ItemView {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ITEM_VIEW: Option<&ItemView> = None;

// pub trait ItemViewExt: 'static {
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

// impl<O: IsA<ItemView>> ItemViewExt for O {
//     fn add_attribute(&self, attribute: &str, column: i32) {
//         unsafe {
//             ffi::item_view_add_attribute(
//                 self.as_ref().to_glib_none().0,
//                 attribute.to_glib_none().0,
//                 column,
//             );
//         }
//     }

//     fn freeze(&self) {
//         unsafe {
//             ffi::item_view_freeze(self.as_ref().to_glib_none().0);
//         }
//     }

//     //fn get_factory(&self) -> /*Ignored*/Option<ItemFactory> {
//     //    unsafe { TODO: call ffi:mx_item_view_get_factory() }
//     //}

//     fn get_item_type(&self) -> glib::types::Type {
//         unsafe {
//             from_glib(ffi::item_view_get_item_type(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_model(&self) -> Option<clutter::Model> {
//         unsafe {
//             from_glib_none(ffi::item_view_get_model(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn set_factory(&self, factory: /*Ignored*/Option<&ItemFactory>) {
//     //    unsafe { TODO: call ffi:mx_item_view_set_factory() }
//     //}

//     fn set_item_type(&self, item_type: glib::types::Type) {
//         unsafe {
//             ffi::item_view_set_item_type(self.as_ref().to_glib_none().0, item_type.to_glib());
//         }
//     }

//     fn set_model<P: IsA<clutter::Model>>(&self, model: &P) {
//         unsafe {
//             ffi::item_view_set_model(
//                 self.as_ref().to_glib_none().0,
//                 model.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn thaw(&self) {
//         unsafe {
//             ffi::item_view_thaw(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn connect_property_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_factory_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ItemView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ItemView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
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
//             this: *mut ffi::ItemView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ItemView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
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
//             this: *mut ffi::ItemView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ItemView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ItemView::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for ItemView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ItemView")
    }
}
