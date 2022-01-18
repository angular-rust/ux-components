#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, ActorBox, Align, Widget};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct StackProps {
    pub current_focus: Actor,
    pub allocation: ActorBox,
}

#[derive(Debug)]
pub struct Stack {
    props: RefCell<StackProps>,
    widget: Widget,
}

impl Stack {
    pub fn new() -> Stack {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::stack_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Stack {}
impl Is<Stack> for Stack {}

impl AsRef<Stack> for Stack {
    fn as_ref(&self) -> &Stack {
        self
    }
}

impl Is<Widget> for Stack {}

impl AsRef<Widget> for Stack {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Stack {}

impl AsRef<Actor> for Stack {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait StackExt: 'static {
    fn child_get_crop<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_get_fit<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_get_x_align<P: Is<Actor>>(&self, child: &P) -> Align;

    fn child_get_x_fill<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_get_y_align<P: Is<Actor>>(&self, child: &P) -> Align;

    fn child_get_y_fill<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_set_crop<P: Is<Actor>>(&self, child: &P, crop: bool);

    fn child_set_fit<P: Is<Actor>>(&self, child: &P, fit: bool);

    fn child_set_x_align<P: Is<Actor>>(&self, child: &P, x_align: Align);

    fn child_set_x_fill<P: Is<Actor>>(&self, child: &P, x_fill: bool);

    fn child_set_y_align<P: Is<Actor>>(&self, child: &P, y_align: Align);

    fn child_set_y_fill<P: Is<Actor>>(&self, child: &P, y_fill: bool);
}

impl<O: Is<Stack>> StackExt for O {
    fn child_get_crop<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::stack_child_get_crop(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_fit<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::stack_child_get_fit(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_x_align<P: Is<Actor>>(&self, child: &P) -> Align {
        //    unsafe { TODO: call ffi:stack_child_get_x_align() }
        unimplemented!()
    }

    fn child_get_x_fill<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::stack_child_get_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_y_align<P: Is<Actor>>(&self, child: &P) -> Align {
        //    unsafe { TODO: call ffi:stack_child_get_y_align() }
        unimplemented!()
    }

    fn child_get_y_fill<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::stack_child_get_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_set_crop<P: Is<Actor>>(&self, child: &P, crop: bool) {
        // unsafe {
        //     ffi::stack_child_set_crop(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         crop.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_fit<P: Is<Actor>>(&self, child: &P, fit: bool) {
        // unsafe {
        //     ffi::stack_child_set_fit(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         fit.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_x_align<P: Is<Actor>>(&self, child: &P, x_align: Align) {
        //    unsafe { TODO: call ffi:stack_child_set_x_align() }
        unimplemented!()
    }

    fn child_set_x_fill<P: Is<Actor>>(&self, child: &P, x_fill: bool) {
        // unsafe {
        //     ffi::stack_child_set_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         x_fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_y_align<P: Is<Actor>>(&self, child: &P, y_align: Align) {
        //    unsafe { TODO: call ffi:stack_child_set_y_align() }
        unimplemented!()
    }

    fn child_set_y_fill<P: Is<Actor>>(&self, child: &P, y_fill: bool) {
        // unsafe {
        //     ffi::stack_child_set_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         y_fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack")
    }
}
