#![allow(unused_variables)]

use crate::prelude::*;
use crate::{PushAction, Actor, Button, Timeline, Widget};
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct DialogProps {
    pub child: Option<Actor>,
    pub visible: bool,
    pub child_has_focus: bool,
    pub transition_time: u32,
    pub angle: f32,
    pub timeline: Option<Timeline>,
    pub zoom: f32,

    // Dialog-specific variables
    pub background: Option<Actor>,
    pub button_box: Option<Actor>,
    pub spacing: u32,
    pub actions: Vec<PushAction>,
}

#[derive(Clone, Debug)]
pub struct Dialog {
    props: RefCell<DialogProps>,
    widget: Widget,
}

impl Dialog {
    pub fn new() -> Dialog {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::dialog_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Dialog {}
impl Is<Dialog> for Dialog {}

impl AsRef<Dialog> for Dialog {
    fn as_ref(&self) -> &Dialog {
        self
    }
}

impl Is<Widget> for Dialog {}

impl AsRef<Widget> for Dialog {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Dialog {}

impl AsRef<Actor> for Dialog {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait DialogExt: 'static {
    /// add_action:
    /// @dialog: A #Dialog
    /// @action: A #PushAction
    ///
    /// Adds an #Button that represents @action to the button area of @dialog
    ///
    fn add_action<P: Is<PushAction>>(&self, action: &P);

    /// get_actions:
    /// @dialog: A #Dialog
    ///
    /// Retrieves a list of actions added to @dialog.
    ///
    /// Returns: (transfer container) (element-type PushAction): A newly allocated
    ///   #GList of #PushAction objects. The actions in the list are owned by the dialog.
    ///
    fn get_actions(&self) -> Vec<PushAction>;

    /// remove_action:
    /// @dialog: A #Dialog
    /// @action: A #PushAction
    ///
    /// Removes the button associated with @action from the button area of @dialog
    ///
    fn remove_action<P: Is<PushAction>>(&self, action: &P);

    /// set_transient_parent:
    /// @dialog: A #Dialog
    /// @actor: A #Actor
    ///
    /// Sets the parent of the #Dialog. This is the actor over which the
    /// modal frame will appear when actor_show() is called.
    ///
    fn set_transient_parent<P: Is<Actor>>(&self, actor: &P);
}

impl<O: Is<Dialog>> DialogExt for O {
    /// add_action:
    /// @dialog: A #Dialog
    /// @action: A #PushAction
    ///
    /// Adds an #Button that represents @action to the button area of @dialog
    ///
    fn add_action<P: Is<PushAction>>(&self, action: &P) {
        let dialog = self.as_ref();
        let action = action.as_ref();
        // let mut props = dialog.props.borrow_mut();

        let button = Button::new();
        button.set_action(action);
        // actor_add_child (props.button_box, button);

        // /* So we can maintain the two way relationship between action and button */
        // let da: DialogAction = g_slice_new (DialogAction);
        // da.action = action;
        // da.button = button;
        // props.actions.push(da);
    }

    /// get_actions:
    /// @dialog: A #Dialog
    ///
    /// Retrieves a list of actions added to @dialog.
    ///
    /// Returns: (transfer container) (element-type PushAction): A newly allocated
    ///   #GList of #PushAction objects. The actions in the list are owned by the dialog.
    ///
    fn get_actions(&self) -> Vec<PushAction> {
        let dialog = self.as_ref();

        // GList *a, *list;
        // list = None;

        // for (a = dialog.actions; a; a = a.next) {
        //     DialogAction *da = a.data;
        //     list = g_list_prepend (list, da.action);
        // }

        // g_list_reverse (list)
        unimplemented!()
    }

    /// remove_action:
    /// @dialog: A #Dialog
    /// @action: A #PushAction
    ///
    /// Removes the button associated with @action from the button area of @dialog
    ///
    fn remove_action<P: Is<PushAction>>(&self, action: &P) {
        let dialog = self.as_ref();
        let action = action.as_ref();

        // DialogAction *da;
        // GList *a;

        // da = None;
        // for (a = dialog.actions; a; a = a.next) {
        //     DialogAction *data = a.data;

        //     if (data.action == action) {
        //         dialog.actions = g_list_delete_link (dialog.actions, a);
        //         da = data;
        //         break;
        //     }
        // }

        // if (da == None) {
        //     g_warning ("Action '%s' was not found in dialog",
        //                 action_get_name (action));
        //     return;
        // }

        // actor_remove_child (dialog.button_box, da.button);
        // g_slice_free (DialogAction, da);
    }

    /// set_transient_parent:
    /// @dialog: A #Dialog
    /// @actor: A #Actor
    ///
    /// Sets the parent of the #Dialog. This is the actor over which the
    /// modal frame will appear when actor_show() is called.
    ///
    fn set_transient_parent<P: Is<Actor>>(&self, actor: &P) {
        let dialog = self.as_ref();
        let actor = actor.as_ref();

        // actor.add_child(CLUTTER_ACTOR (dialog));
        // actor_add_constraint(CLUTTER_ACTOR (dialog),
        //                                 bind_constraint_new(actor,
        //                                                             CLUTTER_BIND_SIZE,
        //                                                             0));
    }
}

impl fmt::Display for Dialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dialog")
    }
}
