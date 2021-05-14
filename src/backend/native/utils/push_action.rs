#![allow(unused_variables)]

use crate::prelude::*;
use crate::HandlerId;
use std::{boxed::Box as Box_, cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct PushActionProps {
    pub name: Option<String>,
    // pub parameter_type: GVariantType,
    pub enabled: bool,
    pub state_set: bool,
    // pub state: GVariant,
    pub display_name: Option<String>,
    pub icon: Option<String>,
}
#[derive(Clone, Debug)]
pub struct PushAction {
    props: RefCell<PushActionProps>,
}

impl PushAction {
    pub fn new() -> PushAction {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::action_new()) }
        unimplemented!()
    }

    pub fn new_full(
        name: &str,
        display_name: &str,
        activated_cb: Option<Box_<dyn FnOnce(&PushAction) + 'static>>,
    ) -> PushAction {
        // assert_initialized_main_thread!();
        // let activated_cb_data: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> =
        //     Box_::new(activated_cb);
        // unsafe extern "C" fn activated_cb_func(
        //     action: *mut ffi::Action,
        //     user_data: glib_sys::gpointer,
        // ) {
        //     let action = from_glib_borrow(action);
        //     let callback: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> =
        //         Box_::from_raw(user_data as *mut _);
        //     let callback = (*callback).expect("cannot get closure...");
        //     callback(&action)
        // }
        // let activated_cb = if activated_cb_data.is_some() {
        //     Some(activated_cb_func as _)
        // } else {
        //     None
        // };
        // let super_callback0: Box_<Option<Box_<dyn FnOnce(&Action) + 'static>>> = activated_cb_data;
        // unsafe {
        //     from_glib_full(ffi::action_new_full(
        //         name.to_glib_none().0,
        //         display_name.to_glib_none().0,
        //         activated_cb,
        //         Box_::into_raw(super_callback0) as *mut _,
        //     ))
        // }
        unimplemented!()
    }

    // pub fn new_stateful(
    //     name: &str,
    //     parameter_type: Option<&glib::VariantTy>,
    //     state: &glib::Variant,
    // ) -> PushAction {
    //     // assert_initialized_main_thread!();
    //     // unsafe {
    //     //     from_glib_none(ffi::action_new_stateful(
    //     //         name.to_glib_none().0,
    //     //         parameter_type.to_glib_none().0,
    //     //         state.to_glib_none().0,
    //     //     ))
    //     // }
    //     unimplemented!()
    // }

    // pub fn with_parameter(name: &str, parameter_type: Option<&glib::VariantTy>) -> PushAction {
    //     // assert_initialized_main_thread!();
    //     // unsafe {
    //     //     from_glib_none(ffi::action_new_with_parameter(
    //     //         name.to_glib_none().0,
    //     //         parameter_type.to_glib_none().0,
    //     //     ))
    //     // }
    //     unimplemented!()
    // }
}

impl Default for PushAction {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for PushAction {}
impl Is<PushAction> for PushAction {}

impl AsRef<PushAction> for PushAction {
    fn as_ref(&self) -> &PushAction {
        self
    }
}

pub trait PushActionExt: 'static {
    /// get_active:
    /// @action: A #Action
    ///
    /// Get the value of the active property
    ///
    /// Returns: #true if the action is active
    ///
    fn get_active(&self) -> bool;

    /// get_display_name:
    /// @action: A #Action
    ///
    /// Get the display name of the action
    ///
    /// Returns: display-name of the action, owned by Action
    ///
    fn get_display_name(&self) -> Option<String>;

    /// get_icon:
    /// @action: A #Action
    ///
    /// Get the icon of the action
    ///
    /// Returns: icon of the action, owned by Action
    ///
    fn get_icon(&self) -> Option<String>;

    /// get_name:
    /// @action: A #Action
    ///
    /// Get the name of the action
    ///
    /// Returns: name of the action, owned by Action
    ///
    fn get_name(&self) -> Option<String>;

    /// set_active:
    /// @action: A #Action
    /// @active: the value to set
    ///
    /// Set the value of the active property
    ///
    fn set_active(&self, active: bool);

    /// set_display_name:
    /// @action: A #Action
    /// @name: new display name to set
    ///
    /// Set the name of the action to display to the user
    ///
    fn set_display_name(&self, name: &str);

    /// set_icon:
    /// @action: A #Action
    /// @name: new icon to set
    ///
    /// The icon to be used in a visual representation of an action.
    ///
    fn set_icon(&self, name: &str);

