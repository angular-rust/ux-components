#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Stage};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct ActorManagerProps {
    // pub ops: GQueue,
    // pub actor_op_links: GHashTable,
    pub source: u32,
    pub post_paint_handler: u64,
    // pub timer: GTimer,
    pub quark_set: bool,
    pub time_slice: u32,
    pub stage: Option<Stage>,
}

#[derive(Debug)]
pub struct ActorManager {
    props: RefCell<ActorManagerProps>,
}

impl ActorManager {
    //pub fn new(stage: &Stage) -> ActorManager {
    //    unsafe { TODO: call ffi:actor_manager_new() }
    //}

    //pub fn get_for_stage(stage: &Stage) -> Option<ActorManager> {
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

pub trait ActorManagerExt: 'static {
    /// add_actor:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    /// @actor: A #Actor
    ///
    /// Adds @actor to @container. The actor may not be parented immediately,
    /// or at all, if the operation is cancelled.
    ///
    /// On successful completion, the #ActorManager::actor_added signal will
    /// be fired.
    ///
    /// Returns: The ID for this operation.
    ///
    fn add_actor<P: Is<Actor>, Q: Is<Actor>>(&self, container: &P, actor: &Q) -> u64;

    /// cancel_operation:
    /// @manager: A #ActorManager
    /// @id: An operation ID
    ///
    /// Cancels the given operation, if it exists. The
    /// #ActorManager::operation_cancelled signal is fired whenever an operation
    /// is cancelled.
    ///
    fn cancel_operation(&self, id: u64);

    /// cancel_operations:
    /// @manager: A #ActorManager
    /// @actor: A #Actor
    ///
    /// Cancels all operations associated with the given actor.
    ///
    fn cancel_operations<P: Is<Actor>>(&self, actor: &P);

    /// create_actor:
    /// @manager: A #ActorManager
    /// @create_func: A #Actor creation function
    /// @userdata: data to be passed to the function, or %None
    /// @destroy_func: callback to invoke before the operation is removed
    ///
    /// Creates a #Actor. The actor may not be created immediately,
    /// or at all, if the operation is cancelled.
    ///
    /// On successful completion, the #ActorManager::actor_created signal will
    /// be fired.
    ///
    /// Returns: The ID for this operation.
    ///
    //fn create_actor(&self, create_func: Fn(&ActorManager, Option<Fundamental: Pointer>) -> Actor, userdata: Option<Fundamental: Pointer>) -> u64;

    /// get_n_operations:
    /// @manager: A #ActorManager
    ///
    /// Retrieves the amount of operations left in the queue.
    ///
    /// Returns: Number of operations left to perform
    ///
    fn get_n_operations(&self) -> u32;

    /// get_stage:
    /// @manager: A #ActorManager
    ///
    /// Gets the #Stage the actor manager is associated with.
    ///
    /// Returns: (transfer none): The #Stage the actor is associated with.
    ///
    fn get_stage(&self) -> Option<Stage>;

    /// get_time_slice:
    /// @manager: A #ActorManager
    ///
    /// Retrieves the current time slice being used for operations.
    ///
    /// Returns: The time-slice being used, in milliseconds
    ///
    fn get_time_slice(&self) -> u32;

    /// remove_actor:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    /// @actor: A #Actor
    ///
    /// Removes @actor from @container.
    ///
    /// On successful completion, the #ActorManager::actor_removed signal will
    /// be fired.
    ///
    /// <note><para>
    /// The actor may not be removed immediately, and thus you may want to set
    /// the actor's opacity to 0 before calling this function.
    /// </para></note>
    ///
    /// Returns: The ID for this operation.
    ///
    fn remove_actor<P: Is<Actor>, Q: Is<Actor>>(&self, container: &P, actor: &Q) -> u64;

    /// remove_container:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    ///
    /// Removes the container. This is a utility function that works by first
    /// removing all the children of the container, then the children itself. This
    /// effectively spreads the load of removing a large container. All prior
    /// operations associated with this container will be cancelled.
    ///
    /// <note><para>
    /// The container may not be removed immediately, and thus you may want to set
    /// the container's opacity to 0 before calling this function.
    /// </para></note>
    ///
    fn remove_container<P: Is<Actor>>(&self, container: &P);

    /// set_time_slice:
    /// @manager: A #ActorManager
    /// @msecs: A time, in milliseconds
    ///
    /// Sets the amount of time the actor manager will spend performing operations,
    /// before yielding to allow any necessary redrawing to occur.
    ///
    /// Lower times will lead to smoother performance, but will increase the amount
    /// of time it takes for operations to complete.
    ///
    fn set_time_slice(&self, msecs: u32);

