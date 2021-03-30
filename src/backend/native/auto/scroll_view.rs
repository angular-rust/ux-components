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
//     pub struct ScrollView(Object<ffi::ScrollView, ffi::ScrollViewClass, ScrollViewClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::scroll_view_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct ScrollView {

}

impl ScrollView {
    pub fn new() -> ScrollView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::scroll_view_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SCROLL_VIEW: Option<&ScrollView> = None;

// pub trait ScrollViewExt: 'static {
//     fn ensure_visible(&self, geometry: &clutter::Geometry);

//     fn get_enable_mouse_scrolling(&self) -> bool;

//     //fn get_scroll_policy(&self) -> /*Ignored*/ScrollPolicy;

//     //fn get_scroll_visibility(&self) -> /*Ignored*/ScrollPolicy;

//     fn set_enable_mouse_scrolling(&self, enabled: bool);

//     //fn set_scroll_policy(&self, policy: /*Ignored*/ScrollPolicy);

//     //fn set_scroll_visibility(&self, visibility: /*Ignored*/ScrollPolicy);

//     fn connect_property_enable_mouse_scrolling_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_scroll_visibility_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;
// }

// impl<O: IsA<ScrollView>> ScrollViewExt for O {
//     fn ensure_visible(&self, geometry: &clutter::Geometry) {
//         unsafe {
//             ffi::scroll_view_ensure_visible(
//                 self.as_ref().to_glib_none().0,
//                 geometry.to_glib_none().0,
//             );
//         }
//     }

//     fn get_enable_mouse_scrolling(&self) -> bool {
//         unsafe {
//             from_glib(ffi::scroll_view_get_enable_mouse_scrolling(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn get_scroll_policy(&self) -> /*Ignored*/ScrollPolicy {
//     //    unsafe { TODO: call ffi:mx_scroll_view_get_scroll_policy() }
//     //}

//     //fn get_scroll_visibility(&self) -> /*Ignored*/ScrollPolicy {
//     //    unsafe { TODO: call ffi:mx_scroll_view_get_scroll_visibility() }
//     //}

//     fn set_enable_mouse_scrolling(&self, enabled: bool) {
//         unsafe {
//             ffi::scroll_view_set_enable_mouse_scrolling(
//                 self.as_ref().to_glib_none().0,
//                 enabled.to_glib(),
//             );
//         }
//     }

//     //fn set_scroll_policy(&self, policy: /*Ignored*/ScrollPolicy) {
//     //    unsafe { TODO: call ffi:mx_scroll_view_set_scroll_policy() }
//     //}

//     //fn set_scroll_visibility(&self, visibility: /*Ignored*/ScrollPolicy) {
//     //    unsafe { TODO: call ffi:mx_scroll_view_set_scroll_visibility() }
//     //}

//     fn connect_property_enable_mouse_scrolling_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_enable_mouse_scrolling_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::enable-mouse-scrolling\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_enable_mouse_scrolling_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_scroll_policy_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::scroll-policy\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_scroll_policy_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_scroll_visibility_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_scroll_visibility_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<ScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::scroll-visibility\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_scroll_visibility_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for ScrollView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollView")
    }
}
