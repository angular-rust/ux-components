#![allow(unused_variables)]

use super::{Action, Widget, Button};
use crate::prelude::*;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Dialog {
    pub child: Option<clutter::Actor>,

    pub visible: bool,
    pub child_has_focus: bool,

    pub transition_time: u32,
    pub angle: f32,

    pub timeline: Option<clutter::Timeline>,
    pub zoom: f32,

    // Dialog-specific variables
    pub background: Option<clutter::Actor>,
    pub button_box: Option<clutter::Actor>,
    pub spacing: u32,
    pub actions: Vec<Action>,
}

impl Dialog {
    pub fn new() -> Dialog {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::dialog_new()).unsafe_cast() }
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

pub const NONE_DIALOG: Option<&Dialog> = None;

pub trait DialogExt: 'static {
    /// add_action:
    /// @dialog: A #Dialog
    /// @action: A #Action
    ///
    /// Adds an #Button that represents @action to the button area of @dialog
    ///
    fn add_action<P: Is<Action>>(&self, action: &P);

    /// get_actions:
    /// @dialog: A #Dialog
    ///
    /// Retrieves a list of actions added to @dialog.
    ///
    /// Returns: (transfer container) (element-type Action): A newly allocated
    ///   #GList of #Action objects. The actions in the list are owned by the dialog.
    ///
    fn get_actions(&self) -> Vec<Action>;

    /// remove_action:
    /// @dialog: A #Dialog
    /// @action: A #Action
    ///
    /// Removes the button associated with @action from the button area of @dialog
    ///
    fn remove_action<P: Is<Action>>(&self, action: &P);

    /// set_transient_parent:
    /// @dialog: A #Dialog
    /// @actor: A #ClutterActor
    ///
    /// Sets the parent of the #Dialog. This is the actor over which the
    /// modal frame will appear when clutter_actor_show() is called.
    ///
    fn set_transient_parent<P: Is<clutter::Actor>>(&self, actor: &P);
}

impl<O: Is<Dialog>> DialogExt for O {
    /// add_action:
    /// @dialog: A #Dialog
    /// @action: A #Action
    ///
    /// Adds an #Button that represents @action to the button area of @dialog
    ///
    fn add_action<P: Is<Action>>(&self, action: &P) {
        let dialog = self.as_ref();
        let action = action.as_ref();
      
        let button = Button::new();
        button.set_action(action);
        // clutter_actor_add_child (dialog.button_box, button);
      
        // /* So we can maintain the two way relationship between action and button */
        // let da: DialogAction = g_slice_new (DialogAction);
        // da.action = action;
        // da.button = button;
        // dialog.actions.push(da);
    }

    /// get_actions:
    /// @dialog: A #Dialog
    ///
    /// Retrieves a list of actions added to @dialog.
    ///
    /// Returns: (transfer container) (element-type Action): A newly allocated
    ///   #GList of #Action objects. The actions in the list are owned by the dialog.
    ///
    fn get_actions(&self) -> Vec<Action> {
        let dialog = self.as_ref();
        
        // GList *a, *list;
        // list = NULL;

        // for (a = dialog.actions; a; a = a.next) {
        //     DialogAction *da = a.data;
        //     list = g_list_prepend (list, da.action);
        // }

        // g_list_reverse (list)
        unimplemented!()
    }

    /// remove_action:
    /// @dialog: A #Dialog
    /// @action: A #Action
    ///
    /// Removes the button associated with @action from the button area of @dialog
    ///
    fn remove_action<P: Is<Action>>(&self, action: &P) {
        let dialog = self.as_ref();
        let action = action.as_ref();
        
        // DialogAction *da;
        // GList *a;

        // da = NULL;
        // for (a = dialog.actions; a; a = a.next) {
        //     DialogAction *data = a.data;

        //     if (data.action == action) {
        //         dialog.actions = g_list_delete_link (dialog.actions, a);
        //         da = data;
        //         break;
        //     }
        // }

        // if (da == NULL) {
        //     g_warning ("Action '%s' was not found in dialog",
        //                 action_get_name (action));
        //     return;
        // }

        // clutter_actor_remove_child (dialog.button_box, da.button);
        // g_slice_free (DialogAction, da);
    }

    /// set_transient_parent:
    /// @dialog: A #Dialog
    /// @actor: A #ClutterActor
    ///
    /// Sets the parent of the #Dialog. This is the actor over which the
    /// modal frame will appear when clutter_actor_show() is called.
    ///
    fn set_transient_parent<P: Is<clutter::Actor>>(&self, actor: &P) {
        let dialog = self.as_ref();
        let actor = actor.as_ref();

        // actor.add_child(CLUTTER_ACTOR (dialog));
        // clutter_actor_add_constraint(CLUTTER_ACTOR (dialog),
        //                                 clutter_bind_constraint_new(actor,
        //                                                             CLUTTER_BIND_SIZE,
        //                                                             0));
    }
}

impl fmt::Display for Dialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dialog")
    }
}
