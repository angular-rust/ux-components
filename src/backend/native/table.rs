#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Table {}

impl Table {
    pub fn new() -> Table {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::table_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Table {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::table_new()) }
    // }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl UxObject for Table {}
impl Is<Table> for Table {}

impl AsRef<Table> for Table {
    fn as_ref(&self) -> &Table {
        unimplemented!()
    }
}

pub const NONE_TABLE: Option<&Table> = None;

pub trait TableExt: 'static {
    fn child_get_column<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_column_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_row<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_row_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    //fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

    fn child_get_x_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    //fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

    fn child_get_y_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_set_column<P: Is<clutter::Actor>>(&self, child: &P, col: i32);

    fn child_set_column_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32);

    fn child_set_row<P: Is<clutter::Actor>>(&self, child: &P, row: i32);

    fn child_set_row_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32);

    //fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align);

    fn child_set_x_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool);

    fn child_set_x_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool);

    //fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align);

    fn child_set_y_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool);

    fn child_set_y_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool);

    fn get_actor_at(&self, row: i32, column: i32) -> Option<clutter::Actor>;

    fn get_column_count(&self) -> i32;

    fn get_column_spacing(&self) -> i32;

    fn get_row_count(&self) -> i32;

    fn get_row_spacing(&self) -> i32;

    fn insert_actor<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32);

    //fn insert_actor_with_properties<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_column_spacing(&self, spacing: i32);

    fn set_row_spacing(&self, spacing: i32);

    fn connect_property_column_count_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_row_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Table>> TableExt for O {
    fn child_get_column<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        // unsafe {
        //     ffi::table_child_get_column(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_column_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        // unsafe {
        //     ffi::table_child_get_column_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_row<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        // unsafe {
        //     ffi::table_child_get_row(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_row_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        // unsafe {
        //     ffi::table_child_get_row_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    //fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
    //    unsafe { TODO: call ffi:table_child_get_x_align() }
    //}

    fn child_get_x_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::table_child_get_x_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::table_child_get_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    //fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
    //    unsafe { TODO: call ffi:table_child_get_y_align() }
    //}

    fn child_get_y_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::table_child_get_y_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::table_child_get_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_set_column<P: Is<clutter::Actor>>(&self, child: &P, col: i32) {
        // unsafe {
        //     ffi::table_child_set_column(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         col,
        //     );
        // }
        unimplemented!()
    }

    fn child_set_column_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32) {
        // unsafe {
        //     ffi::table_child_set_column_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         span,
        //     );
        // }
        unimplemented!()
    }

    fn child_set_row<P: Is<clutter::Actor>>(&self, child: &P, row: i32) {
        // unsafe {
        //     ffi::table_child_set_row(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         row,
        //     );
        // }
        unimplemented!()
    }

    fn child_set_row_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32) {
        // unsafe {
        //     ffi::table_child_set_row_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         span,
        //     );
        // }
        unimplemented!()
    }

    //fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align) {
    //    unsafe { TODO: call ffi:table_child_set_x_align() }
    //}

    fn child_set_x_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool) {
        // unsafe {
        //     ffi::table_child_set_x_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         expand.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_x_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool) {
        // unsafe {
        //     ffi::table_child_set_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    //fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align) {
    //    unsafe { TODO: call ffi:table_child_set_y_align() }
    //}

    fn child_set_y_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool) {
        // unsafe {
        //     ffi::table_child_set_y_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         expand.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_y_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool) {
        // unsafe {
        //     ffi::table_child_set_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn get_actor_at(&self, row: i32, column: i32) -> Option<clutter::Actor> {
        // unsafe {
        //     from_glib_none(ffi::table_get_actor_at(
        //         self.as_ref().to_glib_none().0,
        //         row,
        //         column,
        //     ))
        // }
        unimplemented!()
    }

    fn get_column_count(&self) -> i32 {
        // unsafe { ffi::table_get_column_count(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_column_spacing(&self) -> i32 {
        // unsafe { ffi::table_get_column_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_row_count(&self) -> i32 {
        // unsafe { ffi::table_get_row_count(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_row_spacing(&self) -> i32 {
        // unsafe { ffi::table_get_row_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn insert_actor<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32) {
        // unsafe {
        //     ffi::table_insert_actor(
        //         self.as_ref().to_glib_none().0,
        //         actor.as_ref().to_glib_none().0,
        //         row,
        //         column,
        //     );
        // }
        unimplemented!()
    }

    //fn insert_actor_with_properties<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:table_insert_actor_with_properties() }
    //}

    fn set_column_spacing(&self, spacing: i32) {
        // unsafe {
        //     ffi::table_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
        unimplemented!()
    }

    fn set_row_spacing(&self, spacing: i32) {
        // unsafe {
        //     ffi::table_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
        unimplemented!()
    }

    fn connect_property_column_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_column_count_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Table,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Table>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Table::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::column-count\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_column_count_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Table,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Table>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Table::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::column-spacing\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_column_spacing_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_row_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_row_count_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Table,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Table>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Table::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::row-count\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_row_count_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Table,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Table>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Table::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::row-spacing\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_row_spacing_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table")
    }
}
