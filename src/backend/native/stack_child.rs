#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct StackChild {}

impl Object for StackChild {}
impl Is<StackChild> for StackChild {}

impl AsRef<StackChild> for StackChild {
    fn as_ref(&self) -> &StackChild {
        self
    }
}

pub const NONE_STACK_CHILD: Option<&StackChild> = None;

pub trait StackChildExt: 'static {
    fn get_property_crop(&self) -> bool;

    fn set_property_crop(&self, crop: bool);

    fn get_property_fit(&self) -> bool;

    fn set_property_fit(&self, fit: bool);

    //fn get_property_x_align(&self) -> /*Ignored*/Align;

    //fn set_property_x_align(&self, x_align: /*Ignored*/Align);

    fn get_property_x_fill(&self) -> bool;

    fn set_property_x_fill(&self, x_fill: bool);

    //fn get_property_y_align(&self) -> /*Ignored*/Align;

    //fn set_property_y_align(&self, y_align: /*Ignored*/Align);

    fn get_property_y_fill(&self) -> bool;

    fn set_property_y_fill(&self, y_fill: bool);

    fn connect_property_crop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<StackChild>> StackChildExt for O {
    fn get_property_crop(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"crop\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `crop` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_crop(&self, crop: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"crop\0".as_ptr() as *const _,
        //         Value::from(&crop).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_fit(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fit\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `fit` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_fit(&self, fit: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fit\0".as_ptr() as *const _,
        //         Value::from(&fit).to_glib_none().0,
        //     );
        // }
    }

    //fn get_property_x_align(&self) -> /*Ignored*/Align {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"x-align\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `x-align` getter").unwrap()
    //    }
    //}

    //fn set_property_x_align(&self, x_align: /*Ignored*/Align) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"x-align\0".as_ptr() as *const _, Value::from(&x_align).to_glib_none().0);
    //    }
    //}

    fn get_property_x_fill(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-fill\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `x-fill` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_x_fill(&self, x_fill: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-fill\0".as_ptr() as *const _,
        //         Value::from(&x_fill).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    //fn get_property_y_align(&self) -> /*Ignored*/Align {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"y-align\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `y-align` getter").unwrap()
    //    }
    //}

    //fn set_property_y_align(&self, y_align: /*Ignored*/Align) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"y-align\0".as_ptr() as *const _, Value::from(&y_align).to_glib_none().0);
    //    }
    //}

    fn get_property_y_fill(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-fill\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `y-fill` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_y_fill(&self, y_fill: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-fill\0".as_ptr() as *const _,
        //         Value::from(&y_fill).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_crop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_crop_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::crop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_crop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_fit_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fit\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fit_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_x_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_y_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for StackChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackChild")
    }
}
