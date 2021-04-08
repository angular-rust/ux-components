#![allow(unused_variables)]

// use std::mem::transmute;
use super::{Align, Focusable, Orientation, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct DimensionData {
    pub expand: bool,
    pub is_visible: bool,
    pub min_size: f32,
    pub pref_size: f32,
    pub final_size: f32,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub ignore_css_col_spacing: bool,
    pub ignore_css_row_spacing: bool,
    pub col_spacing: u32,
    pub row_spacing: u32,

    pub visible_rows: u32,
    pub visible_cols: u32,

    pub n_rows: u32,
    pub n_cols: u32,

    pub active_row: i32,
    pub active_col: i32,

    pub columns: Vec<String>,
    pub rows: Vec<String>,

    pub last_focus: Focusable,
    widget: Widget,
}

impl Table {
    pub fn new() -> Table {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::table_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Table {}
impl Is<Table> for Table {}

impl AsRef<Table> for Table {
    fn as_ref(&self) -> &Table {
        self
    }
}

impl Is<Widget> for Table {}

impl AsRef<Widget> for Table {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Table {}

impl AsRef<clutter::Actor> for Table {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_TABLE: Option<&Table> = None;

pub trait TableExt: 'static {
    fn child_get_column<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_column_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_row<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_row_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32;

    fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> Align;

    fn child_get_x_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> Align;

    fn child_get_y_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_set_column<P: Is<clutter::Actor>>(&self, child: &P, col: i32);

    fn child_set_column_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32);

    fn child_set_row<P: Is<clutter::Actor>>(&self, child: &P, row: i32);

    fn child_set_row_span<P: Is<clutter::Actor>>(&self, child: &P, span: i32);

    fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, align: Align);

    fn child_set_x_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool);

    fn child_set_x_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool);

    fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, align: Align);

    fn child_set_y_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool);

    fn child_set_y_fill<P: Is<clutter::Actor>>(&self, child: &P, fill: bool);

    /// get_actor_at:
    /// @table: a #Table
    /// @row: the row to look into
    /// @column: the column to look into
    ///
    /// Get an actor at a given position in @table.
    ///
    /// Return value: (transfer none): the #ClutterActor a the given position, or None.
    ///
    fn get_actor_at(&self, row: u32, column: u32) -> Option<clutter::Actor>;

    /// get_column_count:
    /// @table: A #Table
    ///
    /// Retrieve the current number of columns in @table
    ///
    /// Returns: the number of columns
    ///
    fn get_column_count(&self) -> u32;

    /// get_column_spacing:
    /// @table: a #Table
    ///
    /// Gets the amount of spacing between columns.
    ///
    /// Returns: the spacing between columns in device units
    ///
    fn get_column_spacing(&self) -> u32;

    /// get_row_count:
    /// @table: A #Table
    ///
    /// Retrieve the current number rows in the @table
    ///
    /// Returns: the number of rows
    ///
    fn get_row_count(&self) -> u32;

    /// get_row_spacing:
    /// @table: a #Table
    ///
    /// Gets the amount of spacing between rows.
    ///
    /// Returns: the spacing between rows in device units
    ///
    fn get_row_spacing(&self) -> u32;

    /// insert_actor:
    /// @table: a #Table
    /// @actor: the child to insert
    /// @row: the row to place the child into
    /// @column: the column to place the child into
    ///
    /// Insert an actor at the specified row and column
    ///
    /// Note, column and rows numbers start from zero
    ///
    fn insert_actor<P: Is<clutter::Actor>>(&self, actor: &P, row: u32, column: u32);

    /// insert_actor_with_properties:
    /// @table: a #Table
    /// @actor: the child #ClutterActor
    /// @row: the row to place the child into
    /// @column: the column to place the child into
    /// @first_property_name: name of the first property to set
    /// @...: value for the first property, followed optionally by more name/value pairs terminated with None.
    ///
    /// Add an actor into at the specified row and column, with additional child
    /// properties to set.
    ///
    //fn insert_actor_with_properties<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    /// set_column_spacing:
    /// @table: a #Table
    /// @spacing: spacing in pixels
    ///
    /// Sets the amount of spacing between columns.
    ///
    fn set_column_spacing(&self, spacing: u32);

    /// set_row_spacing:
    /// @table: a #Table
    /// @spacing: spacing in pixels
    ///
    /// Sets the amount of spacing between rows.
    ///
    fn set_row_spacing(&self, spacing: u32);

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
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_get_column(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_column_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_get_column_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_row<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_get_row(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_row_span<P: Is<clutter::Actor>>(&self, child: &P) -> i32 {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_get_row_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     )
        // }
        unimplemented!()
    }

    fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> Align {
        let table = self.as_ref();
        let child = child.as_ref();
        //    unsafe { TODO: call ffi:table_child_get_x_align() }
        unimplemented!()
    }

    fn child_get_x_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     from_glib(ffi::table_child_get_x_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     from_glib(ffi::table_child_get_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> Align {
        let table = self.as_ref();
        let child = child.as_ref();
        //    unsafe { TODO: call ffi:table_child_get_y_align() }
        unimplemented!()
    }

    fn child_get_y_expand<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     from_glib(ffi::table_child_get_y_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     from_glib(ffi::table_child_get_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_set_column<P: Is<clutter::Actor>>(&self, child: &P, col: i32) {
        let table = self.as_ref();
        let child = child.as_ref();
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
        let table = self.as_ref();
        let child = child.as_ref();
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
        let table = self.as_ref();
        let child = child.as_ref();
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
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_set_row_span(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         span,
        //     );
        // }
        unimplemented!()
    }

    fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, align: Align) {
        let table = self.as_ref();
        let child = child.as_ref();
        //    unsafe { TODO: call ffi:table_child_set_x_align() }
        unimplemented!()
    }

    fn child_set_x_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool) {
        let table = self.as_ref();
        let child = child.as_ref();
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
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_set_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, align: Align) {
        let table = self.as_ref();
        let child = child.as_ref();
        //    unsafe { TODO: call ffi:table_child_set_y_align() }
        unimplemented!()
    }

    fn child_set_y_expand<P: Is<clutter::Actor>>(&self, child: &P, expand: bool) {
        let table = self.as_ref();
        let child = child.as_ref();
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
        let table = self.as_ref();
        let child = child.as_ref();
        // unsafe {
        //     ffi::table_child_set_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// get_actor_at:
    /// @table: a #Table
    /// @row: the row to look into
    /// @column: the column to look into
    ///
    /// Get an actor at a given position in @table.
    ///
    /// Return value: (transfer none): the #ClutterActor a the given position, or None.
    ///
    fn get_actor_at(&self, row: u32, column: u32) -> Option<clutter::Actor> {
        let table = self.as_ref();
        // table.find_actor_at(row, column);
        unimplemented!()
    }

    /// get_column_count:
    /// @table: A #Table
    ///
    /// Retrieve the current number of columns in @table
    ///
    /// Returns: the number of columns
    ///
    fn get_column_count(&self) -> u32 {
        let table = self.as_ref();
        table.n_cols
    }

    /// get_column_spacing:
    /// @table: a #Table
    ///
    /// Gets the amount of spacing between columns.
    ///
    /// Returns: the spacing between columns in device units
    ///
    fn get_column_spacing(&self) -> u32 {
        let table = self.as_ref();
        table.col_spacing
    }

    /// get_row_count:
    /// @table: A #Table
    ///
    /// Retrieve the current number rows in the @table
    ///
    /// Returns: the number of rows
    ///
    fn get_row_count(&self) -> u32 {
        let table = self.as_ref();
        table.n_rows
    }

    /// get_row_spacing:
    /// @table: a #Table
    ///
    /// Gets the amount of spacing between rows.
    ///
    /// Returns: the spacing between rows in device units
    ///
    fn get_row_spacing(&self) -> u32 {
        let table = self.as_ref();
        table.row_spacing
    }

    /// insert_actor:
    /// @table: a #Table
    /// @actor: the child to insert
    /// @row: the row to place the child into
    /// @column: the column to place the child into
    ///
    /// Insert an actor at the specified row and column
    ///
    /// Note, column and rows numbers start from zero
    ///
    fn insert_actor<P: Is<clutter::Actor>>(&self, actor: &P, row: u32, column: u32) {
        let table = self.as_ref();

        // if row < 0 {
        //     row = table->priv->n_rows + 1;
        // }

        // if column < 0 {
        //     column = table->priv->n_cols + 1;
        // }

        // clutter_actor_add_child(CLUTTER_ACTOR(table), actor);

        // let meta = (TableChild *)clutter_container_get_child_meta(CLUTTER_CONTAINER(table), actor);
        // meta.row = row;
        // meta.col = column;
        // _table_update_row_col(table, meta);

        // clutter_actor_queue_relayout(CLUTTER_ACTOR(table));
        unimplemented!()
    }

    /// insert_actor_with_properties:
    /// @table: a #Table
    /// @actor: the child #ClutterActor
    /// @row: the row to place the child into
    /// @column: the column to place the child into
    /// @first_property_name: name of the first property to set
    /// @...: value for the first property, followed optionally by more name/value pairs terminated with None.
    ///
    /// Add an actor into at the specified row and column, with additional child
    /// properties to set.
    ///
    //fn insert_actor_with_properties<P: Is<clutter::Actor>>(&self, actor: &P, row: i32, column: i32, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:table_insert_actor_with_properties() }
    //}

    /// set_column_spacing:
    /// @table: a #Table
    /// @spacing: spacing in pixels
    ///
    /// Sets the amount of spacing between columns.
    ///
    fn set_column_spacing(&self, spacing: u32) {
        let table = self.as_ref();
        if table.col_spacing != spacing {
            // table.col_spacing = spacing;
            // table.ignore_css_col_spacing = true;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(table));
            // g_object_notify(G_OBJECT(table), "column-spacing");
        }
    }

    /// set_row_spacing:
    /// @table: a #Table
    /// @spacing: spacing in pixels
    ///
    /// Sets the amount of spacing between rows.
    ///
    fn set_row_spacing(&self, spacing: u32) {
        let table = self.as_ref();

        if table.row_spacing != spacing {
            // table.row_spacing = spacing;
            // table.ignore_css_row_spacing = true;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR (table));
            // g_object_notify(G_OBJECT(table), "row-spacing");
        }
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
