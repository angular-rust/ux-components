#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Position, PushAction, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct ButtonProps {
    pub text: Option<String>,
    pub icon_name: Option<String>,
    pub style_icon_name: Option<String>,
    pub icon_size: u32,
    pub style_icon_size: u32,
    pub old_opacity: u8,
    pub is_pressed: bool,
    pub is_toggle: bool,
    pub is_toggled: bool,
    pub action: Option<PushAction>,
    pub icon_position: Position,
    pub icon_visible: bool,
    pub label_visible: bool,
    pub content_image: dx::Texture,
    // pub animation: Animation,
    pub child: Actor,
    pub hbox: Actor,
    pub icon: Actor,
    pub label: Actor,
    // pub action_label_binding: GBinding,
    // pub action_icon_binding: GBinding,
}

#[derive(Clone, Debug)]
pub struct Button {
    props: RefCell<ButtonProps>,
    widget: Widget,
}

impl Button {
    pub fn new() -> Button {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::button_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_label(text: &str) -> Button {
        // assert_initialized_main_thread!();
        // unsafe {
        //     Actor::from_glib_none(ffi::button_new_with_label(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Button {}
impl Is<Button> for Button {}

impl AsRef<Button> for Button {
    fn as_ref(&self) -> &Button {
        self
    }
}

impl Is<Widget> for Button {}

impl AsRef<Widget> for Button {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Button {}

impl AsRef<Actor> for Button {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait ButtonExt: 'static {
    /// get_action:
    /// @button: A #Button
    ///
    /// Retrieves the #PushAction associated with @button.
    ///
    /// Returns: (transfer none): A #PushAction
    ///
    fn get_action(&self) -> Option<PushAction>;

    /// get_icon_name:
    /// @button: a #Button
    ///
    /// Get the icon-name being used on the button.
    ///
    /// Returns: the icon-name. This must not be freed by the application. %None if
    ///   no icon has been set
    ///
    fn get_icon_name(&self) -> Option<String>;

    /// get_icon_position:
    /// @button: A #Button
    ///
    /// Retrieves the icon's relative position to the text.
    ///
    /// Returns: A #Position
    ///
    fn get_icon_position(&self) -> Position;

    /// get_icon_size:
    /// @button: a #Button
    ///
    /// Retrieves the icon-size being used for the displayed icon inside the button.
    ///
    /// Returns: The icon-size being used for the button icon, in pixels
    ///
    fn get_icon_size(&self) -> u32;

    /// get_icon_visible:
    /// @button: A #Button
    ///
    /// Retrieves the visibility of the icon associated with the button's action.
    ///
    /// Returns: %true if the icon is visible, %false otherwise
    ///
    fn get_icon_visible(&self) -> bool;

    /// get_is_toggle:
    /// @button: a #Button
    ///
    /// Get the toggle mode status of the button.
    ///
    /// Returns: #true if toggle mode is set, otherwise #false
    ///
    fn get_is_toggle(&self) -> bool;

    /// get_label:
    /// @button: a #Button
    ///
    /// Get the text displayed on the button
    ///
    /// Returns: the text for the button. This must not be freed by the application
    ///
    fn get_label(&self) -> Option<String>;

    /// get_label_visible:
    /// @button: A #Button
    ///
    /// Retrieves the visibility of the text associated with the button's action.
    ///
    /// Returns: %true if the text is visible, %false otherwise
    ///
    fn get_label_visible(&self) -> bool;

    /// get_toggled:
    /// @button: a #Button
    ///
    /// Get the state of the button that is in toggle mode.
    ///
    /// Returns: #true if the button is toggled, or #false if not
    ///
    fn get_toggled(&self) -> bool;

    /// set_action:
    /// @button: A #Button
    /// @action: A #PushAction
    ///
    /// Sets @action as the action for @button. @Button will take its label and
    /// icon from @action.
    ///
    fn set_action<P: Is<PushAction>>(&self, action: &P);

    /// set_icon_name:
    /// @button: a #Button
    /// @icon_name: (allow-none): icon-name to use on the button
    ///
    /// Sets the icon-name used to display an icon on the button. Setting %None
    /// will remove the icon name, or resort to the icon-name set in the current
    /// style. Setting an icon name overrides any icon set in the style.
    ///
    fn set_icon_name(&self, icon_name: Option<String>);

    /// set_icon_position:
    /// @button: A #Button
    /// @position: A #Position
    ///
    /// Sets the icon position, relative to the text on the button.
    ///
    fn set_icon_position(&self, position: Position);

    /// set_icon_size:
    /// @button: a #Button
    ///
    /// Sets the icon-size to use for the icon displayed inside the button. This will
    /// override the icon-size set in the style. Setting a value of %0 resets to the
    /// size from the style.
    ///
    fn set_icon_size(&self, icon_size: u32);

    /// set_icon_visible:
    /// @button: A #Button
    /// @visible: %true if the icon should be visible
    ///
    /// Sets the visibility of the icon associated with the button's action.
    ///
    fn set_icon_visible(&self, visible: bool);

    /// set_is_toggle:
    /// @button: a #Button
    /// @toggle: #true or #false
    ///
    /// Enables or disables toggle mode for the button. In toggle mode, the active
    /// state will be "toggled" when the user clicks the button.
    ///
    fn set_is_toggle(&self, toggle: bool);

    /// set_label:
    /// @button: a #Button
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the button
    ///
    fn set_label(&self, text: Option<String>);

    /// set_label_visible:
    /// @button: A #Button
    /// @visible: %true if the text should be visible
    ///
    /// Sets the visibility of the text associated with the button's action.
    ///
    fn set_label_visible(&self, visible: bool);

    /// set_toggled:
    /// @button: a #Button
    /// @toggled: #true or #false
    ///
    /// Sets the toggled state of the button. This is only really useful if the
    /// button has #toggle-mode mode set to #true.
    ///
    fn set_toggled(&self, toggled: bool);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Button>> ButtonExt for O {
    /// get_action:
    /// @button: A #Button
    ///
    /// Retrieves the #PushAction associated with @button.
    ///
    /// Returns: (transfer none): A #PushAction
    ///
    fn get_action(&self) -> Option<PushAction> {
        let button = self.as_ref();
        button.props.borrow().action.clone()
    }

    /// get_icon_name:
    /// @button: a #Button
    ///
    /// Get the icon-name being used on the button.
    ///
    /// Returns: the icon-name. This must not be freed by the application. %None if
    ///   no icon has been set
    ///
    fn get_icon_name(&self) -> Option<String> {
        let button = self.as_ref();
        let props = button.props.borrow();

        if props.icon_name.is_some() {
            props.icon_name.clone()
        } else {
            props.style_icon_name.clone()
        }
    }

    /// get_icon_position:
    /// @button: A #Button
    ///
    /// Retrieves the icon's relative position to the text.
    ///
    /// Returns: A #Position
    ///
    fn get_icon_position(&self) -> Position {
        let button = self.as_ref();
        button.props.borrow().icon_position
    }

    /// get_icon_size:
    /// @button: a #Button
    ///
    /// Retrieves the icon-size being used for the displayed icon inside the button.
    ///
    /// Returns: The icon-size being used for the button icon, in pixels
    ///
    fn get_icon_size(&self) -> u32 {
        let button = self.as_ref();
        let props = button.props.borrow();

        if props.icon_size != 0 {
            props.icon_size
        } else {
            props.style_icon_size
        }
    }

    /// get_icon_visible:
    /// @button: A #Button
    ///
    /// Retrieves the visibility of the icon associated with the button's action.
    ///
    /// Returns: %true if the icon is visible, %false otherwise
    ///
    fn get_icon_visible(&self) -> bool {
        let button = self.as_ref();
        button.props.borrow().icon_visible
    }

    /// get_is_toggle:
    /// @button: a #Button
    ///
    /// Get the toggle mode status of the button.
    ///
    /// Returns: #true if toggle mode is set, otherwise #false
    ///
    fn get_is_toggle(&self) -> bool {
        let button = self.as_ref();
        button.props.borrow().is_toggle
    }

    /// get_label:
    /// @button: a #Button
    ///
    /// Get the text displayed on the button
    ///
    /// Returns: the text for the button. This must not be freed by the application
    ///
    fn get_label(&self) -> Option<String> {
        let button = self.as_ref();
        button.props.borrow().text.clone()
    }

    /// get_label_visible:
    /// @button: A #Button
    ///
    /// Retrieves the visibility of the text associated with the button's action.
    ///
    /// Returns: %true if the text is visible, %false otherwise
    ///
    fn get_label_visible(&self) -> bool {
        let button = self.as_ref();
        button.props.borrow().label_visible
    }

    /// get_toggled:
    /// @button: a #Button
    ///
    /// Get the state of the button that is in toggle mode.
    ///
    /// Returns: #true if the button is toggled, or #false if not
    ///
    fn get_toggled(&self) -> bool {
        let button = self.as_ref();
        button.props.borrow().is_toggled
    }

    /// set_action:
    /// @button: A #Button
    /// @action: A #PushAction
    ///
    /// Sets @action as the action for @button. @Button will take its label and
    /// icon from @action.
    ///
    fn set_action<P: Is<PushAction>>(&self, action: &P) {
        let button = self.as_ref();
        let action = action.as_ref();

        let mut display_name: String;

        // if button.action_label_binding {
        //   g_object_unref (button.action_label_binding);
        // }

        // if button.action_icon_binding {
        //   g_object_unref (button.action_icon_binding);
        // }

        // props.action = g_object_ref_sink (action);

        // display_name = action.get_display_name();

        // icon_set_icon_name(ICON (button.icon), action.get_icon());
        // text_set_text(CLUTTER_TEXT (button.label),
        //                        (display_name) ? display_name : "");

        // // bind action properties to button properties
        // button.action_label_binding = g_object_bind_property (action, "display-name",
        //                                                      button.label, "text", 0);

        // button.action_icon_binding = g_object_bind_property (action, "icon",
        //                                                     button.icon, "icon-name",
        //                                                     0);

        // button.update_contents();
    }

    /// set_icon_name:
    /// @button: a #Button
    /// @icon_name: (allow-none): icon-name to use on the button
    ///
    /// Sets the icon-name used to display an icon on the button. Setting %None
    /// will remove the icon name, or resort to the icon-name set in the current
    /// style. Setting an icon name overrides any icon set in the style.
    ///
    fn set_icon_name(&self, icon_name: Option<String>) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        props.icon_name = icon_name;
        // icon_set_icon_name(ICON (button.icon), icon_name ?
        //                        icon_name : button.style_icon_name);
        // button.update_contents ();
        // g_object_notify (G_OBJECT (button), "icon-name");
    }

    /// set_icon_position:
    /// @button: A #Button
    /// @position: A #Position
    ///
    /// Sets the icon position, relative to the text on the button.
    ///
    fn set_icon_position(&self, position: Position) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        if props.icon_position != position {
            props.icon_position = position;
            // button.update_contents();
            // g_object_notify (G_OBJECT (button), "icon-position");
        }
    }

    /// set_icon_size:
    /// @button: a #Button
    ///
    /// Sets the icon-size to use for the icon displayed inside the button. This will
    /// override the icon-size set in the style. Setting a value of %0 resets to the
    /// size from the style.
    ///
    fn set_icon_size(&self, icon_size: u32) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        if props.icon_size != icon_size {
            props.icon_size = icon_size;
            // icon_set_icon_size (ICON (button.icon), icon_size ?
            //                         icon_size : button.style_icon_size);
            // g_object_notify (G_OBJECT (button), "icon-size");
        }
    }

    /// set_icon_visible:
    /// @button: A #Button
    /// @visible: %true if the icon should be visible
    ///
    /// Sets the visibility of the icon associated with the button's action.
    ///
    fn set_icon_visible(&self, visible: bool) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        if props.icon_visible != visible {
            props.icon_visible = visible;
            // button.update_contents();
            // g_object_notify (G_OBJECT (button), "icon-visible");
        }
    }

    /// set_is_toggle:
    /// @button: a #Button
    /// @toggle: #true or #false
    ///
    /// Enables or disables toggle mode for the button. In toggle mode, the active
    /// state will be "toggled" when the user clicks the button.
    ///
    fn set_is_toggle(&self, toggle: bool) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();
        props.is_toggle = toggle;
        // g_object_notify (G_OBJECT (button), "is-toggle");
    }

