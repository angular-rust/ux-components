use gio::ApplicationExt;
use gio::ApplicationFlags;
use glib;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectExt;
use ffi;
use rt;
use Application;

use std::cell::RefCell;
use std::rc::Rc;

impl Application {
    pub fn new(application_id: &str, flags: ApplicationFlags) -> Application {
        // assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }
}
