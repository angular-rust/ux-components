use crate::prelude::*;
// use glib::object::Cast;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;


// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Action;
// use FloatingWidget;
// use Widget;

// glib_wrapper! {
//     pub struct Menu(Object<ffi::Menu, ffi::MenuClass, MenuClass>) @extends FloatingWidget, Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::menu_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Menu {

}

impl Menu {
    pub fn new() -> Menu {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::menu_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl UxComponent for Menu {}

pub const NONE_MENU: Option<&Menu> = None;

// pub trait MenuExt: 'static {
//     fn add_action<P: IsA<Action>>(&self, action: &P);

//     fn remove_action<P: IsA<Action>>(&self, action: &P);

//     fn remove_all(&self);

//     fn show_with_position(&self, x: f32, y: f32);

//     fn connect_action_activated<F: Fn(&Self, &Action) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Menu>> MenuExt for O {
//     fn add_action<P: IsA<Action>>(&self, action: &P) {
//         unsafe {
//             ffi::menu_add_action(
//                 self.as_ref().to_glib_none().0,
//                 action.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn remove_action<P: IsA<Action>>(&self, action: &P) {
//         unsafe {
//             ffi::menu_remove_action(
//                 self.as_ref().to_glib_none().0,
//                 action.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn remove_all(&self) {
//         unsafe {
//             ffi::menu_remove_all(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn show_with_position(&self, x: f32, y: f32) {
//         unsafe {
//             ffi::menu_show_with_position(self.as_ref().to_glib_none().0, x, y);
//         }
//     }

//     fn connect_action_activated<F: Fn(&Self, &Action) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn action_activated_trampoline<P, F: Fn(&P, &Action) + 'static>(
//             this: *mut ffi::Menu,
//             object: *mut ffi::Action,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Menu>,
//         {
//             let f: &F = &*(f as *const F);
//             f(
//                 &Menu::from_glib_borrow(this).unsafe_cast_ref(),
//                 &from_glib_borrow(object),
//             )
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"action-activated\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     action_activated_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
