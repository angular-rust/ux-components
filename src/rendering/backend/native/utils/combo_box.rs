#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::{
    platform::core::Texture, Actor, HandlerId, Icon, IconTheme, Label, PushAction, Widget,
};
use crate::{prelude::*, PushActionExt};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct ComboBoxProps {
    pub label: Option<Label>, //Actor,
    pub icon: Option<Icon>,   // Actor

    pub marker: Option<Texture>,
    pub actions: Vec<PushAction>,

    pub clip_x: f32,
    pub clip_y: f32,
    pub index: i32,
    pub spacing: i32,
}

#[derive(Debug)]
pub struct ComboBox {
    props: RefCell<ComboBoxProps>,
    widget: Widget,
}

impl ComboBox {
    pub fn new() -> ComboBox {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::combo_box_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ComboBox {}
impl Is<ComboBox> for ComboBox {}

impl AsRef<ComboBox> for ComboBox {
    fn as_ref(&self) -> &ComboBox {
        self
    }
}

impl Is<Widget> for ComboBox {}

impl AsRef<Widget> for ComboBox {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for ComboBox {}

impl AsRef<Actor> for ComboBox {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait ComboBoxExt: 'static {
    /// append_text:
    /// @box: A #ComboBox
    /// @text: name of the item
    ///
    /// Append an item to the combo box list
    ///
    fn append_text(&self, text: &str);

    /// get_active_icon_name:
    /// @box: A #ComboBox
    ///
    /// Get the name of the icon displayed in the combo box
    ///
    /// Returns: the text string of the name of the displayed icon, owned by
    ///          the combo box, or %None if there is no active icon.
    ///
    fn get_active_icon_name(&self) -> Option<String>;

    /// get_active_text:
    /// @box: A #ComboBox
    ///
    /// Get the text displayed in the combo box
    ///
    /// Returns: the text string, owned by the combo box
    ///
    fn get_active_text(&self) -> Option<String>;

    /// get_index:
    /// @box: A #ComboBox
    ///
    /// Get the index of the last item selected
    ///
    /// Returns: gint
    ///
    fn get_index(&self) -> i32;

    /// insert_text:
    /// @box: A #ComboBox
    /// @position: zero indexed position to insert the item at
    /// @text: name of the item
    ///
    /// Insert an item into the combo box list.
    ///
    fn insert_text(&self, position: i32, text: &str);

    /// insert_text_with_icon:
    /// @box: A #ComboBox
    /// @position: zero indexed position to insert the item at
    /// @text: name of the item
    /// @icon: name of an icon from the icon theme
    ///
    /// Insert an item with text and an icon into the combo box list.
    ///
    fn insert_text_with_icon(&self, position: i32, text: &str, icon: &str);

    /// prepend_text:
    /// @box: A #ComboBox
    /// @text: name of the item
    ///
    /// Prepend an item to the combo box list
    ///
    fn prepend_text(&self, text: &str);

    /// remove_all:
    /// @box: A #ComboBox
    ///
    /// Remove all the items of @box
    ///
    fn remove_all(&self);

    /// remove_text:
    /// @box: A #ComboBox
    /// @position: position of the item to remove
    ///
    /// Remove the item at @position
    ///
    fn remove_text(&self, position: usize);

    /// set_active_icon_name:
    /// @box: A #ComboBox
    /// @icon_name: (allow-none): Icon name to use for displayed icon
    ///
    /// Set the icon displayed in the combo box.
    ///
    fn set_active_icon_name(&self, icon_name: Option<String>);

    /// set_active_text:
    /// @box: A #ComboBox
    /// @text: text to display
    ///
    /// Set the text displayed in the combo box
    ///
    fn set_active_text(&self, text: &str);

    /// set_index:
    /// @box: A #ComboBox
    /// @index: the index of the list item to set
    ///
    /// Set the current combo box text from the item at @index in the list.
    ///
    fn set_index(&self, index: i32);

    fn connect_property_active_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_active_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<ComboBox>> ComboBoxExt for O {
    /// append_text:
    /// @box: A #ComboBox
    /// @text: name of the item
    ///
    /// Append an item to the combo box list
    ///
    fn append_text(&self, text: &str) {
        self.insert_text(-1, text);
    }

    /// get_active_icon_name:
    /// @box: A #ComboBox
    ///
    /// Get the name of the icon displayed in the combo box
    ///
    /// Returns: the text string of the name of the displayed icon, owned by
    ///          the combo box, or %None if there is no active icon.
    ///
    fn get_active_icon_name(&self) -> Option<String> {
        let combobox = self.as_ref();
        let props = combobox.props.borrow();

        // match &props.icon {
        //     Some(icon) => icon.get_icon_name(),
        //     None => None,
        // }
        unimplemented!()
    }

    /// get_active_text:
    /// @box: A #ComboBox
    ///
    /// Get the text displayed in the combo box
    ///
    /// Returns: the text string, owned by the combo box
    ///
    fn get_active_text(&self) -> Option<String> {
        let combobox = self.as_ref();
        let props = combobox.props.borrow();

        // match &props.label {
        //     Some(label) => label.get_text(),
        //     None => None,
        // }
        unimplemented!()
    }

    /// get_index:
    /// @box: A #ComboBox
    ///
    /// Get the index of the last item selected
    ///
    /// Returns: gint
    ///
    fn get_index(&self) -> i32 {
        let combobox = self.as_ref();
        let props = combobox.props.borrow();
        props.index
    }

    /// insert_text:
    /// @box: A #ComboBox
    /// @position: zero indexed position to insert the item at
    /// @text: name of the item
    ///
    /// Insert an item into the combo box list.
    ///
    fn insert_text(&self, position: i32, text: &str) {
        let combobox = self.as_ref();

        let action = PushAction::new();
        action.set_display_name(text);

        // combobox.actions.push(action);
        // combobox.update_menu();
    }

    /// insert_text_with_icon:
    /// @box: A #ComboBox
    /// @position: zero indexed position to insert the item at
    /// @text: name of the item
    /// @icon: name of an icon from the icon theme
    ///
    /// Insert an item with text and an icon into the combo box list.
    ///
    fn insert_text_with_icon(&self, position: i32, text: &str, icon: &str) {
        let combobox = self.as_ref();
        let combobox = self.as_ref();

        let action = PushAction::new();
        action.set_display_name(text);
        action.set_icon(icon);

        // combobox.actions.push(action);
        // combobox.update_menu();
    }

    /// prepend_text:
    /// @box: A #ComboBox
    /// @text: name of the item
    ///
    /// Prepend an item to the combo box list
    ///
    fn prepend_text(&self, text: &str) {
        self.insert_text(0, text);
    }

    /// remove_all:
    /// @box: A #ComboBox
    ///
    /// Remove all the items of @box
    ///
    fn remove_all(&self) {
        let combobox = self.as_ref();

        // combobox.actions.clear();
        // combobox.update_menu();
    }

    /// remove_text:
    /// @box: A #ComboBox
    /// @position: position of the item to remove
    ///
    /// Remove the item at @position
    ///
    fn remove_text(&self, position: usize) {
        let combobox = self.as_ref();

        // find the item, free the string and remove it from the list
        // combobox.actions.remove(position);
        // combobox.update_menu();
    }

    /// set_active_icon_name:
    /// @box: A #ComboBox
    /// @icon_name: (allow-none): Icon name to use for displayed icon
    ///
    /// Set the icon displayed in the combo box.
    ///
    fn set_active_icon_name(&self, icon_name: Option<String>) {
        let combobox = self.as_ref();
        let mut props = combobox.props.borrow_mut();

        // match &props.icon {
        //     None => {
        //         if let Some(icon_name) = &icon_name {
        //             let icon_theme = IconTheme::get_default().unwrap();
        //             if icon_theme.has_icon(icon_name) {
        //                 let icon = Icon::new();
        //                 icon.set_icon_name(Some(icon_name.clone()));
        //                 // actor_add_child (CLUTTER_ACTOR (box), combobox.icon);
        //                 props.icon = Some(icon);
        //             }
        //         }
        //     }
        //     Some(icon) => {
        //         if let Some(icon_name) = icon_name {
        //             icon.set_icon_name(Some(icon_name));
        //         } else {
        //             // actor_destroy (priv->icon);
        //             props.icon = None;
        //             // actor_queue_relayout (CLUTTER_ACTOR (box));
        //         }
        //     }
        // }

        props.index = -1;
        // g_object_notify (G_OBJECT (box), "index");
        // g_object_notify (G_OBJECT (box), "active-icon-name");
    }

    /// set_active_text:
    /// @box: A #ComboBox
    /// @text: text to display
    ///
    /// Set the text displayed in the combo box
    ///
    fn set_active_text(&self, text: &str) {
        let combobox = self.as_ref();
        let mut props = combobox.props.borrow_mut();

        props.index = -1;
        // text_set_text((ClutterText*)combobox.label, text);

        // g_object_notify(G_OBJECT (box), "index");
        // g_object_notify(G_OBJECT (box), "active-text");
    }

    /// set_index:
    /// @box: A #ComboBox
    /// @index: the index of the list item to set
    ///
    /// Set the current combo box text from the item at @index in the list.
    ///
    fn set_index(&self, index: i32) {
        let combobox = self.as_ref();
        let mut props = combobox.props.borrow_mut();

        // GSList *item;
        // PushAction *action;
        // const gchar *icon_name;

        // let item = g_slist_nth(combobox.actions, index);

        // if !item {
        //     combobox.index = -1;
        //     text_set_text((ClutterText*)combobox.label, "");
        //     return;
        // }

        props.index = index;
        // action = (PushAction *)item.data;
        // text_set_text((ClutterText*) combobox.label,
        //                         action_get_display_name(action));

        if props.icon.is_some() {
            //     actor_destroy(combobox.icon);
            props.icon = None;
        }

        // icon_name = action_get_icon(item.data);
        // if icon_name {
        //     let icon_theme = IconTheme::get_default().unwrap();
        //     if icon_theme.has_icon(icon_name) {
        //         props.icon = icon_new ();
        //         icon_set_icon_name(ICON (combobox.icon), icon_name);
        //         actor_add_child(CLUTTER_ACTOR (box), combobox.icon);
        //     }
        // }

        // g_object_notify(G_OBJECT (box), "index");
        // g_object_notify(G_OBJECT (box), "active-text");
        // g_object_notify(G_OBJECT (box), "active-icon-name");
    }

    fn connect_property_active_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_active_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_active_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_active_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::active-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_active_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_index_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ComboBox,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ComboBox>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::index\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_index_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ComboBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComboBox")
    }
}
