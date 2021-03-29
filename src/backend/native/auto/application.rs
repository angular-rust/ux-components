// use glib::object::IsA;
// use glib::translate::*;
use std::fmt;
use super::Window;

// glib_wrapper! {
//     pub struct Application(Object<ffi::MxApplication, ffi::MxApplicationClass, ApplicationClass>) @extends gio::Application, @implements gio::ActionGroup, gio::ActionMap;

//     match fn {
//         get_type => || ffi::mx_application_get_type(),
//     }
// }

struct ApplicationProps {
    name: String,
    windows: Vec<Window>
}

pub struct Application {
    parent: gio::Application,
    props: ApplicationProps,
}

impl Application {
    pub fn new(application_id: &str, flags: gio::ApplicationFlags) -> Application {
        // mx_application_new
        unimplemented!()
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

// pub trait ApplicationExt: 'static {
//     fn add_window<P: IsA<Window>>(&self, window: &P);

//     fn create_window(&self, window_title: &str) -> Option<Window>;

//     fn get_windows(&self) -> Vec<Window>;

//     fn remove_window<P: IsA<Window>>(&self, window: &P);
// }

// impl<O: IsA<Application>> ApplicationExt for O {
//     fn add_window<P: IsA<Window>>(&self, window: &P) {
//         // unsafe {
//         //     ffi::mx_application_add_window(
//         //         self.as_ref().to_glib_none().0,
//         //         window.as_ref().to_glib_full(),
//         //     );
//         // }
//     }

//     fn create_window(&self, window_title: &str) -> Option<Window> {
//         // unsafe {
//         //     from_glib_none(ffi::mx_application_create_window(
//         //         self.as_ref().to_glib_none().0,
//         //         window_title.to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn get_windows(&self) -> Vec<Window> {
//         // unsafe {
//         //     FromGlibPtrContainer::from_glib_none(ffi::mx_application_get_windows(
//         //         self.as_ref().to_glib_none().0,
//         //     ))
//         // }
//         unimplemented!()
//     }

//     fn remove_window<P: IsA<Window>>(&self, window: &P) {
//         // unsafe {
//         //     ffi::mx_application_remove_window(
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
