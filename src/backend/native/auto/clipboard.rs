// use glib::object::IsA;
// use glib::translate::*;
// use glib::GString;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;

// glib_wrapper! {
//     pub struct Clipboard(Object<ffi::MxClipboard, ffi::MxClipboardClass, ClipboardClass>);

//     match fn {
//         get_type => || ffi::mx_clipboard_get_type(),
//     }
// }

pub struct Clipboard {

}

impl Clipboard {
    pub fn get_default() -> Option<Clipboard> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::mx_clipboard_get_default()) }
        unimplemented!()
    }
}

pub const NONE_CLIPBOARD: Option<&Clipboard> = None;

pub trait ClipboardExt: 'static {
    fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P);

    fn set_text(&self, text: &str);
}

// impl<O: IsA<Clipboard>> ClipboardExt for O {
//     fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P) {
//         // let callback_data: Box_<P> = Box_::new(callback);
//         // unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &str) + 'static>(
//         //     clipboard: *mut ffi::MxClipboard,
//         //     text: *const libc::c_char,
//         //     user_data: glib_sys::gpointer,
//         // ) {
//         //     let clipboard = from_glib_borrow(clipboard);
//         //     let text: Borrowed<GString> = from_glib_borrow(text);
//         //     let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
//         //     (*callback)(&clipboard, text.as_str());
//         // }
//         // let callback = Some(callback_func::<P> as _);
//         // let super_callback0: Box_<P> = callback_data;
//         // unsafe {
//         //     ffi::mx_clipboard_get_text(
//         //         self.as_ref().to_glib_none().0,
//         //         callback,
//         //         Box_::into_raw(super_callback0) as *mut _,
//         //     );
//         // }
//         unimplemented!()
//     }

//     fn set_text(&self, text: &str) {
//         // unsafe {
//         //     ffi::mx_clipboard_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         // }
//         unimplemented!()
//     }
// }

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clipboard")
    }
}
