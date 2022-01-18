#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Align, HandlerId};
use std::fmt;

#[derive(Clone, Debug)]
pub struct TableChild {}

impl Object for TableChild {}
impl Is<TableChild> for TableChild {}

impl AsRef<TableChild> for TableChild {
    fn as_ref(&self) -> &TableChild {
        self
    }
}

pub trait TableChildExt: 'static {
    fn get_property_column(&self) -> i32;

    fn set_property_column(&self, column: i32);

    fn get_property_column_span(&self) -> i32;

    fn set_property_column_span(&self, column_span: i32);

    fn get_property_row(&self) -> i32;

    fn set_property_row(&self, row: i32);

    fn get_property_row_span(&self) -> i32;

    fn set_property_row_span(&self, row_span: i32);

    fn get_property_x_align(&self) -> Align;

    fn set_property_x_align(&self, x_align: Align);

    fn get_property_x_expand(&self) -> bool;

    fn set_property_x_expand(&self, x_expand: bool);

    fn get_property_x_fill(&self) -> bool;

    fn set_property_x_fill(&self, x_fill: bool);

    fn get_property_y_align(&self) -> Align;

    fn set_property_y_align(&self, y_align: Align);

    fn get_property_y_expand(&self) -> bool;

    fn set_property_y_expand(&self, y_expand: bool);

    fn get_property_y_fill(&self) -> bool;

    fn set_property_y_fill(&self, y_fill: bool);

    fn connect_property_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_column_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_row_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<TableChild>> TableChildExt for O {
    fn get_property_column(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"column\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `column` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_column(&self, column: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"column\0".as_ptr() as *const _,
        //         Value::from(&column).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_column_span(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"column-span\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `column-span` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_column_span(&self, column_span: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"column-span\0".as_ptr() as *const _,
        //         Value::from(&column_span).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_row(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"row\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `row` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_row(&self, row: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"row\0".as_ptr() as *const _,
        //         Value::from(&row).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_row_span(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"row-span\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `row-span` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_row_span(&self, row_span: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"row-span\0".as_ptr() as *const _,
        //         Value::from(&row_span).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_x_align(&self) -> Align {
        //    unsafe {
        //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
        //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"x-align\0".as_ptr() as *const _, value.to_glib_none_mut().0);
        //        value.get().expect("Return Value for property `x-align` getter").unwrap()
        //    }
        unimplemented!()
    }

    fn set_property_x_align(&self, x_align: Align) {
        //    unsafe {
        //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"x-align\0".as_ptr() as *const _, Value::from(&x_align).to_glib_none().0);
        //    }
        unimplemented!()
    }

    fn get_property_x_expand(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-expand\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `x-expand` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_x_expand(&self, x_expand: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"x-expand\0".as_ptr() as *const _,
        //         Value::from(&x_expand).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

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

    fn get_property_y_align(&self) -> Align {
        //    unsafe {
        //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
        //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"y-align\0".as_ptr() as *const _, value.to_glib_none_mut().0);
        //        value.get().expect("Return Value for property `y-align` getter").unwrap()
        //    }
        unimplemented!()
    }

    fn set_property_y_align(&self, y_align: Align) {
        //    unsafe {
        //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"y-align\0".as_ptr() as *const _, Value::from(&y_align).to_glib_none().0);
        //    }
        unimplemented!()
    }

    fn get_property_y_expand(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-expand\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `y-expand` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_y_expand(&self, y_expand: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"y-expand\0".as_ptr() as *const _,
        //         Value::from(&y_expand).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

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

    fn connect_property_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_column_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::column\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_column_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_column_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_column_span_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::column-span\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_column_span_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_row_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::row\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_row_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_row_span_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_row_span_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::row-span\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_row_span_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_expand_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-expand\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_expand_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_expand_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-expand\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_expand_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::TableChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<TableChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&TableChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for TableChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TableChild")
    }
}
