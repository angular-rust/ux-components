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
// use super::Action;
// use super::Widget;

// glib_wrapper! {
//     pub struct Button(Object<ffi::MxButton, ffi::MxButtonClass, ButtonClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_button_get_type(),
//     }
// }

pub struct Button {

}

impl Button {
    pub fn new() -> Button {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_button_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_label(text: &str) -> Button {
        // assert_initialized_main_thread!();
        // unsafe {
        //     clutter::Actor::from_glib_none(ffi::mx_button_new_with_label(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BUTTON: Option<&Button> = None;

// pub trait ButtonExt: 'static {
//     fn get_action(&self) -> Option<Action>;

//     fn get_icon_name(&self) -> Option<GString>;

//     //fn get_icon_position(&self) -> /*Ignored*/Position;

//     fn get_icon_size(&self) -> u32;

//     fn get_icon_visible(&self) -> bool;

//     fn get_is_toggle(&self) -> bool;

//     fn get_label(&self) -> Option<GString>;

//     fn get_label_visible(&self) -> bool;

//     fn get_toggled(&self) -> bool;

//     fn set_action<P: IsA<Action>>(&self, action: &P);

//     fn set_icon_name(&self, icon_name: Option<&str>);

//     //fn set_icon_position(&self, position: /*Ignored*/Position);

//     fn set_icon_size(&self, icon_size: u32);

//     fn set_icon_visible(&self, visible: bool);

//     fn set_is_toggle(&self, toggle: bool);

//     fn set_label(&self, text: &str);

//     fn set_label_visible(&self, visible: bool);

//     fn set_toggled(&self, toggled: bool);

//     fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Button>> ButtonExt for O {
//     fn get_action(&self) -> Option<Action> {
//         // unsafe { from_glib_none(ffi::mx_button_get_action(self.as_ref().to_glib_none().0)) }
//         unimplemented!()
//     }

//     fn get_icon_name(&self) -> Option<GString> {
//         // unsafe {
//         //     from_glib_none(ffi::mx_button_get_icon_name(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     //fn get_icon_position(&self) -> /*Ignored*/Position {
//     //    unsafe { TODO: call ffi:mx_button_get_icon_position() }
//     //}

//     fn get_icon_size(&self) -> u32 {
//         // unsafe { ffi::mx_button_get_icon_size(self.as_ref().to_glib_none().0) }
//         unimplemented!()
//     }

//     fn get_icon_visible(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_button_get_icon_visible(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_is_toggle(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_button_get_is_toggle(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_label(&self) -> Option<GString> {
//         // unsafe { from_glib_none(ffi::mx_button_get_label(self.as_ref().to_glib_none().0)) }
//         unimplemented!()
//     }

//     fn get_label_visible(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_button_get_label_visible(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_toggled(&self) -> bool {
//         // unsafe {
//         //     from_glib(ffi::mx_button_get_toggled(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn set_action<P: IsA<Action>>(&self, action: &P) {
//         // unsafe {
//         //     ffi::mx_button_set_action(
//         //         self.as_ref().to_glib_none().0,
//         //         action.as_ref().to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_icon_name(&self, icon_name: Option<&str>) {
//         // unsafe {
//         //     ffi::mx_button_set_icon_name(
//         //         self.as_ref().to_glib_none().0,
//         //         icon_name.to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }

//     //fn set_icon_position(&self, position: /*Ignored*/Position) {
//     //    unsafe { TODO: call ffi:mx_button_set_icon_position() }
//     //}

//     fn set_icon_size(&self, icon_size: u32) {
//         // unsafe {
//         //     ffi::mx_button_set_icon_size(self.as_ref().to_glib_none().0, icon_size);
//         // }
//         unimplemented!()
//     }

//     fn set_icon_visible(&self, visible: bool) {
//         // unsafe {
//         //     ffi::mx_button_set_icon_visible(self.as_ref().to_glib_none().0, visible.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_is_toggle(&self, toggle: bool) {
//         // unsafe {
//         //     ffi::mx_button_set_is_toggle(self.as_ref().to_glib_none().0, toggle.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_label(&self, text: &str) {
//         // unsafe {
//         //     ffi::mx_button_set_label(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         // }
//         unimplemented!()
//     }

//     fn set_label_visible(&self, visible: bool) {
//         // unsafe {
//         //     ffi::mx_button_set_label_visible(self.as_ref().to_glib_none().0, visible.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_toggled(&self, toggled: bool) {
//         // unsafe {
//         //     ffi::mx_button_set_toggled(self.as_ref().to_glib_none().0, toggled.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"clicked\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             clicked_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::action\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_action_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::icon-name\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_icon_name_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_icon_position_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::icon-position\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_icon_position_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::icon-size\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_icon_size_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_icon_visible_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::icon-visible\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_icon_visible_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_is_toggle_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::is-toggle\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_is_toggle_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::label\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_label_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_label_visible_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::label-visible\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_label_visible_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_toggled_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::MxButton,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: IsA<Button>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::toggled\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_toggled_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button")
    }
}
