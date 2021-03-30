#![allow(unused_variables)]
#![allow(dead_code)]

// use std::mem::transmute;

use glib::signal::SignalHandlerId;
use std::boxed::Box as Box_;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Action {}

impl Action {
    pub fn new() -> Action {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::action_new()) }
        unimplemented!()
    }

    pub fn new_full(
        name: &str,
        display_name: &str,
        activated_cb: Option<Box_<dyn FnOnce(&Action) + 'static>>,
    ) -> Action {
        // assert_initialized_main_thread!();
        // let activated_cb_data: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> =
        //     Box_::new(activated_cb);
        // unsafe extern "C" fn activated_cb_func(
        //     action: *mut ffi::Action,
        //     user_data: glib_sys::gpointer,
        // ) {
        //     let action = from_glib_borrow(action);
        //     let callback: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> =
        //         Box_::from_raw(user_data as *mut _);
        //     let callback = (*callback).expect("cannot get closure...");
        //     callback(&action)
        // }
        // let activated_cb = if activated_cb_data.is_some() {
        //     Some(activated_cb_func as _)
        // } else {
        //     None
        // };
        // let super_callback0: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> = activated_cb_data;
        // unsafe {
        //     from_glib_full(ffi::action_new_full(
        //         name.to_glib_none().0,
        //         display_name.to_glib_none().0,
        //         activated_cb,
        //         Box_::into_raw(super_callback0) as *mut _,
        //     ))
        // }
        unimplemented!()
    }

    pub fn new_stateful(
        name: &str,
        parameter_type: Option<&glib::VariantTy>,
        state: &glib::Variant,
    ) -> Action {
        // assert_initialized_main_thread!();
        // unsafe {
        //     from_glib_none(ffi::action_new_stateful(
        //         name.to_glib_none().0,
        //         parameter_type.to_glib_none().0,
        //         state.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    pub fn with_parameter(name: &str, parameter_type: Option<&glib::VariantTy>) -> Action {
        // assert_initialized_main_thread!();
        // unsafe {
        //     from_glib_none(ffi::action_new_with_parameter(
        //         name.to_glib_none().0,
        //         parameter_type.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ACTION: Option<&Action> = None;

pub trait ActionExt: 'static {
    fn get_active(&self) -> bool;

    fn get_display_name(&self) -> Option<String>;

    fn get_icon(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn set_active(&self, active: bool);

    fn set_display_name(&self, name: &str);

    fn set_icon(&self, name: &str);

    fn set_name(&self, name: &str);

    fn connect_activate<F: Fn(&Self, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_display_name_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

// impl<O: Is<Action>> ActionExt for O {
//     fn get_active(&self) -> bool {
//         // unsafe { from_glib(ffi::action_get_active(self.as_ref().to_glib_none().0)) }
//         unimplemented!()
//     }

//     fn get_display_name(&self) -> Option<String> {
//         // unsafe {
//         //     from_glib_none(ffi::action_get_display_name(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_icon(&self) -> Option<String> {
//         // unsafe { from_glib_none(ffi::action_get_icon(self.as_ref().to_glib_none().0)) }
//         unimplemented!()
//     }

//     fn get_name(&self) -> Option<String> {
//         // unsafe { from_glib_none(ffi::action_get_name(self.as_ref().to_glib_none().0)) }
//         unimplemented!()
//     }

//     fn set_active(&self, active: bool) {
//         // unsafe {
//         //     ffi::action_set_active(self.as_ref().to_glib_none().0, active.to_glib());
//         // }
//         unimplemented!()
//     }

//     fn set_display_name(&self, name: &str) {
//         // unsafe {
//         //     ffi::action_set_display_name(
//         //         self.as_ref().to_glib_none().0,
//         //         name.to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_icon(&self, name: &str) {
//         // unsafe {
//         //     ffi::action_set_icon(self.as_ref().to_glib_none().0, name.to_glib_none().0);
//         // }
//         unimplemented!()
//     }

//     fn set_name(&self, name: &str) {
//         // unsafe {
//         //     ffi::action_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
//         // }
//         unimplemented!()
//     }

//     fn connect_activate<F: Fn(&Self, Option<&glib::Variant>) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn activate_trampoline<P, F: Fn(&P, Option<&glib::Variant>) + 'static>(
//         //     this: *mut ffi::Action,
//         //     parameter: *mut glib_sys::GVariant,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Action>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(
//         //         &Action::from_glib_borrow(this).unsafe_cast_ref(),
//         //         Option::<glib::Variant>::from_glib_borrow(parameter)
//         //             .as_ref()
//         //             .as_ref(),
//         //     )
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"activate\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             activate_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_display_name_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_display_name_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Action,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Action>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Action::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::display-name\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_display_name_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }

//     fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         // unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(
//         //     this: *mut ffi::Action,
//         //     _param_spec: glib_sys::gpointer,
//         //     f: glib_sys::gpointer,
//         // ) where
//         //     P: Is<Action>,
//         // {
//         //     let f: &F = &*(f as *const F);
//         //     f(&Action::from_glib_borrow(this).unsafe_cast_ref())
//         // }
//         // unsafe {
//         //     let f: Box_<F> = Box_::new(f);
//         //     connect_raw(
//         //         self.as_ptr() as *mut _,
//         //         b"notify::icon\0".as_ptr() as *const _,
//         //         Some(transmute::<_, unsafe extern "C" fn()>(
//         //             notify_icon_trampoline::<Self, F> as *const (),
//         //         )),
//         //         Box_::into_raw(f),
//         //     )
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
