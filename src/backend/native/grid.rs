#![allow(unused_variables)]

// use std::mem::transmute;
use super::{Adjustment, Align, Focusable, Orientation, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Grid {
    // pub hash_table: GHashTable,
    pub homogenous_rows: bool,
    pub homogenous_columns: bool,
    pub line_alignment: Align,
    pub col_spacing: f32,
    pub row_spacing: f32,
    pub child_x_align: Align,
    pub child_y_align: Align,
    pub orientation: Orientation,
    pub first_of_batch: bool,
    pub a_current_sum: f32,
    pub a_wrap: f32,
    pub max_extent_a: f32,
    pub max_extent_b: f32,
    pub max_stride: i32,
    pub hadjustment: Adjustment,
    pub vadjustment: Adjustment,
    pub last_focus: Focusable,
    pub ignore_css_col_spacing: bool,
    pub ignore_css_row_spacing: bool,
    widget: Widget,
}

impl Grid {
    pub fn new() -> Grid {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::grid_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Grid {}
impl Is<Grid> for Grid {}

impl AsRef<Grid> for Grid {
    fn as_ref(&self) -> &Grid {
        self
    }
}

impl Is<Widget> for Grid {}

impl AsRef<Widget> for Grid {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Grid {}

impl AsRef<clutter::Actor> for Grid {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_GRID: Option<&Grid> = None;

pub trait GridExt: 'static {
    fn get_child_x_align(&self) -> Align;

    fn get_child_y_align(&self) -> Align;

    fn get_column_spacing(&self) -> f32;

    fn get_homogenous_columns(&self) -> bool;

    fn get_homogenous_rows(&self) -> bool;

    fn get_line_alignment(&self) -> Align;

    fn get_max_stride(&self) -> i32;

    fn get_orientation(&self) -> Orientation;

    fn get_row_spacing(&self) -> f32;

    fn set_child_x_align(&self, value: Align);

    fn set_child_y_align(&self, value: Align);

    fn set_column_spacing(&self, value: f32);

    fn set_homogenous_columns(&self, value: bool);

    fn set_homogenous_rows(&self, value: bool);

    fn set_line_alignment(&self, value: Align);

    fn set_max_stride(&self, value: i32);

    fn set_orientation(&self, orientation: Orientation);

    fn set_row_spacing(&self, value: f32);

    fn connect_property_child_x_align_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_child_y_align_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_homogenous_columns_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_homogenous_rows_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_max_stride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Grid>> GridExt for O {
    fn get_child_x_align(&self) -> Align {
        let grid = self.as_ref();
        grid.child_x_align
    }

    fn get_child_y_align(&self) -> Align {
        let grid = self.as_ref();
        grid.child_y_align
    }

    fn get_column_spacing(&self) -> f32 {
        let grid = self.as_ref();
        grid.col_spacing
    }

    fn get_homogenous_columns(&self) -> bool {
        let grid = self.as_ref();
        grid.homogenous_columns
    }

    fn get_homogenous_rows(&self) -> bool {
        let grid = self.as_ref();
        grid.homogenous_rows
    }

    fn get_line_alignment(&self) -> Align {
        let grid = self.as_ref();
        grid.line_alignment
    }

    fn get_max_stride(&self) -> i32 {
        let grid = self.as_ref();
        grid.max_stride
    }

    fn get_orientation(&self) -> Orientation {
        let grid = self.as_ref();
        grid.orientation
    }

    fn get_row_spacing(&self) -> f32 {
        let grid = self.as_ref();
        grid.row_spacing
    }

    fn set_child_x_align(&self, value: Align) {
        let grid = self.as_ref();

        if value != grid.child_x_align {
            // grid.child_x_align = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_child_y_align(&self, value: Align) {
        let grid = self.as_ref();

        if value != grid.child_y_align {
            // grid.child_y_align = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_column_spacing(&self, value: f32) {
        let grid = self.as_ref();

        if grid.col_spacing != value {
            // grid.ignore_css_col_spacing = true;
            // grid.col_spacing = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_homogenous_columns(&self, value: bool) {
        let grid = self.as_ref();

        if value != grid.homogenous_columns {
            // grid.homogenous_columns = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_homogenous_rows(&self, value: bool) {
        let grid = self.as_ref();

        if value != grid.homogenous_rows {
            // grid.homogenous_rows = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_line_alignment(&self, value: Align) {
        let grid = self.as_ref();

        if value != grid.line_alignment {
            // grid.line_alignment = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_max_stride(&self, value: i32) {
        let grid = self.as_ref();

        if value != grid.max_stride {
            // grid.max_stride = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        let grid = self.as_ref();

        if grid.orientation != orientation {
            // grid.orientation = orientation;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
            // g_object_notify(G_OBJECT(self), "orientation");
        }
    }

    fn set_row_spacing(&self, value: f32) {
        let grid = self.as_ref();

        if value != grid.row_spacing {
            // grid.ignore_css_row_spacing = true;
            // grid.row_spacing = value;
            // clutter_actor_queue_relayout(CLUTTER_ACTOR(self));
        }
    }

    fn connect_property_child_x_align_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_child_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::child-x-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_child_x_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_child_y_align_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_child_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::child-y-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_child_y_align_trampoline::<Self, F> as *const (),
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
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_homogenous_columns_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_homogenous_columns_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::homogenous-columns\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_homogenous_columns_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_homogenous_rows_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_homogenous_rows_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::homogenous-rows\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_homogenous_rows_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_line_alignment_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::line-alignment\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_line_alignment_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_max_stride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_max_stride_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::max-stride\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_max_stride_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::orientation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_orientation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Grid,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Grid>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid")
    }
}
