use ffi;

use glib::object::IsA;
use glib::translate::*;

use Adjustment;
use ScrollBar;

impl ScrollBar {
    pub fn new() -> ScrollBar {
        unimplemented!(); // TODO: complete it

        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::scroll_bar_new()) }
    }

    pub fn with_adjustment<P: IsA<Adjustment>>(adjustment: &P) -> ScrollBar {
        unimplemented!(); // TODO: complete it

        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::scroll_bar_new_with_adjustment()) }
    }
}
