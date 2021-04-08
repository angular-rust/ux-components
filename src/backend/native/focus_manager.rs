#![allow(unused_variables)]

// use std::mem::transmute;
use crate::prelude::*;
use crate::{FocusDirection, FocusHint, Focusable};
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct FocusManagerProps {
    pub stage: Option<clutter::Stage>,

    pub focused: Option<Focusable>,
    pub focused_toplevel: Option<Focusable>,
    pub refocus_idle: u32,
}

#[derive(Clone, Debug)]
pub struct FocusManager {
    props: RefCell<FocusManagerProps>,
}

impl FocusManager {
    //pub fn get_for_stage(stage: &clutter::Stage) -> Option<FocusManager> {
    //    unsafe { TODO: call ffi:focus_manager_get_for_stage() }
    //}
}

impl Object for FocusManager {}
impl Is<FocusManager> for FocusManager {}

impl AsRef<FocusManager> for FocusManager {
    fn as_ref(&self) -> &FocusManager {
        self
    }
}

pub const NONE_FOCUS_MANAGER: Option<&FocusManager> = None;

pub trait FocusManagerExt: 'static {
    /// focus_manager_get_focused:
    /// @manager: A #FocusManager
    ///
    /// Get the currently focused #Focusable
    ///
    /// Returns: (transfer none): Focusable
    ///
    fn get_focused(&self) -> Option<Focusable>;

    /// focus_manager_get_stage:
    /// @manager: A #FocusManager
    ///
    /// Get the stage the FocusManager is associated with
    ///
    /// Returns: (transfer none): A #ClutterStage
    ///
    fn get_stage(&self) -> Option<clutter::Stage>;

    /// focus_manager_move_focus:
    /// @manager: the focus manager
    /// @direction: The direction to move focus in
    ///
    /// Moves the current focus in the given direction.
    ///
    fn move_focus(&self, direction: FocusDirection);

    /// focus_manager_push_focus:
    /// @manager: the focus manager
    /// @focusable: the object to set focus on
    ///
    /// Sets the currently focused actor, with an #FocusHint of
    /// %FOCUS_HINT_PRIOR.
    ///
    /// Note: the final focused object may not be the same as @focusable if
    /// @focusable does not accept focus directly.
    ///
    fn push_focus(&self, focusable: &Focusable);

    /// focus_manager_push_focus_with_hint:
    /// @manager: the focus manager
    /// @focusable: the object to set focus on
    /// @hint: an #FocusHint
    ///
    /// Similar to #focus_manager_push_focus, but allows the hint to be specified.
    ///
    /// Note: the final focused object may not be the same as @focusable if
    /// @focusable does not accept focus directly.
    ///
    fn push_focus_with_hint(&self, focusable: &Focusable, hint: FocusHint);

    fn connect_property_focused_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<FocusManager>> FocusManagerExt for O {
    /// focus_manager_get_focused:
    /// @manager: A #FocusManager
    ///
    /// Get the currently focused #Focusable
    ///
    /// Returns: (transfer none): Focusable
    ///
    fn get_focused(&self) -> Option<Focusable> {
        let focusmanager = self.as_ref();
        let props = focusmanager.props.borrow();

        props.focused.clone()
    }

    /// focus_manager_get_stage:
    /// @manager: A #FocusManager
    ///
    /// Get the stage the FocusManager is associated with
    ///
    /// Returns: (transfer none): A #ClutterStage
    ///
    fn get_stage(&self) -> Option<clutter::Stage> {
        let focusmanager = self.as_ref();
        let props = focusmanager.props.borrow();

        props.stage.clone()
    }

    /// focus_manager_move_focus:
    /// @manager: the focus manager
    /// @direction: The direction to move focus in
    ///
    /// Moves the current focus in the given direction.
    ///
    fn move_focus(&self, direction: FocusDirection) {
        let focusmanager = self.as_ref();
        let props = focusmanager.props.borrow();

        // let mut new_focused;

        let old_focus = props.focused.clone();

        if props.focused.is_some() {
            // new_focused = focusable_move_focus(focusmanager.focused,
            //                                         direction,
            //                                         focusmanager.focused);
            // focus_manager_set_focused(manager, new_focused);
        } else {
            // // If we're going next or previous, we wrap around, otherwise
            // // re-focus the last actor.
            // match direction {
            //     FocusDirection::Next => {
            //         if old_focus {
            //             focus_manager_start_focus(manager, FocusHint::First);
            //         } else {
            //             focus_manager_ensure_focused(
            //                 manager,
            //                 CLUTTER_STAGE(focusmanager.stage),
            //                 FocusHint::First,
            //             );
            //         }
            //     }
            //     FocusDirection::Previous => {
            //         if old_focus {
            //             focus_manager_start_focus(manager, FocusHint::Last);
            //         } else {
            //             focus_manager_ensure_focused(
            //                 manager,
            //                 CLUTTER_STAGE(focusmanager.stage),
            //                 FocusHint::Last,
            //             );
            //         }
            //     }
            //     _ => {
            //         // re-focus the original
            //         if old_focus && (direction != FocusDirection::Out) {
            //             new_focused = focusable_accept_focus(old_focus, 0);
            //             focus_manager_set_focused(manager, new_focused);
            //         } else {
            //             focus_manager_ensure_focused(
            //                 manager,
            //                 CLUTTER_STAGE(focusmanager.stage),
            //                 FocusHint::First,
            //             );
            //         }
            //     }
            // }
        }

        // // Notify if the focus has changed
        // if props.focused != old_focus {
        //     g_object_notify(G_OBJECT(manager), "focused");
        // }
    }

    /// focus_manager_push_focus:
    /// @manager: the focus manager
    /// @focusable: the object to set focus on
    ///
    /// Sets the currently focused actor, with an #FocusHint of
    /// %FOCUS_HINT_PRIOR.
    ///
    /// Note: the final focused object may not be the same as @focusable if
    /// @focusable does not accept focus directly.
    ///
    fn push_focus(&self, focusable: &Focusable) {
        let focusmanager = self.as_ref();

        // if focusmanager.focused != focusable {
        //     if focusmanager.focused {
        //         // notify the current focusable that focus is being moved
        //         focusable_move_focus(
        //             focusmanager.focused,
        //             FocusDirection::Out,
        //             focusmanager.focused,
        //         );
        //     }

        //     let new_focused = focusable_accept_focus(focusable, FocusHint::Prior);
        //     focus_manager_set_focused(manager, new_focused);

        //     g_object_notify(G_OBJECT(manager), "focused");
        // }
    }

    /// focus_manager_push_focus_with_hint:
    /// @manager: the focus manager
    /// @focusable: the object to set focus on
    /// @hint: an #FocusHint
    ///
    /// Similar to #focus_manager_push_focus, but allows the hint to be specified.
    ///
    /// Note: the final focused object may not be the same as @focusable if
    /// @focusable does not accept focus directly.
    ///
    fn push_focus_with_hint(&self, focusable: &Focusable, hint: FocusHint) {
        let focusmanager = self.as_ref();

        // if focusmanager.focused != focusable {
        //     if focusmanager.focused {
        //         // notify the current focusable that focus is being moved
        //         focusable_move_focus(
        //             focusmanager.focused,
        //             FocusDirection::Out,
        //             focusmanager.focused,
        //         );
        //     }

        //     let new_focused = focusable_accept_focus(focusable, hint);
        //     focus_manager_set_focused(manager, new_focused);

        //     g_object_notify(G_OBJECT(manager), "focused");
        // }
    }

    fn connect_property_focused_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_focused_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FocusManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FocusManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FocusManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::focused\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_focused_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_stage_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::FocusManager,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<FocusManager>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&FocusManager::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::stage\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_stage_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for FocusManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FocusManager")
    }
}
