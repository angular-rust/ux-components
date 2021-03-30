#![allow(unused_variables)]
#![allow(dead_code)]

use super::Window;
use crate::prelude::*;
use std::fmt;

struct ApplicationProps {
    name: String,
    windows: Vec<Window>,
    first_window: bool,
}

// @extends gio::Application, @implements gio::ActionGroup, gio::ActionMap;
pub struct Application {
    parent: gio::Application,
    props: ApplicationProps,
}

impl Application {
    pub fn new(application_id: &str, flags: gio::ApplicationFlags) -> Application {
        // application_new

        // assert_initialized_main_thread!();
        // unsafe {
        //     from_glib_full(ffi::application_new(
        //         application_id.to_glib_none().0,
        //         flags.to_glib(),
        //     ))
        // }
        unimplemented!()
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

pub trait ApplicationExt: 'static {
    // fn add_window<P: Is<Window>>(&self, window: &P);

    fn create_window(&self, window_title: &str) -> Option<Window>;

    fn get_windows(&self) -> Vec<Window>;

    // fn remove_window<P: Is<Window>>(&self, window: &P);
}

impl Application {
    fn add_window(&self, window: &Window) {
        // self.props.windows.push(window);
    }

    fn create_window(&self, window_title: &str) -> Option<Window> {
        let window = Window::new();
        // window.set_title(window_title);
        self.add_window(&window);
        Some(window)
    }

    fn get_windows(&self) -> &Vec<Window> {
        &self.props.windows
    }

    fn remove_window(&self, window: &Window) {
        unimplemented!()
    }
}

// impl<O: Is<Application>> ApplicationExt for O {
//     fn add_window<P: Is<Window>>(&self, window: &P) {
//         // unsafe {
//         //     ffi::application_add_window(
//         //         self.as_ref().to_glib_none().0,
//         //         window.as_ref().to_glib_full(),
//         //     );
//         // }
//     }

//     fn create_window(&self, window_title: &str) -> Option<Window> {
//         // unsafe {
//         //     from_glib_none(ffi::application_create_window(
//         //         self.as_ref().to_glib_none().0,
//         //         window_title.to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_windows(&self) -> Vec<Window> {
//         // unsafe {
//         //     FromGlibPtrContainer::from_glib_none(ffi::application_get_windows(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn remove_window<P: Is<Window>>(&self, window: &P) {
//         // unsafe {
//         //     ffi::application_remove_window(
//         //         self.as_ref().to_glib_none().0,
//         //         window.as_ref().to_glib_none().0,
//         //     );
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application")
    }
}
