#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Stack, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Stack, Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Pager {}

impl Pager {
    pub fn new() -> Pager {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::pager_new()) }
        unimplemented!()
    }
}

impl Default for Pager {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Pager {}
impl Is<Pager> for Pager {}

impl AsRef<Pager> for Pager {
    fn as_ref(&self) -> &Pager {
        self
    }
}

pub const NONE_PAGER: Option<&Pager> = None;

pub trait PagerExt: 'static {
    fn get_actor_for_page(&self, page: u32) -> Option<clutter::Actor>;

    fn get_current_page(&self) -> u32;

    fn get_current_page_actor(&self) -> Option<clutter::Actor>;

    fn get_edge_previews(&self) -> bool;

    fn get_n_pages(&self) -> u32;

    fn insert_page<P: Is<clutter::Actor>>(&self, child: &P, position: i32);

    fn next(&self);

    fn previous(&self);

    fn set_current_page(&self, page: u32, animate: bool);

    fn set_current_page_by_actor<P: Is<clutter::Actor>>(&self, actor: &P, animate: bool);

    fn set_edge_previews(&self, edge_previews: bool);

    fn get_property_page_actor(&self) -> Option<clutter::Actor>;

    // fn set_property_page_actor<P: Is<clutter::Actor> + SetValueOptional>(
    //     &self,
    //     page_actor: Option<&P>,
    // );

    fn get_property_page_num(&self) -> u32;

    fn set_property_page_num(&self, page_num: u32);

    fn connect_property_edge_previews_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_page_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_num_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Pager>> PagerExt for O {
    fn get_actor_for_page(&self, page: u32) -> Option<clutter::Actor> {
        // unsafe {
        //     from_glib_none(ffi::pager_get_actor_for_page(
        //         self.as_ref().to_glib_none().0,
        //         page,
        //     ))
        // }
        unimplemented!()
    }

    fn get_current_page(&self) -> u32 {
        // unsafe { ffi::pager_get_current_page(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_current_page_actor(&self) -> Option<clutter::Actor> {
        // unsafe {
        //     from_glib_none(ffi::pager_get_current_page_actor(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_edge_previews(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::pager_get_edge_previews(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_n_pages(&self) -> u32 {
        // unsafe { ffi::pager_get_n_pages(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn insert_page<P: Is<clutter::Actor>>(&self, child: &P, position: i32) {
        // unsafe {
        //     ffi::pager_insert_page(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         position,
        //     );
        // }
        unimplemented!()
    }

    fn next(&self) {
        // unsafe {
        //     ffi::pager_next(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn previous(&self) {
        // unsafe {
        //     ffi::pager_previous(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_current_page(&self, page: u32, animate: bool) {
        // unsafe {
        //     ffi::pager_set_current_page(
        //         self.as_ref().to_glib_none().0,
        //         page,
        //         animate.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_current_page_by_actor<P: Is<clutter::Actor>>(&self, actor: &P, animate: bool) {
        // unsafe {
        //     ffi::pager_set_current_page_by_actor(
        //         self.as_ref().to_glib_none().0,
        //         actor.as_ref().to_glib_none().0,
        //         animate.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_edge_previews(&self, edge_previews: bool) {
        // unsafe {
        //     ffi::pager_set_edge_previews(
        //         self.as_ref().to_glib_none().0,
        //         edge_previews.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn get_property_page_actor(&self) -> Option<clutter::Actor> {
        // unsafe {
        //     let mut value = Value::from_type(<clutter::Actor as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-actor\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `page-actor` getter")
        // }
        unimplemented!()
    }

    // fn set_property_page_actor<P: Is<clutter::Actor> + SetValueOptional>(
    //     &self,
    //     page_actor: Option<&P>,
    // ) {
    //     // unsafe {
    //     //     gobject_sys::g_object_set_property(
    //     //         self.to_glib_none().0 as *mut gobject_sys::GObject,
    //     //         b"page-actor\0".as_ptr() as *const _,
    //     //         Value::from(page_actor).to_glib_none().0,
    //     //     );
    //     // }
    //     unimplemented!()
    // }

    fn get_property_page_num(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-num\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `page-num` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_page_num(&self, page_num: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"page-num\0".as_ptr() as *const _,
        //         Value::from(&page_num).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_edge_previews_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_edge_previews_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::edge-previews\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_edge_previews_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_actor_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-actor\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_actor_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_num_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_num_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Pager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Pager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Pager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-num\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_num_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Pager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pager")
    }
}
