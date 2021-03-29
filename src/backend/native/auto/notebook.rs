// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib_sys;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Widget;

// glib_wrapper! {
//     pub struct Notebook(Object<ffi::MxNotebook, ffi::MxNotebookClass, NotebookClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_notebook_get_type(),
//     }
// }

pub struct Notebook{

}

impl Notebook {
    pub fn new() -> Notebook {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_notebook_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_NOTEBOOK: Option<&Notebook> = None;

// pub trait NotebookExt: 'static {
//     fn get_current_page(&self) -> Option<clutter::Actor>;

//     fn next_page(&self);

//     fn previous_page(&self);

//     fn set_current_page<P: IsA<clutter::Actor>>(&self, page: &P);

//     fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;
// }

// impl<O: IsA<Notebook>> NotebookExt for O {
//     fn get_current_page(&self) -> Option<clutter::Actor> {
//         unsafe {
//             from_glib_none(ffi::mx_notebook_get_current_page(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn next_page(&self) {
//         unsafe {
//             ffi::mx_notebook_next_page(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn previous_page(&self) {
//         unsafe {
//             ffi::mx_notebook_previous_page(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn set_current_page<P: IsA<clutter::Actor>>(&self, page: &P) {
//         unsafe {
//             ffi::mx_notebook_set_current_page(
//                 self.as_ref().to_glib_none().0,
//                 page.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn connect_property_current_page_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_current_page_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxNotebook,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Notebook>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Notebook::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::current-page\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_current_page_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notebook")
    }
}
