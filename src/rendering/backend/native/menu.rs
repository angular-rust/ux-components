#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, FloatingWidget, HandlerId, PushAction, Widget};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct MenuChild {
    pub action: PushAction,
    pub widget: Widget, // called `box` before
}

#[derive(Debug)]
pub struct MenuProps {
    pub children: Vec<MenuChild>,
    pub transition_out: bool,
    pub stage: Option<Actor>,
    pub captured_event_handler: u64,
    pub internal_focus_push: bool,
    pub scrolling_mode: bool,
    pub id_offset: i32,
    pub last_shown_id: i32,
    pub up_button: Option<Actor>,
    pub down_button: Option<Actor>,
    pub up_source: u64,
    pub down_source: u64,
}

#[derive(Debug)]
pub struct Menu {
    props: RefCell<MenuProps>,
    widget: FloatingWidget,
}

impl Menu {
    pub fn new() -> Menu {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::menu_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Menu {}
impl Is<Menu> for Menu {}

impl AsRef<Menu> for Menu {
    fn as_ref(&self) -> &Menu {
        self
    }
}

impl Is<FloatingWidget> for Menu {}

impl AsRef<FloatingWidget> for Menu {
    fn as_ref(&self) -> &FloatingWidget {
        &self.widget
    }
}

impl Is<Widget> for Menu {}

impl AsRef<Widget> for Menu {
    fn as_ref(&self) -> &Widget {
        let widget: &Widget = self.widget.as_ref();
        widget
    }
}

impl Is<Actor> for Menu {}

impl AsRef<Actor> for Menu {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait MenuExt: 'static {
    /// add_action:
    /// @menu: A #Menu
    /// @action: A #PushAction
    ///
    /// Append @action to @menu.
    ///
    fn add_action<P: Is<PushAction>>(&self, action: &P);

    /// remove_action:
    /// @menu: A #Menu
    /// @action: A #PushAction
    ///
    /// Remove @action from @menu.
    ///
    fn remove_action<P: Is<PushAction>>(&self, action: &P);

    /// remove_all:
    /// @menu: A #Menu
    ///
    /// Remove all the actions from @menu.
    ///
    fn remove_all(&self);

    /// show_with_position:
    /// @menu: A #Menu
    /// @x: X position
    /// @y: Y position
    ///
    /// Moves the menu to the specified position and shows it.
    ///
    fn show_with_position(&self, x: f32, y: f32);

    fn connect_action_activated<F: Fn(&Self, &PushAction) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Menu>> MenuExt for O {
    /// add_action:
    /// @menu: A #Menu
    /// @action: A #PushAction
    ///
    /// Append @action to @menu.
    ///
    fn add_action<P: Is<PushAction>>(&self, action: &P) {
        let menu = self.as_ref();
        let action = action.as_ref();

        // let child: MenuChild;
        // child.action = g_object_ref_sink(action);
        // // TODO: Connect to notify signals in case action properties change
        // child.widget = g_object_new(TYPE_BUTTON,
        //                             "action", child.action,
        //                             None);
        // button_set_action(BUTTON (child.widget), child.action);

        // // align to the left
        // let button_child: Actor = actor_get_child_at_index((ClutterActor*)child.widget, 0);
        // actor_set_x_align(button_child, CLUTTER_ACTOR_ALIGN_START);

        // g_signal_connect(child.widget, "clicked",
        //                     G_CALLBACK (menu_button_clicked_cb), action);
        // g_signal_connect(child.widget, "enter-event",
        //                     G_CALLBACK (menu_button_enter_event_cb), menu);
        // actor_add_child(CLUTTER_ACTOR (menu), CLUTTER_ACTOR(child.widget));
        // g_array_append_val(menu.children, child);
        // actor_queue_relayout(CLUTTER_ACTOR(menu));
    }

    /// remove_action:
    /// @menu: A #Menu
    /// @action: A #PushAction
    ///
    /// Remove @action from @menu.
    ///
    fn remove_action<P: Is<PushAction>>(&self, action: &P) {
        let menu = self.as_ref();
        let action = action.as_ref();

        // for (i = 0; i < menu.children.len(); i++) {
        //     MenuChild *child = &g_array_index (menu.children, MenuChild, i);

        //     if child->action == action {
        //         menu_free_action_at (menu, i, true);
        //         break;
        //     }
        // }
    }

    /// remove_all:
    /// @menu: A #Menu
    ///
    /// Remove all the actions from @menu.
    ///
    fn remove_all(&self) {
        let menu = self.as_ref();

        // if !menu.children.len() {
        //     return;
        // }

        // for (i = 0; i < menu.children.len; i++) {
        //     menu_free_action_at(menu, i, false);
        // }

        // g_array_remove_range(menu.children, 0, menu.children.len());
    }

    /// show_with_position:
    /// @menu: A #Menu
    /// @x: X position
    /// @y: Y position
    ///
    /// Moves the menu to the specified position and shows it.
    ///
    fn show_with_position(&self, x: f32, y: f32) {
        let menu = self.as_ref();

        // actor_set_position(CLUTTER_ACTOR(menu), x, y);
        // actor_show(CLUTTER_ACTOR(menu));
    }

    fn connect_action_activated<F: Fn(&Self, &PushAction) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn action_activated_trampoline<P, F: Fn(&P, &Action) + 'static>(
        //     this: *mut ffi::Menu,
        //     object: *mut ffi::Action,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Menu>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(
        //         &Menu::from_glib_borrow(this).unsafe_cast_ref(),
        //         &from_glib_borrow(object),
        //     )
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"action-activated\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             action_activated_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
