use ffi;

use glib::translate::*;

use Entry;

impl Entry {
    pub fn new() -> Entry {
        unimplemented!(); // TODO: complete it

        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::mx_entry_new()) }
    }

    pub fn with_text(text: &str) -> Entry {
        unimplemented!(); // TODO: complete it

        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::mx_entry_new_with_text()) }
    }
}
