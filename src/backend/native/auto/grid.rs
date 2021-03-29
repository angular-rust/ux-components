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
//     pub struct Grid(Object<ffi::MxGrid, ffi::MxGridClass, GridClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_grid_get_type(),
//     }
// }

pub struct Grid {

}

impl Grid {
    pub fn new() -> Grid {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_grid_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GRID: Option<&Grid> = None;

// pub trait GridExt: 'static {
//     //fn get_child_x_align(&self) -> /*Ignored*/Align;

//     //fn get_child_y_align(&self) -> /*Ignored*/Align;

//     fn get_column_spacing(&self) -> f32;

//     fn get_homogenous_columns(&self) -> bool;

//     fn get_homogenous_rows(&self) -> bool;

//     //fn get_line_alignment(&self) -> /*Ignored*/Align;

//     fn get_max_stride(&self) -> i32;

//     //fn get_orientation(&self) -> /*Ignored*/Orientation;

//     fn get_row_spacing(&self) -> f32;

//     //fn set_child_x_align(&self, value: /*Ignored*/Align);

//     //fn set_child_y_align(&self, value: /*Ignored*/Align);

//     fn set_column_spacing(&self, value: f32);

//     fn set_homogenous_columns(&self, value: bool);

//     fn set_homogenous_rows(&self, value: bool);

//     //fn set_line_alignment(&self, value: /*Ignored*/Align);

//     fn set_max_stride(&self, value: i32);

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation);

//     fn set_row_spacing(&self, value: f32);

//     fn connect_property_child_x_align_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_child_y_align_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_homogenous_columns_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_homogenous_rows_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_max_stride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Grid>> GridExt for O {
//     //fn get_child_x_align(&self) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_grid_get_child_x_align() }
//     //}

//     //fn get_child_y_align(&self) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_grid_get_child_y_align() }
//     //}

//     fn get_column_spacing(&self) -> f32 {
//         unsafe { ffi::mx_grid_get_column_spacing(self.as_ref().to_glib_none().0) }
//     }

//     fn get_homogenous_columns(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_grid_get_homogenous_columns(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_homogenous_rows(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_grid_get_homogenous_rows(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn get_line_alignment(&self) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:mx_grid_get_line_alignment() }
//     //}

//     fn get_max_stride(&self) -> i32 {
//         unsafe { ffi::mx_grid_get_max_stride(self.as_ref().to_glib_none().0) }
//     }

//     //fn get_orientation(&self) -> /*Ignored*/Orientation {
//     //    unsafe { TODO: call ffi:mx_grid_get_orientation() }
//     //}

//     fn get_row_spacing(&self) -> f32 {
//         unsafe { ffi::mx_grid_get_row_spacing(self.as_ref().to_glib_none().0) }
//     }

//     //fn set_child_x_align(&self, value: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_grid_set_child_x_align() }
//     //}

//     //fn set_child_y_align(&self, value: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_grid_set_child_y_align() }
//     //}

//     fn set_column_spacing(&self, value: f32) {
//         unsafe {
//             ffi::mx_grid_set_column_spacing(self.as_ref().to_glib_none().0, value);
//         }
//     }

//     fn set_homogenous_columns(&self, value: bool) {
//         unsafe {
//             ffi::mx_grid_set_homogenous_columns(self.as_ref().to_glib_none().0, value.to_glib());
//         }
//     }

//     fn set_homogenous_rows(&self, value: bool) {
//         unsafe {
//             ffi::mx_grid_set_homogenous_rows(self.as_ref().to_glib_none().0, value.to_glib());
//         }
//     }

//     //fn set_line_alignment(&self, value: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:mx_grid_set_line_alignment() }
//     //}

//     fn set_max_stride(&self, value: i32) {
//         unsafe {
//             ffi::mx_grid_set_max_stride(self.as_ref().to_glib_none().0, value);
//         }
//     }

//     //fn set_orientation(&self, orientation: /*Ignored*/Orientation) {
//     //    unsafe { TODO: call ffi:mx_grid_set_orientation() }
//     //}

//     fn set_row_spacing(&self, value: f32) {
//         unsafe {
//             ffi::mx_grid_set_row_spacing(self.as_ref().to_glib_none().0, value);
//         }
//     }

//     fn connect_property_child_x_align_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_child_x_align_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::child-x-align\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_child_x_align_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_child_y_align_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_child_y_align_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::child-y-align\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_child_y_align_trampoline::<Self, F> as *const (),
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
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
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

//     fn connect_property_homogenous_columns_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_homogenous_columns_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::homogenous-columns\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_homogenous_columns_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_homogenous_rows_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_homogenous_rows_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::homogenous-rows\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_homogenous_rows_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_line_alignment_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::line-alignment\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_line_alignment_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_max_stride_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_max_stride_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::max-stride\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_max_stride_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::orientation\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_orientation_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_row_spacing_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxGrid,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Grid>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Grid::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid")
    }
}
