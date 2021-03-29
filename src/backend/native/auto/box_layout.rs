// use clutter;
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
// use super::Widget;

// glib_wrapper! {
//     pub struct BoxLayout(Object<ffi::MxBoxLayout, ffi::MxBoxLayoutClass, BoxLayoutClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_box_layout_get_type(),
//     }
// }

pub struct BoxLayout {

}

impl BoxLayout {
    pub fn new() -> BoxLayout {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_box_layout_new()).unsafe_cast() }
        unimplemented!()
    }

    //pub fn with_orientation(orientation: /*Ignored*/Orientation) -> BoxLayout {
    //    unsafe { TODO: call ffi:mx_box_layout_new_with_orientation() }
    //}
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BOX_LAYOUT: Option<&BoxLayout> = None;

// pub trait BoxLayoutExt: 'static {
//     fn child_get_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     //fn child_get_x_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

//     fn child_get_x_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     //fn child_get_y_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

//     fn child_get_y_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     fn child_set_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool);

//     //fn child_set_x_align<P: IsA<clutter::Actor>>(&self, child: &P, x_align: /*Ignored*/Align);

//     fn child_set_x_fill<P: IsA<clutter::Actor>>(&self, child: &P, x_fill: bool);

//     //fn child_set_y_align<P: IsA<clutter::Actor>>(&self, child: &P, y_align: /*Ignored*/Align);

//     fn child_set_y_fill<P: IsA<clutter::Actor>>(&self, child: &P, y_fill: bool);

//     fn get_enable_animations(&self) -> bool;

//     //fn get_orientation(&self) -> /*Ignored*/Orientation;

//     fn get_scroll_to_focused(&self) -> bool;

//     fn get_spacing(&self) -> u32;

//     fn insert_actor<P: IsA<clutter::Actor>>(&self, actor: &P, position: i32);

//     //fn insert_actor_with_properties<P: IsA<clutter::Actor>>(&self, actor: &P, position: i32, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

//     fn set_enable_animations(&self, enable_animations: bool);

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation);

//     fn set_scroll_to_focused(&self, scroll_to_focused: bool);

//     fn set_spacing(&self, spacing: u32);

//     fn connect_property_enable_animations_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_scroll_to_focused_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<BoxLayout>> BoxLayoutExt for O {
//     fn child_get_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_box_layout_child_get_expand(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     //fn child_get_x_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_box_layout_child_get_x_align() }
//     //}

//     fn child_get_x_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_box_layout_child_get_x_fill(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     //fn child_get_y_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_box_layout_child_get_y_align() }
//     //}

//     fn child_get_y_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_box_layout_child_get_y_fill(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn child_set_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool) {
//         // unsafe {
//         //     ffi::mx_box_layout_child_set_expand(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //         expand.to_glib(),
//         //     );
//         // }
//         unimplemented!()
//     }

//     //fn child_set_x_align<P: IsA<clutter::Actor>>(&self, child: &P, x_align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_box_layout_child_set_x_align() }
//     //}

//     fn child_set_x_fill<P: IsA<clutter::Actor>>(&self, child: &P, x_fill: bool) {
//         // unsafe {
//         //     ffi::mx_box_layout_child_set_x_fill(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //         x_fill.to_glib(),
//         //     );
//         // }
//         unimplemented!()
//     }

//     //fn child_set_y_align<P: IsA<clutter::Actor>>(&self, child: &P, y_align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_box_layout_child_set_y_align() }
//     //}

//     fn child_set_y_fill<P: IsA<clutter::Actor>>(&self, child: &P, y_fill: bool) {
//         // unsafe {
//         //     ffi::mx_box_layout_child_set_y_fill(
//         //         self.as_ref().to_glib_none().0,
//         //         child.as_ref().to_glib_none().0,
//         //         y_fill.to_glib(),
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn get_enable_animations(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_box_layout_get_enable_animations(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     //fn get_orientation(&self) -> /*Ignored*/Orientation {
//     //    unsafe { TODO: call ffi:mx_box_layout_get_orientation() }
//     //}

//     fn get_scroll_to_focused(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_box_layout_get_scroll_to_focused(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_spacing(&self) -> u32 {
//         // unsafe { ffi::mx_box_layout_get_spacing(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn insert_actor<P: IsA<clutter::Actor>>(&self, actor: &P, position: i32) {
//         // unsafe {
//         //     ffi::mx_box_layout_insert_actor(
//         //         self.as_ref().to_glib_none().0,
//         //         actor.as_ref().to_glib_none().0,
//         //         position,
//         //     );
//         // }
//         unimplemented!()
//     }

//     //fn insert_actor_with_properties<P: IsA<clutter::Actor>>(&self, actor: &P, position: i32, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//     //    unsafe { TODO: call ffi:mx_box_layout_insert_actor_with_properties() }
//     //}

//     fn set_enable_animations(&self, enable_animations: bool) {
//         // unsafe {
//         //     ffi::mx_box_layout_set_enable_animations(
//         //         self.as_ref().to_glib_none().0,
//         //         enable_animations.to_glib(),
//         //     );
//         // }
//         unimplemented!()
//     }

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation) {
//     //    unsafe { TODO: call ffi:mx_box_layout_set_orientation() }
//     //}

//     fn set_scroll_to_focused(&self, scroll_to_focused: bool) {
//         // unsafe {
//         //     ffi::mx_box_layout_set_scroll_to_focused(
//         //         self.as_ref().to_glib_none().0,
//         //         scroll_to_focused.to_glib(),
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_spacing(&self, spacing: u32) {
//         // unsafe {
//         //     ffi::mx_box_layout_set_spacing(self.as_ref().to_glib_none().0, spacing);
//         // }
//         unimplemented!()
//     }

//     fn connect_property_enable_animations_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_enable_animations_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxBoxLayout,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<BoxLayout>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::enable-animations\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_enable_animations_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxBoxLayout,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<BoxLayout>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::orientation\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_orientation_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_scroll_to_focused_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_scroll_to_focused_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxBoxLayout,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<BoxLayout>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::scroll-to-focused\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_scroll_to_focused_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxBoxLayout,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<BoxLayout>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::spacing\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_spacing_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxLayout")
    }
}
