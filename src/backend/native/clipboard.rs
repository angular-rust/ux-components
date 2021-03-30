#![allow(unused_variables)]

// use std::boxed::Box as Box_;

use crate::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Clipboard {}

impl Clipboard {
    pub fn get_default() -> Option<Clipboard> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::clipboard_get_default()) }
        unimplemented!()
    }
}

impl UxObject for Clipboard {}
impl Is<Clipboard> for Clipboard {}

impl AsRef<Clipboard> for Clipboard {
    fn as_ref(&self) -> &Clipboard {
        unimplemented!()
    }
}

pub const NONE_CLIPBOARD: Option<&Clipboard> = None;

pub trait ClipboardExt: 'static {
    fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P);

    fn set_text(&self, text: &str);
}

impl<O: Is<Clipboard>> ClipboardExt for O {
    fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P) {
        // let callback_data: Box_<P> = Box_::new(callback);
        // unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &str) + 'static>(
        //     clipboard: *mut ffi::Clipboard,
        //     text: *const libc::c_char,
        //     user_data: glib_sys::gpointer,
        // ) {
        //     let clipboard = from_glib_borrow(clipboard);
        //     let text: Borrowed<String> = from_glib_borrow(text);
        //     let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
        //     (*callback)(&clipboard, text.as_str());
        // }
        // let callback = Some(callback_func::<P> as _);
        // let super_callback0: Box_<P> = callback_data;
        // unsafe {
        //     ffi::clipboard_get_text(
        //         self.as_ref().to_glib_none().0,
        //         callback,
        //         Box_::into_raw(super_callback0) as *mut _,
        //     );
        // }
        unimplemented!()
    }

    fn set_text(&self, text: &str) {
        // unsafe {
        //     ffi::clipboard_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        // }
        unimplemented!()
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clipboard")
    }
}
