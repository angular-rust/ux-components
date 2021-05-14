#![allow(unused_variables)]

use crate::{prelude::*, Icon, StyleClass, Theme};
use crate::{Actor, HandlerId, Position, PushAction, Text, Widget};
use std::{cell::RefCell, fmt};

#[derive(Default, Debug)]
pub struct CheckboxProps {
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
    pub content_image: Option<dx::Texture>,
    pub hbox: Option<Actor>,
    pub icon: Option<Actor>,
    pub label: Option<Actor>,
    // pub animation: Animation,
    // pub action_label_binding: GBinding,
    // pub action_icon_binding: GBinding,
}

#[derive(Debug)]
pub struct Checkbox {
    props: RefCell<CheckboxProps>,
    inner: Widget,
}

impl Checkbox {
    pub fn new() -> Self {
        let props = CheckboxProps::default();

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    pub fn with_label(text: &str) -> Self {
        let mut props = CheckboxProps::default();
        if !text.is_empty() {
            props.text = Some(text.into())
        }

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    pub fn with_icon(name: &str) -> Self {
        let mut props = CheckboxProps::default();
        if !name.is_empty() {
            props.icon_name = Some(name.into())
        }

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    fn init(&self) {
        println!("INIT CHECKBOX");

        self.inner.set_reactive(true);

        // g_signal_connect (button, "style-changed",
        //             G_CALLBACK (button_style_changed), NULL);
        // g_signal_connect (button, "actor-added",
        //                     G_CALLBACK (button_actor_added), NULL);
        // g_signal_connect (button, "actor-removed",
        //                     G_CALLBACK (button_actor_removed), NULL);

        let mut props = self.props.borrow_mut();
        props.icon_visible = true;
        props.label_visible = true;
        props.icon_position = Position::Left;

        // take an extra reference to the hbox
        // priv->hbox = g_object_ref (box_layout_new ());
        // clutter_actor_add_child (CLUTTER_ACTOR (button), priv->hbox);

        if let Some(name) = &props.icon_name {
            println!("ADD ICON TO CHECKBOX [{}]", name);
            let icon = Icon::new();
            // priv->icon = icon_new ();
            // clutter_actor_add_child (priv->hbox, priv->icon);

            self.inner.add_child(&icon);
        }

        if let Some(text) = &props.text {
            println!("ADD TEXT TO CHECKBOX [{}]", text);
            let style = Theme::global().get(StyleClass::MdcButton).unwrap();
            let fontfamily = if let Some(fontfamily) = style.fontfamily {
                fontfamily
            } else {
                "Roboto".into()
            };

            let label = Text::with_text(Some(fontfamily.as_str()), text.as_str());
            // priv->label = g_object_new (CLUTTER_TYPE_TEXT,
            //                             "line-alignment", PANGO_ALIGN_CENTER,
            //                             "ellipsize", PANGO_ELLIPSIZE_END,
            //                             NULL);
            // clutter_actor_add_child (priv->hbox, priv->label);
            // let actor: &Actor = label.as_ref();
            // self.inner.add_child(&label);
        }

        // box_layout_child_set_expand (BOX_LAYOUT (priv->hbox),
        //                                 priv->label, TRUE);
        // box_layout_child_set_y_fill (BOX_LAYOUT (priv->hbox),
        //                                 priv->label, FALSE);
        // box_layout_child_set_x_fill (BOX_LAYOUT (priv->hbox),
        //                                 priv->label, FALSE);

        // box_layout_child_set_expand (BOX_LAYOUT (priv->hbox),
        //                                 priv->icon, TRUE);
        // box_layout_child_set_y_fill (BOX_LAYOUT (priv->hbox),
        //                                 priv->icon, FALSE);
        // box_layout_child_set_x_fill (BOX_LAYOUT (priv->hbox),
        //                                 priv->icon, FALSE);

        // button_update_contents (button);
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Checkbox {}
impl Is<Checkbox> for Checkbox {}

impl AsRef<Checkbox> for Checkbox {
    fn as_ref(&self) -> &Checkbox {
        self
    }
}

impl Is<Widget> for Checkbox {}

impl AsRef<Widget> for Checkbox {
    fn as_ref(&self) -> &Widget {
        &self.inner
    }
}

impl Is<Actor> for Checkbox {}

impl AsRef<Actor> for Checkbox {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.inner.as_ref();
        actor
    }
}

pub trait CheckboxExt: 'static {
    /// get_action:
    /// @button: A #Checkbox
    ///
    /// Retrieves the #PushAction associated with @button.
    ///
    /// Returns: (transfer none): A #PushAction
    ///
    fn get_action(&self) -> Option<PushAction>;

    /// get_icon_name:
    /// @button: a #Checkbox
    ///
    /// Get the icon-name being used on the button.
    ///
    /// Returns: the icon-name. This must not be freed by the application. %None if
    ///   no icon has been set
    ///
    fn get_icon_name(&self) -> Option<String>;

    /// get_icon_position:
    /// @button: A #Checkbox
    ///
    /// Retrieves the icon's relative position to the text.
    ///
    /// Returns: A #Position
    ///
    fn get_icon_position(&self) -> Position;

    /// get_icon_size:
    /// @button: a #Checkbox
    ///
    /// Retrieves the icon-size being used for the displayed icon inside the button.
    ///
    /// Returns: The icon-size being used for the button icon, in pixels
    ///
    fn get_icon_size(&self) -> u32;

    /// get_icon_visible:
    /// @button: A #Checkbox
    ///
    /// Retrieves the visibility of the icon associated with the button's action.
    ///
    /// Returns: %true if the icon is visible, %false otherwise
    ///
    fn get_icon_visible(&self) -> bool;

    /// get_is_toggle:
    /// @button: a #Checkbox
    ///
    /// Get the toggle mode status of the button.
    ///
    /// Returns: #true if toggle mode is set, otherwise #false
    ///
    fn get_is_toggle(&self) -> bool;

    /// get_label:
    /// @button: a #Checkbox
    ///
    /// Get the text displayed on the button
    ///
    /// Returns: the text for the button. This must not be freed by the application
    ///
    fn get_label(&self) -> Option<String>;

    /// get_label_visible:
    /// @button: A #Checkbox
    ///
    /// Retrieves the visibility of the text associated with the button's action.
    ///
    /// Returns: %true if the text is visible, %false otherwise
    ///
    fn get_label_visible(&self) -> bool;

    /// get_toggled:
    /// @button: a #Checkbox
    ///
    /// Get the state of the button that is in toggle mode.
    ///
    /// Returns: #true if the button is toggled, or #false if not
    ///
    fn get_toggled(&self) -> bool;

    /// set_action:
    /// @button: A #Checkbox
    /// @action: A #PushAction
    ///
    /// Sets @action as the action for @button. @Checkbox will take its label and
    /// icon from @action.
    ///
    fn set_action<P: Is<PushAction>>(&self, action: &P);

    /// set_icon_name:
    /// @button: a #Checkbox
    /// @icon_name: (allow-none): icon-name to use on the button
    ///
    /// Sets the icon-name used to display an icon on the button. Setting %None
    /// will remove the icon name, or resort to the icon-name set in the current
    /// style. Setting an icon name overrides any icon set in the style.
    ///
    fn set_icon_name(&self, icon_name: Option<String>);

    /// set_icon_position:
    /// @button: A #Checkbox
    /// @position: A #Position
    ///
    /// Sets the icon position, relative to the text on the button.
    ///
    fn set_icon_position(&self, position: Position);

    /// set_icon_size:
    /// @button: a #Checkbox
    ///
    /// Sets the icon-size to use for the icon displayed inside the button. This will
    /// override the icon-size set in the style. Setting a value of %0 resets to the
    /// size from the style.
    ///
    fn set_icon_size(&self, icon_size: u32);

    /// set_icon_visible:
    /// @button: A #Checkbox
    /// @visible: %true if the icon should be visible
    ///
    /// Sets the visibility of the icon associated with the button's action.
    ///
    fn set_icon_visible(&self, visible: bool);

    /// set_is_toggle:
    /// @button: a #Checkbox
    /// @toggle: #true or #false
    ///
    /// Enables or disables toggle mode for the button. In toggle mode, the active
    /// state will be "toggled" when the user clicks the button.
    ///
    fn set_is_toggle(&self, toggle: bool);

    /// set_label:
    /// @button: a #Checkbox
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the button
    ///
    fn set_label(&self, text: Option<String>);

    /// set_label_visible:
    /// @button: A #Checkbox
    /// @visible: %true if the text should be visible
    ///
    /// Sets the visibility of the text associated with the button's action.
    ///
    fn set_label_visible(&self, visible: bool);

    /// set_toggled:
    /// @button: a #Checkbox
    /// @toggled: #true or #false
    ///
    /// Sets the toggled state of the button. This is only really useful if the
    /// button has #toggle-mode mode set to #true.
    ///
    fn set_toggled(&self, toggled: bool);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Checkbox>> CheckboxExt for O {
    /// get_action:
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
    /// @action: A #PushAction
    ///
    /// Sets @action as the action for @button. @Checkbox will take its label and
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: a #Checkbox
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
    /// @button: A #Checkbox
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
    /// @button: a #Checkbox
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

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::action\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_action_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_position_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-position\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_position_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-size\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_size_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_visible_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-visible\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_visible_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_is_toggle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_is_toggle_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::is-toggle\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_is_toggle_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::label\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_label_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_label_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_label_visible_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::label-visible\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_label_visible_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_toggled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_toggled_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Checkbox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Checkbox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Checkbox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::toggled\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_toggled_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Checkbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Checkbox")
    }
}
