#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ActorManager {}

impl ActorManager {
    //pub fn new(stage: /*Ignored*/&clutter::Stage) -> ActorManager {
    //    unsafe { TODO: call ffi:actor_manager_new() }
    //}

    //pub fn get_for_stage(stage: /*Ignored*/&clutter::Stage) -> Option<ActorManager> {
    //    unsafe { TODO: call ffi:actor_manager_get_for_stage() }
    //}
}

impl Object for ActorManager {}
impl Is<ActorManager> for ActorManager {}

impl AsRef<ActorManager> for ActorManager {
    fn as_ref(&self) -> &ActorManager {
        self
    }
}

pub const NONE_ACTOR_MANAGER: Option<&ActorManager> = None;

pub trait ActorManagerExt: 'static {
    // fn add_actor<P: Is<clutter::Actor>, Q: Is<clutter::Actor>>(
    //     &self,
    //     container: &P,
    //     actor: &Q,
    // ) -> libc::c_ulong;

    // fn cancel_operation(&self, id: libc::c_ulong);

    fn cancel_operations<P: Is<clutter::Actor>>(&self, actor: &P);

    //fn create_actor(&self, create_func: /*Unimplemented*/Fn(&ActorManager, /*Unimplemented*/Option<Fundamental: Pointer>) -> clutter::Actor, userdata: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong;

    fn get_n_operations(&self) -> u32;

    fn get_stage(&self) -> Option<clutter::Stage>;

    fn get_time_slice(&self) -> u32;

    // fn remove_actor<P: Is<clutter::Actor>, Q: Is<clutter::Actor>>(
    //     &self,
    //     container: &P,
    //     actor: &Q,
    // ) -> libc::c_ulong;

    fn remove_container<P: Is<clutter::Actor>>(&self, container: &P);

    fn set_time_slice(&self, msecs: u32);

    // fn connect_actor_added<
    //     F: Fn(&Self, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    // fn connect_actor_created<F: Fn(&Self, libc::c_ulong, &clutter::Actor) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    fn connect_actor_finished<F: Fn(&Self, &clutter::Actor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    // fn connect_actor_removed<
    //     F: Fn(&Self, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    // fn connect_operation_cancelled<F: Fn(&Self, libc::c_ulong) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    // fn connect_operation_completed<F: Fn(&Self, libc::c_ulong) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    // fn connect_operation_failed<F: Fn(&Self, libc::c_ulong, &glib::Error) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId;

    fn connect_property_n_operations_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_time_slice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ActorManager>> ActorManagerExt for O {
    // fn add_actor<P: Is<clutter::Actor>, Q: Is<clutter::Actor>>(
    //     &self,
    //     container: &P,
    //     actor: &Q,
    // ) -> libc::c_ulong {
    //     // unsafe {
    //     //     ffi::actor_manager_add_actor(
    //     //         self.as_ref().to_glib_none().0,
    //     //         container.as_ref().to_glib_none().0,
    //     //         actor.as_ref().to_glib_none().0,
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn cancel_operation(&self, id: libc::c_ulong) {
    //     // unsafe {
    //     //     ffi::actor_manager_cancel_operation(self.as_ref().to_glib_none().0, id);
    //     // }
    //     unimplemented!()
    // }

    fn cancel_operations<P: Is<clutter::Actor>>(&self, actor: &P) {
        // unsafe {
        //     ffi::actor_manager_cancel_operations(
        //         self.as_ref().to_glib_none().0,
        //         actor.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    //fn create_actor(&self, create_func: /*Unimplemented*/Fn(&ActorManager, /*Unimplemented*/Option<Fundamental: Pointer>) -> clutter::Actor, userdata: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong {
    //    unsafe { TODO: call ffi:actor_manager_create_actor() }
    //}

    fn get_n_operations(&self) -> u32 {
        // unsafe { ffi::actor_manager_get_n_operations(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_stage(&self) -> Option<clutter::Stage> {
        //    unsafe { TODO: call ffi:actor_manager_get_stage() }
        unimplemented!()
    }

    fn get_time_slice(&self) -> u32 {
        // unsafe { ffi::actor_manager_get_time_slice(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    // fn remove_actor<P: Is<clutter::Actor>, Q: Is<clutter::Actor>>(
    //     &self,
    //     container: &P,
    //     actor: &Q,
    // ) -> libc::c_ulong {
    //     // unsafe {
    //     //     ffi::actor_manager_remove_actor(
    //     //         self.as_ref().to_glib_none().0,
    //     //         container.as_ref().to_glib_none().0,
    //     //         actor.as_ref().to_glib_none().0,
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn remove_container<P: Is<clutter::Actor>>(&self, container: &P) {
        // unsafe {
        //     ffi::actor_manager_remove_container(
        //         self.as_ref().to_glib_none().0,
        //         container.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_time_slice(&self, msecs: u32) {
        // unsafe {
        //     ffi::actor_manager_set_time_slice(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    // fn connect_actor_added<
    //     F: Fn(&Self, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn actor_added_trampoline<
    //     //     P,
    //     //     F: Fn(&P, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     container: *mut clutter_sys::ClutterActor,
    //     //     actor: *mut clutter_sys::ClutterActor,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &ActorManager::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         id,
    //     //         &from_glib_borrow(container),
    //     //         &from_glib_borrow(actor),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-added\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_added_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_actor_created<F: Fn(&Self, libc::c_ulong, &clutter::Actor) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn actor_created_trampoline<
    //     //     P,
    //     //     F: Fn(&P, libc::c_ulong, &clutter::Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     actor: *mut clutter_sys::ClutterActor,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &ActorManager::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         id,
    //     //         &from_glib_borrow(actor),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-created\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_created_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_actor_finished<F: Fn(&Self, &clutter::Actor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn actor_finished_trampoline<P, F: Fn(&P, &clutter::Actor) + 'static>(
        //     this: *mut ffi::ActorManager,
        //     actor: *mut clutter_sys::ClutterActor,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ActorManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(
        //         &ActorManager::from_glib_borrow(this).unsafe_cast_ref(),
        //         &from_glib_borrow(actor),
        //     )
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"actor-finished\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             actor_finished_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    // fn connect_actor_removed<
    //     F: Fn(&Self, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn actor_removed_trampoline<
    //     //     P,
    //     //     F: Fn(&P, libc::c_ulong, &clutter::Actor, &clutter::Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     container: *mut clutter_sys::ClutterActor,
    //     //     actor: *mut clutter_sys::ClutterActor,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &ActorManager::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         id,
    //     //         &from_glib_borrow(container),
    //     //         &from_glib_borrow(actor),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-removed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_removed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_cancelled<F: Fn(&Self, libc::c_ulong) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn operation_cancelled_trampoline<P, F: Fn(&P, libc::c_ulong) + 'static>(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref(), id)
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-cancelled\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_cancelled_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_completed<F: Fn(&Self, libc::c_ulong) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn operation_completed_trampoline<P, F: Fn(&P, libc::c_ulong) + 'static>(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref(), id)
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-completed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_completed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_failed<F: Fn(&Self, libc::c_ulong, &glib::Error) + 'static>(
    //     &self,
    //     f: F,
    // ) -> SignalHandlerId {
    //     // unsafe extern "C" fn operation_failed_trampoline<
    //     //     P,
    //     //     F: Fn(&P, libc::c_ulong, &glib::Error) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: libc::c_ulong,
    //     //     error: *mut glib_sys::GError,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &ActorManager::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         id,
    //     //         &from_glib_borrow(error),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-failed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_failed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_property_n_operations_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_n_operations_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ActorManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ActorManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::n-operations\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_n_operations_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_time_slice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_time_slice_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ActorManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ActorManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::time-slice\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_time_slice_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ActorManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorManager")
    }
}
