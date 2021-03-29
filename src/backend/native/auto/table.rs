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
// use Widget;

// glib_wrapper! {
//     pub struct Table(Object<ffi::MxTable, ffi::MxTableClass, TableClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_table_get_type(),
//     }
// }

pub struct Table {

}

impl Table {
    pub fn new() -> Table {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_table_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TABLE: Option<&Table> = None;

// pub trait TableExt: 'static {
//     fn child_get_column<P: IsA<clutter::Actor>>(&self, child: &P) -> i32;

//     fn child_get_column_span<P: IsA<clutter::Actor>>(&self, child: &P) -> i32;

//     fn child_get_row<P: IsA<clutter::Actor>>(&self, child: &P) -> i32;

//     fn child_get_row_span<P: IsA<clutter::Actor>>(&self, child: &P) -> i32;

//     //fn child_get_x_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

//     fn child_get_x_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     fn child_get_x_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     //fn child_get_y_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

//     fn child_get_y_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     fn child_get_y_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool;

//     fn child_set_column<P: IsA<clutter::Actor>>(&self, child: &P, col: i32);

//     fn child_set_column_span<P: IsA<clutter::Actor>>(&self, child: &P, span: i32);

//     fn child_set_row<P: IsA<clutter::Actor>>(&self, child: &P, row: i32);

//     fn child_set_row_span<P: IsA<clutter::Actor>>(&self, child: &P, span: i32);

//     //fn child_set_x_align<P: IsA<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align);

//     fn child_set_x_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool);

//     fn child_set_x_fill<P: IsA<clutter::Actor>>(&self, child: &P, fill: bool);

//     //fn child_set_y_align<P: IsA<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align);

//     fn child_set_y_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool);

//     fn child_set_y_fill<P: IsA<clutter::Actor>>(&self, child: &P, fill: bool);

//     fn get_actor_at(&self, row: i32, column: i32) -> Option<clutter::Actor>;

//     fn get_column_count(&self) -> i32;

//     fn get_column_spacing(&self) -> i32;

//     fn get_row_count(&self) -> i32;

//     fn get_row_spacing(&self) -> i32;

//     fn insert_actor<P: IsA<clutter::Actor>>(&self, actor: &P, row: i32, column: i32);

//     //fn insert_actor_with_properties<P: IsA<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

//     fn set_column_spacing(&self, spacing: i32);

//     fn set_row_spacing(&self, spacing: i32);

//     fn connect_property_column_count_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_row_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Table>> TableExt for O {
//     fn child_get_column<P: IsA<clutter::Actor>>(&self, child: &P) -> i32 {
//         unsafe {
//             ffi::mx_table_child_get_column(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             )
//         }
//     }

//     fn child_get_column_span<P: IsA<clutter::Actor>>(&self, child: &P) -> i32 {
//         unsafe {
//             ffi::mx_table_child_get_column_span(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             )
//         }
//     }

//     fn child_get_row<P: IsA<clutter::Actor>>(&self, child: &P) -> i32 {
//         unsafe {
//             ffi::mx_table_child_get_row(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             )
//         }
//     }

//     fn child_get_row_span<P: IsA<clutter::Actor>>(&self, child: &P) -> i32 {
//         unsafe {
//             ffi::mx_table_child_get_row_span(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             )
//         }
//     }

//     //fn child_get_x_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_table_child_get_x_align() }
//     //}

//     fn child_get_x_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::mx_table_child_get_x_expand(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn child_get_x_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::mx_table_child_get_x_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn child_get_y_align<P: IsA<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_table_child_get_y_align() }
//     //}

//     fn child_get_y_expand<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::mx_table_child_get_y_expand(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn child_get_y_fill<P: IsA<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::mx_table_child_get_y_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn child_set_column<P: IsA<clutter::Actor>>(&self, child: &P, col: i32) {
//         unsafe {
//             ffi::mx_table_child_set_column(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 col,
//             );
//         }
//     }

//     fn child_set_column_span<P: IsA<clutter::Actor>>(&self, child: &P, span: i32) {
//         unsafe {
//             ffi::mx_table_child_set_column_span(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 span,
//             );
//         }
//     }

//     fn child_set_row<P: IsA<clutter::Actor>>(&self, child: &P, row: i32) {
//         unsafe {
//             ffi::mx_table_child_set_row(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 row,
//             );
//         }
//     }

//     fn child_set_row_span<P: IsA<clutter::Actor>>(&self, child: &P, span: i32) {
//         unsafe {
//             ffi::mx_table_child_set_row_span(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 span,
//             );
//         }
//     }

//     //fn child_set_x_align<P: IsA<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_table_child_set_x_align() }
//     //}

//     fn child_set_x_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool) {
//         unsafe {
//             ffi::mx_table_child_set_x_expand(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 expand.to_glib(),
//             );
//         }
//     }

//     fn child_set_x_fill<P: IsA<clutter::Actor>>(&self, child: &P, fill: bool) {
//         unsafe {
//             ffi::mx_table_child_set_x_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 fill.to_glib(),
//             );
//         }
//     }

//     //fn child_set_y_align<P: IsA<clutter::Actor>>(&self, child: &P, align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_table_child_set_y_align() }
//     //}

//     fn child_set_y_expand<P: IsA<clutter::Actor>>(&self, child: &P, expand: bool) {
//         unsafe {
//             ffi::mx_table_child_set_y_expand(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 expand.to_glib(),
//             );
//         }
//     }

//     fn child_set_y_fill<P: IsA<clutter::Actor>>(&self, child: &P, fill: bool) {
//         unsafe {
//             ffi::mx_table_child_set_y_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 fill.to_glib(),
//             );
//         }
//     }

//     fn get_actor_at(&self, row: i32, column: i32) -> Option<clutter::Actor> {
//         unsafe {
//             from_glib_none(ffi::mx_table_get_actor_at(
//                 self.as_ref().to_glib_none().0,
//                 row,
//                 column,
//             ))
//         }
//     }

//     fn get_column_count(&self) -> i32 {
//         unsafe { ffi::mx_table_get_column_count(self.as_ref().to_glib_none().0) }
//     }

//     fn get_column_spacing(&self) -> i32 {
//         unsafe { ffi::mx_table_get_column_spacing(self.as_ref().to_glib_none().0) }
//     }

//     fn get_row_count(&self) -> i32 {
//         unsafe { ffi::mx_table_get_row_count(self.as_ref().to_glib_none().0) }
//     }

//     fn get_row_spacing(&self) -> i32 {
//         unsafe { ffi::mx_table_get_row_spacing(self.as_ref().to_glib_none().0) }
//     }

//     fn insert_actor<P: IsA<clutter::Actor>>(&self, actor: &P, row: i32, column: i32) {
//         unsafe {
//             ffi::mx_table_insert_actor(
//                 self.as_ref().to_glib_none().0,
//                 actor.as_ref().to_glib_none().0,
//                 row,
//                 column,
//             );
//         }
//     }

//     //fn insert_actor_with_properties<P: IsA<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//     //    unsafe { TODO: call ffi:mx_table_insert_actor_with_properties() }
//     //}

//     fn set_column_spacing(&self, spacing: i32) {
//         unsafe {
//             ffi::mx_table_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
//         }
//     }

//     fn set_row_spacing(&self, spacing: i32) {
//         unsafe {
//             ffi::mx_table_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
//         }
//     }

//     fn connect_property_column_count_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_column_count_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTable,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Table>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Table::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::column-count\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_column_count_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_column_spacing_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTable,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Table>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Table::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::column-spacing\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_column_spacing_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_row_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_row_count_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTable,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Table>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Table::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::row-count\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_row_count_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTable,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Table>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Table::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::row-spacing\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_row_spacing_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table")
    }
}