    /// set_name:
    /// @action: A #Action
    /// @name: new name to set
    ///
    /// Set the name of the action
    ///
    fn set_name(&self, name: &str);

    // fn connect_activate<F: Fn(&Self, Option<&glib::Variant>) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_display_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<PushAction>> PushActionExt for O {
    /// get_active:
    /// @action: A #PushAction
    ///
    /// Get the value of the active property
    ///
    /// Returns: #true if the action is active
    ///
    fn get_active(&self) -> bool {
        let action = self.as_ref();
        action.props.borrow().enabled
    }

    /// get_display_name:
    /// @action: A #PushAction
    ///
    /// Get the display name of the action
    ///
    /// Returns: display-name of the action, owned by PushAction
    ///
    fn get_display_name(&self) -> Option<String> {
        let action = self.as_ref();
        action.props.borrow().display_name.clone()
    }

    /// get_icon:
    /// @action: A #PushAction
    ///
    /// Get the icon of the action
    ///
    /// Returns: icon of the action, owned by PushAction
    ///
    fn get_icon(&self) -> Option<String> {
        let action = self.as_ref();
        action.props.borrow().icon.clone()
    }

    /// get_name:
    /// @action: A #PushAction
    ///
    /// Get the name of the action
    ///
    /// Returns: name of the action, owned by PushAction
    ///
    fn get_name(&self) -> Option<String> {
        let action = self.as_ref();
        action.props.borrow().name.clone()
    }

    /// set_active:
    /// @action: A #PushAction
    /// @active: the value to set
    ///
    /// Set the value of the active property
    ///
    fn set_active(&self, active: bool) {
        let action = self.as_ref();
        let mut props = action.props.borrow_mut();

        if props.enabled != active {
            props.enabled = active;
            // g_object_notify(G_OBJECT(action), "active");
        }
    }

    /// set_display_name:
    /// @action: A #PushAction
    /// @name: new display name to set
    ///
    /// Set the name of the action to display to the user
    ///
    fn set_display_name(&self, name: &str) {
        let action = self.as_ref();
        let mut props = action.props.borrow_mut();

        match &props.display_name {
            Some(display_name) => {
                if display_name.as_str() != name {
                    props.display_name = Some(name.into());
                    // g_object_notify (G_OBJECT (action), "display-name");
                }
            }
            None => {
                props.display_name = Some(name.into());
                // g_object_notify (G_OBJECT (action), "display-name");
            }
        }
    }

    /// set_icon:
    /// @action: A #PushAction
    /// @name: new icon to set
    ///
    /// The icon to be used in a visual representation of an action.
    ///
    fn set_icon(&self, name: &str) {
        let action = self.as_ref();
        let mut props = action.props.borrow_mut();

        match &props.icon {
            Some(icon) => {
                if icon.as_str() != name {
                    props.icon = Some(name.into());
                    // g_object_notify (G_OBJECT (action), "icon");
                }
            }
            None => {
                props.icon = Some(name.into());
                // g_object_notify (G_OBJECT (action), "icon");
            }
        }
    }

    /// set_name:
    /// @action: A #PushAction
    /// @name: new name to set
    ///
    /// Set the name of the action
    ///
    fn set_name(&self, name: &str) {
        let action = self.as_ref();
        let mut props = action.props.borrow_mut();

        match &props.name {
            Some(name) => {
                if name.as_str() != name {
                    props.name = Some(name.into());
                    // g_object_notify (G_OBJECT (action), "name");
                }
            }
            None => {
                props.name = Some(name.into());
                // g_object_notify (G_OBJECT (action), "name");
            }
        }
    }

    // fn connect_activate<F: Fn(&Self, Option<&glib::Variant>) + 'static>(&self, f: F) -> HandlerId {
    //     // unsafe extern "C" fn activate_trampoline<P, F: Fn(&P, Option<&glib::Variant>) + 'static>(
    //     //     this: *mut ffi::Action,
    //     //     parameter: *mut glib_sys::GVariant,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<Action>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &Action::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         Option::<glib::Variant>::from_glib_borrow(parameter)
    //     //             .as_ref()
    //     //             .as_ref(),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box_<F> = Box_::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"activate\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             activate_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box_::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_property_display_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_display_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Action,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Action>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::display-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_display_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Action,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Action>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for PushAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PushAction")
    }
}
