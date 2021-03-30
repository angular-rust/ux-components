use super::Widget;
use crate::prelude::*;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Stack {}

impl Stack {
    pub fn new() -> Stack {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::stack_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Stack {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::stack_new()) }
    // }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STACK: Option<&Stack> = None;

pub trait StackExt: 'static {
    fn child_get_crop<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_get_fit<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    //fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

    fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    //fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align;

    fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool;

    fn child_set_crop<P: Is<clutter::Actor>>(&self, child: &P, crop: bool);

    fn child_set_fit<P: Is<clutter::Actor>>(&self, child: &P, fit: bool);

    //fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, x_align: /*Ignored*/Align);

    fn child_set_x_fill<P: Is<clutter::Actor>>(&self, child: &P, x_fill: bool);

    //fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, y_align: /*Ignored*/Align);

    fn child_set_y_fill<P: Is<clutter::Actor>>(&self, child: &P, y_fill: bool);
}

// impl<O: Is<Stack>> StackExt for O {
//     fn child_get_crop<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::stack_child_get_crop(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn child_get_fit<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::stack_child_get_fit(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn child_get_x_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:stack_child_get_x_align() }
//     //}

//     fn child_get_x_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::stack_child_get_x_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     //fn child_get_y_align<P: Is<clutter::Actor>>(&self, child: &P) -> /*Ignored*/Align {
//     //    unsafe { TODO: call ffi:stack_child_get_y_align() }
//     //}

//     fn child_get_y_fill<P: Is<clutter::Actor>>(&self, child: &P) -> bool {
//         unsafe {
//             from_glib(ffi::stack_child_get_y_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn child_set_crop<P: Is<clutter::Actor>>(&self, child: &P, crop: bool) {
//         unsafe {
//             ffi::stack_child_set_crop(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 crop.to_glib(),
//             );
//         }
//     }

//     fn child_set_fit<P: Is<clutter::Actor>>(&self, child: &P, fit: bool) {
//         unsafe {
//             ffi::stack_child_set_fit(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 fit.to_glib(),
//             );
//         }
//     }

//     //fn child_set_x_align<P: Is<clutter::Actor>>(&self, child: &P, x_align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:stack_child_set_x_align() }
//     //}

//     fn child_set_x_fill<P: Is<clutter::Actor>>(&self, child: &P, x_fill: bool) {
//         unsafe {
//             ffi::stack_child_set_x_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 x_fill.to_glib(),
//             );
//         }
//     }

//     //fn child_set_y_align<P: Is<clutter::Actor>>(&self, child: &P, y_align: /*Ignored*/Align) {
//     //    unsafe { TODO: call ffi:stack_child_set_y_align() }
//     //}

//     fn child_set_y_fill<P: Is<clutter::Actor>>(&self, child: &P, y_fill: bool) {
//         unsafe {
//             ffi::stack_child_set_y_fill(
//                 self.as_ref().to_glib_none().0,
//                 child.as_ref().to_glib_none().0,
//                 y_fill.to_glib(),
//             );
//         }
//     }
// }

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack")
    }
}
