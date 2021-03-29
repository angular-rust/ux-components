// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib::StaticType;
// use glib::Value;
// use glib_sys;
// use gobject_sys;
// use libc;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use std::ptr;
// use Widget;

// glib_wrapper! {
//     pub struct KineticScrollView(Object<ffi::MxKineticScrollView, ffi::MxKineticScrollViewClass, KineticScrollViewClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_kinetic_scroll_view_get_type(),
//     }
// }

pub struct KineticScrollView {

}

impl KineticScrollView {
    pub fn new() -> KineticScrollView {
        // assert_initialized_main_thread!();
        // unsafe {
        //     clutter::Actor::from_glib_none(ffi::mx_kinetic_scroll_view_new()).unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for KineticScrollView {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_KINETIC_SCROLL_VIEW: Option<&KineticScrollView> = None;

// pub trait KineticScrollViewExt: 'static {
//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn ensure_visible(&self, geometry: &clutter::Geometry);

//     fn get_acceleration_factor(&self) -> f64;

//     fn get_clamp_duration(&self) -> u32;

//     fn get_clamp_mode(&self) -> libc::c_ulong;

//     fn get_clamp_to_center(&self) -> bool;

//     fn get_deceleration(&self) -> f64;

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_input(&self) -> (clutter::InputDevice, clutter::EventSequence);

//     fn get_mouse_button(&self) -> u32;

//     fn get_overshoot(&self) -> f64;

//     //fn get_scroll_policy(&self) -> /*Ignored*/ScrollPolicy;

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_snap_on_page(&self) -> bool;

//     fn get_use_captured(&self) -> bool;

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_use_grab(&self) -> bool;

//     fn set_acceleration_factor(&self, acceleration_factor: f64);

//     fn set_clamp_duration(&self, clamp_duration: u32);

//     fn set_clamp_mode(&self, clamp_mode: libc::c_ulong);

//     fn set_clamp_to_center(&self, clamp_to_center: bool);

//     fn set_deceleration(&self, rate: f64);

//     fn set_mouse_button(&self, button: u32);

//     fn set_overshoot(&self, overshoot: f64);

//     //fn set_scroll_policy(&self, policy: /*Ignored*/ScrollPolicy);

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn set_snap_on_page(&self, snap_on_page: bool);

//     fn set_use_captured(&self, use_captured: bool);

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn set_use_grab(&self, use_grab: bool);

//     fn stop(&self);

//     fn get_property_snap_on_page(&self) -> bool;

//     fn set_property_snap_on_page(&self, snap_on_page: bool);

//     //fn get_property_state(&self) -> /*Ignored*/KineticScrollViewState;

//     fn get_property_use_grab(&self) -> bool;

//     fn set_property_use_grab(&self, use_grab: bool);

//     fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_clamp_duration_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_clamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_clamp_to_center_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_overshoot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_snap_on_page_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_use_captured_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_use_grab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<KineticScrollView>> KineticScrollViewExt for O {
//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn ensure_visible(&self, geometry: &clutter::Geometry) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_ensure_visible(
//                 self.as_ref().to_glib_none().0,
//                 geometry.to_glib_none().0,
//             );
//         }
//     }

//     fn get_acceleration_factor(&self) -> f64 {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_get_acceleration_factor(self.as_ref().to_glib_none().0)
//         }
//     }

//     fn get_clamp_duration(&self) -> u32 {
//         unsafe { ffi::mx_kinetic_scroll_view_get_clamp_duration(self.as_ref().to_glib_none().0) }
//     }

//     fn get_clamp_mode(&self) -> libc::c_ulong {
//         unsafe { ffi::mx_kinetic_scroll_view_get_clamp_mode(self.as_ref().to_glib_none().0) }
//     }

//     fn get_clamp_to_center(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_kinetic_scroll_view_get_clamp_to_center(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_deceleration(&self) -> f64 {
//         unsafe { ffi::mx_kinetic_scroll_view_get_deceleration(self.as_ref().to_glib_none().0) }
//     }

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_input(&self) -> (clutter::InputDevice, clutter::EventSequence) {
//         unsafe {
//             let mut device = ptr::null_mut();
//             let mut sequence = ptr::null_mut();
//             ffi::mx_kinetic_scroll_view_get_input(
//                 self.as_ref().to_glib_none().0,
//                 &mut device,
//                 &mut sequence,
//             );
//             (from_glib_none(device), from_glib_none(sequence))
//         }
//     }

//     fn get_mouse_button(&self) -> u32 {
//         unsafe { ffi::mx_kinetic_scroll_view_get_mouse_button(self.as_ref().to_glib_none().0) }
//     }

//     fn get_overshoot(&self) -> f64 {
//         unsafe { ffi::mx_kinetic_scroll_view_get_overshoot(self.as_ref().to_glib_none().0) }
//     }

//     //fn get_scroll_policy(&self) -> /*Ignored*/ScrollPolicy {
//     //    unsafe { TODO: call ffi:mx_kinetic_scroll_view_get_scroll_policy() }
//     //}

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_snap_on_page(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_kinetic_scroll_view_get_snap_on_page(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_use_captured(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_kinetic_scroll_view_get_use_captured(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn get_use_grab(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_kinetic_scroll_view_get_use_grab(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn set_acceleration_factor(&self, acceleration_factor: f64) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_acceleration_factor(
//                 self.as_ref().to_glib_none().0,
//                 acceleration_factor,
//             );
//         }
//     }

//     fn set_clamp_duration(&self, clamp_duration: u32) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_clamp_duration(
//                 self.as_ref().to_glib_none().0,
//                 clamp_duration,
//             );
//         }
//     }

//     fn set_clamp_mode(&self, clamp_mode: libc::c_ulong) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_clamp_mode(
//                 self.as_ref().to_glib_none().0,
//                 clamp_mode,
//             );
//         }
//     }

//     fn set_clamp_to_center(&self, clamp_to_center: bool) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_clamp_to_center(
//                 self.as_ref().to_glib_none().0,
//                 clamp_to_center.to_glib(),
//             );
//         }
//     }

//     fn set_deceleration(&self, rate: f64) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_deceleration(self.as_ref().to_glib_none().0, rate);
//         }
//     }

//     fn set_mouse_button(&self, button: u32) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_mouse_button(self.as_ref().to_glib_none().0, button);
//         }
//     }

//     fn set_overshoot(&self, overshoot: f64) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_overshoot(self.as_ref().to_glib_none().0, overshoot);
//         }
//     }

//     //fn set_scroll_policy(&self, policy: /*Ignored*/ScrollPolicy) {
//     //    unsafe { TODO: call ffi:mx_kinetic_scroll_view_set_scroll_policy() }
//     //}

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn set_snap_on_page(&self, snap_on_page: bool) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_snap_on_page(
//                 self.as_ref().to_glib_none().0,
//                 snap_on_page.to_glib(),
//             );
//         }
//     }

//     fn set_use_captured(&self, use_captured: bool) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_use_captured(
//                 self.as_ref().to_glib_none().0,
//                 use_captured.to_glib(),
//             );
//         }
//     }

//     #[cfg(any(feature = "v2_0", feature = "dox"))]
//     fn set_use_grab(&self, use_grab: bool) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_set_use_grab(
//                 self.as_ref().to_glib_none().0,
//                 use_grab.to_glib(),
//             );
//         }
//     }

//     fn stop(&self) {
//         unsafe {
//             ffi::mx_kinetic_scroll_view_stop(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn get_property_snap_on_page(&self) -> bool {
//         unsafe {
//             let mut value = Value::from_type(<bool as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"snap-on-page\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `snap-on-page` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_snap_on_page(&self, snap_on_page: bool) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"snap-on-page\0".as_ptr() as *const _,
//                 Value::from(&snap_on_page).to_glib_none().0,
//             );
//         }
//     }

//     //fn get_property_state(&self) -> /*Ignored*/KineticScrollViewState {
//     //    unsafe {
//     //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
//     //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"state\0".as_ptr() as *const _, value.to_glib_none_mut().0);
//     //        value.get().expect("Return Value for property `state` getter").unwrap()
//     //    }
//     //}

//     fn get_property_use_grab(&self) -> bool {
//         unsafe {
//             let mut value = Value::from_type(<bool as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"use-grab\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `use-grab` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_use_grab(&self, use_grab: bool) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"use-grab\0".as_ptr() as *const _,
//                 Value::from(&use_grab).to_glib_none().0,
//             );
//         }
//     }

//     fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_acceleration_factor_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::acceleration-factor\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_acceleration_factor_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_clamp_duration_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clamp_duration_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::clamp-duration\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_clamp_duration_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_clamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clamp_mode_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::clamp-mode\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_clamp_mode_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_clamp_to_center_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_clamp_to_center_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::clamp-to-center\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_clamp_to_center_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_deceleration_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::deceleration\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_deceleration_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_mouse_button_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::mouse-button\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_mouse_button_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_overshoot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_overshoot_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::overshoot\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_overshoot_trampoline::<Self, F> as *const (),
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
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
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

//     fn connect_property_snap_on_page_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_snap_on_page_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::snap-on-page\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_snap_on_page_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::state\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_state_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_use_captured_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_use_captured_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::use-captured\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_use_captured_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_use_grab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_use_grab_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxKineticScrollView,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<KineticScrollView>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::use-grab\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_use_grab_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for KineticScrollView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KineticScrollView")
    }
}