    // fn connect_actor_added<
    //     F: Fn(&Self, u64, &Actor, &Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    // fn connect_actor_created<F: Fn(&Self, u64, &Actor) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    fn connect_actor_finished<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    // fn connect_actor_removed<
    //     F: Fn(&Self, u64, &Actor, &Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    // fn connect_operation_cancelled<F: Fn(&Self, u64) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    // fn connect_operation_completed<F: Fn(&Self, u64) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    // fn connect_operation_failed<F: Fn(&Self, u64, &glib::Error) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId;

    fn connect_property_n_operations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_time_slice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<ActorManager>> ActorManagerExt for O {
    /// add_actor:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    /// @actor: A #Actor
    ///
    /// Adds @actor to @container. The actor may not be parented immediately,
    /// or at all, if the operation is cancelled.
    ///
    /// On successful completion, the #ActorManager::actor_added signal will
    /// be fired.
    ///
    /// Returns: The ID for this operation.
    ///
    fn add_actor<P: Is<Actor>, Q: Is<Actor>>(&self, container: &P, actor: &Q) -> u64 {
        let manager = self.as_ref();
        let container = container.as_ref();
        let actor = actor.as_ref();

        // let op: ActorManagerOperation =
        //     manager.op_new(ACTOR_MANAGER_ADD, None, None, actor, container);
        // manager.ensure_processing(manager);

        // return op.id;
        unimplemented!()
    }

    /// cancel_operation:
    /// @manager: A #ActorManager
    /// @id: An operation ID
    ///
    /// Cancels the given operation, if it exists. The
    /// #ActorManager::operation_cancelled signal is fired whenever an operation
    /// is cancelled.
    ///
    fn cancel_operation(&self, id: u64) {
        let manager = self.as_ref();

        // GList *op_link;

        // op_link = g_queue_find_custom (manager.ops, &id, actor_manager_find_by_id);

        // if !op_link {
        //     g_warning (G_STRLOC ": Unknown operation (%lu)", id);
        //     return;
        // }

        // g_queue_unlink(manager.ops, op_link);

        // g_signal_emit(manager, signals[OP_CANCELLED], 0, id);

        // manager.op_free(op_link, false);
        // g_list_free(op_link);
    }

    /// cancel_operations:
    /// @manager: A #ActorManager
    /// @actor: A #Actor
    ///
    /// Cancels all operations associated with the given actor.
    ///
    fn cancel_operations<P: Is<Actor>>(&self, actor: &P) {
        let manager = self.as_ref();
        let actor = actor.as_ref();

        // GList *op_links;

        // op_links = g_hash_table_lookup (manager.actor_op_links, actor);
        // while op_links {
        //     GList *op_link = op_links->data;
        //     ActorManagerOperation *op = op_link.data;
        //     op_links = op_links.next;
        //     g_queue_unlink (manager.ops, op_link);
        //     g_signal_emit (manager, signals[OP_CANCELLED], 0, op.id);
        //     actor_manager_op_free (manager, op_link, false);
        //     g_list_free (op_link);
        // }
    }

    /// create_actor:
    /// @manager: A #ActorManager
    /// @create_func: A #Actor creation function
    /// @userdata: data to be passed to the function, or %None
    /// @destroy_func: callback to invoke before the operation is removed
    ///
    /// Creates a #Actor. The actor may not be created immediately,
    /// or at all, if the operation is cancelled.
    ///
    /// On successful completion, the #ActorManager::actor_created signal will
    /// be fired.
    ///
    /// Returns: The ID for this operation.
    ///
    //fn create_actor(&self, create_func: Fn(&ActorManager, Option<Fundamental: Pointer>) -> Actor, userdata: Option<Fundamental: Pointer>) -> u64 {
    //    unsafe { TODO: call ffi:actor_manager_create_actor() }
    //}

    /// get_n_operations:
    /// @manager: A #ActorManager
    ///
    /// Retrieves the amount of operations left in the queue.
    ///
    /// Returns: Number of operations left to perform
    ///
    fn get_n_operations(&self) -> u32 {
        let manager = self.as_ref();
        // g_queue_get_length(manager.ops)
        unimplemented!()
    }

    /// get_stage:
    /// @manager: A #ActorManager
    ///
    /// Gets the #Stage the actor manager is associated with.
    ///
    /// Returns: (transfer none): The #Stage the actor is associated with.
    ///
    fn get_stage(&self) -> Option<Stage> {
        let manager = self.as_ref();
        let props = manager.props.borrow();

        // props.stage.clone()
        unimplemented!()
    }

    /// get_time_slice:
    /// @manager: A #ActorManager
    ///
    /// Retrieves the current time slice being used for operations.
    ///
    /// Returns: The time-slice being used, in milliseconds
    ///
    fn get_time_slice(&self) -> u32 {
        let manager = self.as_ref();
        manager.props.borrow().time_slice
    }

    /// remove_actor:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    /// @actor: A #Actor
    ///
    /// Removes @actor from @container.
    ///
    /// On successful completion, the #ActorManager::actor_removed signal will
    /// be fired.
    ///
    /// <note><para>
    /// The actor may not be removed immediately, and thus you may want to set
    /// the actor's opacity to 0 before calling this function.
    /// </para></note>
    ///
    /// Returns: The ID for this operation.
    ///
    fn remove_actor<P: Is<Actor>, Q: Is<Actor>>(&self, container: &P, actor: &Q) -> u64 {
        let manager = self.as_ref();
        let container = container.as_ref();
        let actor = actor.as_ref();

        // let op: ActorManagerOperation =
        //     manager.op_new(ACTOR_MANAGER_REMOVE, None, None, actor, container);
        // manager.ensure_processing();
        // op.id

        unimplemented!()
    }

    /// remove_container:
    /// @manager: A #ActorManager
    /// @container: A #Actor
    ///
    /// Removes the container. This is a utility function that works by first
    /// removing all the children of the container, then the children itself. This
    /// effectively spreads the load of removing a large container. All prior
    /// operations associated with this container will be cancelled.
    ///
    /// <note><para>
    /// The container may not be removed immediately, and thus you may want to set
    /// the container's opacity to 0 before calling this function.
    /// </para></note>
    ///
    fn remove_container<P: Is<Actor>>(&self, container: &P) {
        let manager = self.as_ref();
        let container = container.as_ref();

        // /* Cancel all operations on this container */
        // manager.cancel_operations(container);

        // /* Remove all children */
        // let children: GList = container.get_children ();
        // while children {
        //     let child: Actor = children.data;
        //     manager.op_new(ACTOR_MANAGER_REMOVE, None, None, child, container);
        //     children = g_list_delete_link (children, children);
        // }

        // /* Then remove the container */
        // let parent: Actor = actor_get_parent (CLUTTER_ACTOR (container));
        // if parent && CLUTTER_IS_CONTAINER (parent) {
        //     g_object_ref (container);
        //     actor_remove_child (parent, container);
        //     manager.op_new (ACTOR_MANAGER_UNREF, None, None, (ClutterActor *)container, None);
        // }

        // manager.ensure_processing();
    }

    /// set_time_slice:
    /// @manager: A #ActorManager
    /// @msecs: A time, in milliseconds
    ///
    /// Sets the amount of time the actor manager will spend performing operations,
    /// before yielding to allow any necessary redrawing to occur.
    ///
    /// Lower times will lead to smoother performance, but will increase the amount
    /// of time it takes for operations to complete.
    ///
    fn set_time_slice(&self, msecs: u32) {
        let manager = self.as_ref();
        let mut props = manager.props.borrow_mut();
        if props.time_slice != msecs {
            props.time_slice = msecs;
            // g_object_notify (G_OBJECT (manager), "time-slice");
        }
    }

    // fn connect_actor_added<
    //     F: Fn(&Self, u64, &Actor, &Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn actor_added_trampoline<
    //     //     P,
    //     //     F: Fn(&P, u64, &Actor, &Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
    //     //     container: *mut ffi::ClutterActor,
    //     //     actor: *mut ffi::ClutterActor,
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
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-added\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_added_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_actor_created<F: Fn(&Self, u64, &Actor) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn actor_created_trampoline<
    //     //     P,
    //     //     F: Fn(&P, u64, &Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
    //     //     actor: *mut ffi::ClutterActor,
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
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-created\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_created_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_actor_finished<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn actor_finished_trampoline<P, F: Fn(&P, &Actor) + 'static>(
        //     this: *mut ffi::ActorManager,
        //     actor: *mut ffi::ClutterActor,
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"actor-finished\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             actor_finished_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    // fn connect_actor_removed<
    //     F: Fn(&Self, u64, &Actor, &Actor) + 'static,
    // >(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn actor_removed_trampoline<
    //     //     P,
    //     //     F: Fn(&P, u64, &Actor, &Actor) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
    //     //     container: *mut ffi::ClutterActor,
    //     //     actor: *mut ffi::ClutterActor,
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
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"actor-removed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             actor_removed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_cancelled<F: Fn(&Self, u64) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn operation_cancelled_trampoline<P, F: Fn(&P, u64) + 'static>(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref(), id)
    //     // }
    //     // unsafe {
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-cancelled\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_cancelled_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_completed<F: Fn(&Self, u64) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn operation_completed_trampoline<P, F: Fn(&P, u64) + 'static>(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<ActorManager>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(&ActorManager::from_glib_borrow(this).unsafe_cast_ref(), id)
    //     // }
    //     // unsafe {
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-completed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_completed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    // fn connect_operation_failed<F: Fn(&Self, u64, &glib::Error) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     // unsafe extern "C" fn operation_failed_trampoline<
    //     //     P,
    //     //     F: Fn(&P, u64, &glib::Error) + 'static,
    //     // >(
    //     //     this: *mut ffi::ActorManager,
    //     //     id: u64,
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
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"operation-failed\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             operation_failed_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_property_n_operations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::n-operations\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_n_operations_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_time_slice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::time-slice\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_time_slice_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
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
