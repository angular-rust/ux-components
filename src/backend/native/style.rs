#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;
// use std::ptr;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Style {}

impl Style {
    pub fn new() -> Style {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::style_new()) }
        unimplemented!()
    }

    pub fn get_default() -> Option<Style> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::style_get_default()) }
        unimplemented!()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Style {}
impl Is<Style> for Style {}

impl AsRef<Style> for Style {
    fn as_ref(&self) -> &Style {
        self
    }
}

pub const NONE_STYLE: Option<&Style> = None;

pub trait StyleExt: 'static {
    //fn get(&self, stylable: /*Ignored*/&Stylable, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    //fn get_property(&self, stylable: /*Ignored*/&Stylable, pspec: /*Ignored*/&glib::ParamSpec) -> glib::Value;

    //fn get_valist(&self, stylable: /*Ignored*/&Stylable, first_property_name: &str, va_args: /*Unknown conversion*/Unsupported);

    fn load_from_data(&self, id: &str, data: &str) -> Result<(), glib::Error>;

    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error>;

    fn load_from_resource(&self, path: &str) -> Result<(), glib::Error>;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Style>> StyleExt for O {
    //fn get(&self, stylable: /*Ignored*/&Stylable, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:style_get() }
    //}

    //fn get_property(&self, stylable: /*Ignored*/&Stylable, pspec: /*Ignored*/&glib::ParamSpec) -> glib::Value {
    //    unsafe { TODO: call ffi:style_get_property() }
    //}

    //fn get_valist(&self, stylable: /*Ignored*/&Stylable, first_property_name: &str, va_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi:style_get_valist() }
    //}

    fn load_from_data(&self, id: &str, data: &str) -> Result<(), glib::Error> {
        // unsafe {
        //     let mut error = ptr::null_mut();
        //     let _ = ffi::style_load_from_data(
        //         self.as_ref().to_glib_none().0,
        //         id.to_glib_none().0,
        //         data.to_glib_none().0,
        //         &mut error,
        //     );
        //     if error.is_null() {
        //         Ok(())
        //     } else {
        //         Err(from_glib_full(error))
        //     }
        // }
        unimplemented!()
    }

    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error> {
        // unsafe {
        //     let mut error = ptr::null_mut();
        //     let _ = ffi::style_load_from_file(
        //         self.as_ref().to_glib_none().0,
        //         filename.to_glib_none().0,
        //         &mut error,
        //     );
        //     if error.is_null() {
        //         Ok(())
        //     } else {
        //         Err(from_glib_full(error))
        //     }
        // }
        unimplemented!()
    }

    fn load_from_resource(&self, path: &str) -> Result<(), glib::Error> {
        // unsafe {
        //     let mut error = ptr::null_mut();
        //     let _ = ffi::style_load_from_resource(
        //         self.as_ref().to_glib_none().0,
        //         path.to_glib_none().0,
        //         &mut error,
        //     );
        //     if error.is_null() {
        //         Ok(())
        //     } else {
        //         Err(from_glib_full(error))
        //     }
        // }
        unimplemented!()
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Style,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Style>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Style::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"changed\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             changed_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Style")
    }
}
