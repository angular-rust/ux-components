#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Timeline, Widget};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct ToggleHandle {
    pub parent: Widget,
}

#[derive(Clone, Debug)]
pub struct ToggleProps {
    pub active: bool,
    pub handle: Option<Actor>,
    pub handle_filename: String,
    pub timeline: Option<Timeline>,
    pub position: f32,
    pub drag_offset: f32,
    pub slide_length: f32,
    pub last_move: f32,
}

#[derive(Debug)]
pub struct Toggle {
    props: RefCell<ToggleProps>,
    widget: Widget,
}

impl Toggle {
    pub fn new() -> Toggle {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::toggle_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Toggle {}
impl Is<Toggle> for Toggle {}

impl AsRef<Toggle> for Toggle {
    fn as_ref(&self) -> &Toggle {
        self
    }
}

impl Is<Widget> for Toggle {}

impl AsRef<Widget> for Toggle {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Toggle {}

impl AsRef<Actor> for Toggle {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait ToggleExt: 'static {
    fn get_active(&self) -> bool;

    fn set_active(&self, active: bool);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Toggle>> ToggleExt for O {
    fn get_active(&self) -> bool {
        let toggle = self.as_ref();
        let props = toggle.props.borrow();

        props.active
    }

    fn set_active(&self, active: bool) {
        let toggle = self.as_ref();
        let mut props = toggle.props.borrow_mut();

        if props.active != active || (props.position > 0.0 && props.position < 1.0) {
            props.active = active;
            if active {
                // stylable_set_style_pseudo_class(MX_STYLABLE(toggle), "checked");
            } else {
                // stylable_set_style_pseudo_class(MX_STYLABLE(toggle), None);
            }

            // g_object_notify(G_OBJECT(toggle), "active");

            // // don't run an animation if the actor is not mapped
            // if !CLUTTER_ACTOR_IS_MAPPED(CLUTTER_ACTOR(toggle)) {
            //     toggle.position = (active) ? 1 : 0;
            //     return;
            // }

            if active {
                // clutter_timeline_set_direction(toggle.timeline, CLUTTER_TIMELINE_FORWARD);
            } else {
                // clutter_timeline_set_direction(toggle.timeline, CLUTTER_TIMELINE_BACKWARD);
            }

            // if clutter_timeline_is_playing (toggle.timeline) {
            //     return;
            // }

            // clutter_timeline_rewind(toggle.timeline);

            if props.drag_offset > -1.0 {
                // clutter_timeline_set_progress_mode(toggle.timeline, CLUTTER_LINEAR);
                // clutter_timeline_advance(toggle.timeline, toggle.position * 300);
            } else {
                // clutter_timeline_set_progress_mode(toggle.timeline, CLUTTER_EASE_IN_OUT_CUBIC);
            }

            // clutter_timeline_start(toggle.timeline);
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Toggle,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Toggle>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Toggle::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toggle")
    }
}