    /// set_label:
    /// @button: a #Button
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the button
    ///
    fn set_label(&self, text: Option<String>) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        props.text = text;

        // text_set_text(CLUTTER_TEXT(button.label), button.text);
        // button.update_contents();
        // g_object_notify(G_OBJECT(button), "label");
    }

    /// set_label_visible:
    /// @button: A #Button
    /// @visible: %true if the text should be visible
    ///
    /// Sets the visibility of the text associated with the button's action.
    ///
    fn set_label_visible(&self, visible: bool) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        if props.label_visible != visible {
            props.label_visible = visible;
            // button.update_contents(button);
            // g_object_notify(G_OBJECT(button), "label-visible");
        }
    }

    /// set_toggled:
    /// @button: a #Button
    /// @toggled: #true or #false
    ///
    /// Sets the toggled state of the button. This is only really useful if the
    /// button has #toggle-mode mode set to #true.
    ///
    fn set_toggled(&self, toggled: bool) {
        let button = self.as_ref();
        let mut props = button.props.borrow_mut();

        if props.is_toggled != toggled {
            props.is_toggled = toggled;

            if toggled {
                // stylable_style_pseudo_class_add(STYLABLE(button), "checked");
            } else {
                // stylable_style_pseudo_class_remove(STYLABLE(button), "checked");
            }
            // g_object_notify(G_OBJECT(button), "toggled");
        }
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::action\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_action_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_position_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-position\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_position_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-size\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_size_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_visible_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-visible\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_visible_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_is_toggle_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::is-toggle\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_is_toggle_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::label\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_label_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_label_visible_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::label-visible\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_label_visible_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_toggled_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Button,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Button>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Button::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::toggled\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_toggled_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button")
    }
}
