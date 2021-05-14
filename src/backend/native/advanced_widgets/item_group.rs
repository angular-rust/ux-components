#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Button, HandlerId};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct ItemGroupProps {
    pub active_button: Option<Button>,
    pub children: Vec<Button>,
    pub allow_no_active: bool,
}

#[derive(Debug)]
pub struct ItemGroup {
    props: RefCell<ItemGroupProps>,
}

impl ItemGroup {
    pub fn new() -> ItemGroup {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::button_group_new()) }
        unimplemented!()
    }
}

impl Default for ItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ItemGroup {}
impl Is<ItemGroup> for ItemGroup {}

impl AsRef<ItemGroup> for ItemGroup {
    fn as_ref(&self) -> &ItemGroup {
        self
    }
}

pub trait ItemGroupExt: 'static {
    /// add:
    /// @group: A #ItemGroup
    /// @button: A #Button
    ///
    /// Add @button to the #ItemGroup.
    ///
    fn add<P: Is<Button>>(&self, button: &P);

    /// foreach:
    /// @group: A #ItemGroup
    /// @callback: (scope call): A #Callback
    /// @userdata: (closure): A #gpointer
    ///
    /// Calls @callback for each button in the group.
    ///
    fn foreach<P: FnMut(&Actor)>(&self, callback: P);

    /// get_active_button:
    /// @group: A #ItemGroup
    ///
    /// Get the current active button
    ///
    /// Returns: (transfer none): the currently active button
    ///
    fn get_active_button(&self) -> &Option<Button>;

    /// get_allow_no_active:
    /// @group: A #ItemGroup
    ///
    /// Get the value of the #ItemGroup:allow-no-active property.
    ///
    /// Returns: the value of the "allow-no-active" property.
    ///
    fn get_allow_no_active(&self) -> bool;

    /// get_buttons:
    /// @group: A #ItemGroup
    ///
    /// Get a list of the buttons in the button group.
    ///
    /// Returns: (element-type .Button): a list of buttons. The list is
    /// owned by the #ItemGroup and should not be modified by the application.
    ///
    fn get_buttons(&self) -> &Vec<Button>;

    /// remove:
    /// @group: A #ItemGroup
    /// @button: A #Button
    ///
    /// Remove @button from the #ItemGroup
    ///
    fn remove<P: Is<Button>>(&self, button: &P);

    /// set_active_button:
    /// @group: A #ItemGroup
    /// @button: (allow-none): A #Button
    ///
    /// Set the current active button in the group. The previous active button will
    /// have #Button:toggled set to #false.
    ///
    fn set_active_button<P: Is<Button>>(&self, button: Option<&P>);

    /// set_allow_no_active:
    /// @group: A #ItemGroup
    /// @allow_no_active: A #gboolean
    ///
    /// Set the value of the #ItemGroup:allow-no-active property.
    ///
    fn set_allow_no_active(&self, allow_no_active: bool);

    fn connect_property_active_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_allow_no_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<ItemGroup>> ItemGroupExt for O {
    /// add:
    /// @group: A #ItemGroup
    /// @button: A #Button
    ///
    /// Add @button to the #ItemGroup.
    ///
    fn add<P: Is<Button>>(&self, button: &P) {
        let buttongroup = self.as_ref();
        let button = button.as_ref();

        let props = buttongroup.props.borrow();

        // buttongroup.children.push(button);
        // g_signal_connect (button, "notify::toggled",
        //                     G_CALLBACK (button_toggled_notify_cb), group);
        // g_signal_connect (button, "button-press-event",
        //                     G_CALLBACK (button_click_intercept), group);
        // g_signal_connect (button, "button-release-event",
        //                     G_CALLBACK (button_click_intercept), group);
        // g_signal_connect (button, "touch-event",
        //                     G_CALLBACK (button_click_intercept), group);

        // g_object_weak_ref (G_OBJECT (button), (GWeakNotify) button_weak_notify,
        //                     group);

        if !props.allow_no_active && props.active_button.is_none() {
            button.set_toggled(true);
        }
    }

    /// foreach:
    /// @group: A #ItemGroup
    /// @callback: (scope call): A #Callback
    /// @userdata: (closure): A #gpointer
    ///
    /// Calls @callback for each button in the group.
    ///
    fn foreach<P: FnMut(&Actor)>(&self, callback: P) {
        let buttongroup = self.as_ref();
        // g_return_if_fail (IS_BUTTON_GROUP (group));
        // g_return_if_fail (callback != None);

        // g_slist_foreach (buttongroup.children, (GFunc) callback, userdata);
    }

    /// get_active_button:
    /// @group: A #ItemGroup
    ///
    /// Get the current active button
    ///
    /// Returns: (transfer none): the currently active button
    ///
    fn get_active_button(&self) -> &Option<Button> {
        let buttongroup = self.as_ref();
        // &buttongroup.props.borrow().active_button // TODO: OOPS
        unimplemented!()
    }

    /// get_allow_no_active:
    /// @group: A #ItemGroup
    ///
    /// Get the value of the #ItemGroup:allow-no-active property.
    ///
    /// Returns: the value of the "allow-no-active" property.
    ///
    fn get_allow_no_active(&self) -> bool {
        let buttongroup = self.as_ref();
        buttongroup.props.borrow().allow_no_active
    }

    /// get_buttons:
    /// @group: A #ItemGroup
    ///
    /// Get a list of the buttons in the button group.
    ///
    /// Returns: (element-type .Button): a list of buttons. The list is
    /// owned by the #ItemGroup and should not be modified by the application.
    ///
    fn get_buttons(&self) -> &Vec<Button> {
        let buttongroup = self.as_ref();
        // &buttongroup.props.borrow().children // TODO: OOPS
        unimplemented!()
    }

    /// remove:
    /// @group: A #ItemGroup
    /// @button: A #Button
    ///
    /// Remove @button from the #ItemGroup
    ///
    fn remove<P: Is<Button>>(&self, button: &P) {
        let buttongroup = self.as_ref();
        let button = button.as_ref();
        let props = buttongroup.props.borrow();

        // GSList *l, *prev = None, *next;
        // let mut found = false;

        // // check the button exists in this group
        // for btn in buttongroup.children.iter() {
        //     if (Button*) btn->data == button {
        //         found = true;
        //         break;
        //     }
        //     prev = btn;
        // }

        // if !found {
        //     return;
        // }

        // next = g_slist_next (l);
        // buttongroup.children = g_slist_remove (buttongroup.children, button);

        // g_signal_handlers_disconnect_by_func (button, button_toggled_notify_cb,
        //                                         group);
        // g_signal_handlers_disconnect_by_func (button, button_click_intercept, group);

        // g_object_weak_unref (G_OBJECT (button), (GWeakNotify) button_weak_notify, group);

        // if props.active_button == button {
        //     /* Try and select another button if the one we've removed is active.
        //     * But we shouldn't do this in the case where we allow no active button.
        //     */
        //     if props.allow_no_active {
        //         buttongroup.set_active_button (None);
        //     } else if prev {
        //         buttongroup.set_active_button ((Button *) prev->data);
        //     } else if next {
        //         buttongroup.set_active_button ((Button *) next->data);
        //     } else if buttongroup.children {
        //         buttongroup.set_active_button ((Button *) priv->children->data);
        //     } else {
        //         buttongroup.set_active_button (None);
        //     }
        // }
    }

    /// set_active_button:
    /// @group: A #ItemGroup
    /// @button: (allow-none): A #Button
    ///
    /// Set the current active button in the group. The previous active button will
    /// have #Button:toggled set to #false.
    ///
    fn set_active_button<P: Is<Button>>(&self, button: Option<&P>) {
        let buttongroup = self.as_ref();
        let button = button.as_ref();
        let mut props = buttongroup.props.borrow_mut();

        // if let Some(active_button) = props.active_button {
        //     if let Some(button) = button {
        //         // if *button == active_button {
        //         //     return;
        //         // }
        //     }
        //     active_button.set_toggled(false);
        // }

        // if let Some(button) = button {
        //     button.set_toggled(true);
        // }

        match button {
            Some(button) => {
                let button = button.as_ref();
                // props.active_button = Some(button); //TODO: OOPS
            }
            None => {
                props.active_button = None;
            }
        }

        // g_object_notify (G_OBJECT (group), "active-button");
    }

    /// set_allow_no_active:
    /// @group: A #ItemGroup
    /// @allow_no_active: A #gboolean
    ///
    /// Set the value of the #ItemGroup:allow-no-active property.
    ///
    fn set_allow_no_active(&self, allow_no_active: bool) {
        let buttongroup = self.as_ref();
        let mut props = buttongroup.props.borrow_mut();

        if props.allow_no_active != allow_no_active {
            props.allow_no_active = allow_no_active;
            // g_object_notify (G_OBJECT (group), "allow-no-active");
        }
    }

    fn connect_property_active_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_active_button_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemGroup,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemGroup>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-button\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_button_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_allow_no_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_allow_no_active_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ItemGroup,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ItemGroup>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::allow-no-active\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_allow_no_active_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ItemGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ItemGroup")
    }
}
