use crate::prelude::*;
// use glib::object::Cast;
// use glib::translate::*;

use std::fmt;
// use super::Action;
// use super::Widget;

// glib_wrapper! {
//     pub struct Dialog(Object<ffi::Dialog, ffi::DialogClass, DialogClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::dialog_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Dialog {

}

impl Dialog {
    pub fn new() -> Dialog {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::dialog_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DIALOG: Option<&Dialog> = None;

// pub trait DialogExt: 'static {
//     fn add_action<P: IsA<Action>>(&self, action: &P);

//     fn get_actions(&self) -> Vec<Action>;

//     fn remove_action<P: IsA<Action>>(&self, action: &P);

//     fn set_transient_parent<P: IsA<clutter::Actor>>(&self, actor: &P);
// }

// impl<O: IsA<Dialog>> DialogExt for O {
//     fn add_action<P: IsA<Action>>(&self, action: &P) {
//         // unsafe {
//         //     ffi::dialog_add_action(
//         //         self.as_ref().to_glib_none().0,
//         //         action.as_ref().to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn get_actions(&self) -> Vec<Action> {
//         // unsafe {
//         //     FromGlibPtrContainer::from_glib_container(ffi::dialog_get_actions(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn remove_action<P: IsA<Action>>(&self, action: &P) {
//         // unsafe {
//         //     ffi::dialog_remove_action(
//         //         self.as_ref().to_glib_none().0,
//         //         action.as_ref().to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_transient_parent<P: IsA<clutter::Actor>>(&self, actor: &P) {
//         // unsafe {
//         //     ffi::dialog_set_transient_parent(
//         //         self.as_ref().to_glib_none().0,
//         //         actor.as_ref().to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Dialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dialog")
    }
}
