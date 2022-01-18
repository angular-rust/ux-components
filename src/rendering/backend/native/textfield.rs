#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Tooltip, Widget};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct TextfieldProps {
    pub entry: Option<Actor>,
    pub placeholder: Option<String>,
    pub primary_icon: Option<Actor>,
    pub primary_icon_highlight: Option<Actor>,
    pub primary_icon_tooltip: Tooltip,
    pub secondary_icon: Option<Actor>,
    pub secondary_icon_highlight: Option<Actor>,
    pub secondary_icon_tooltip: Tooltip,
    pub primary_icon_filename: String,
    pub secondary_icon_filename: String,
    pub icon_highlight_suffix: Option<String>,
    pub spacing: f64,
    pub password_char: char,
    pub undo_history: Vec<String>,
    pub undo_timeout_source: u64,
    pub pause_undo: bool,
    pub scrolling: bool,
    pub unicode_input_mode: bool,
    pub pointer_in_entry: bool,
    pub preedit_string: String,
    pub tooltip_timeout: u32,
}

#[derive(Debug)]
pub struct Textfield {
    props: RefCell<TextfieldProps>,
    widget: Widget,
}

impl Textfield {
    pub fn new() -> Textfield {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::entry_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_text(text: &str) -> Textfield {
        // assert_initialized_main_thread!();
        // unsafe {
        //     Actor::from_glib_none(ffi::entry_new_with_text(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for Textfield {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Textfield {}
impl Is<Textfield> for Textfield {}

impl AsRef<Textfield> for Textfield {
    fn as_ref(&self) -> &Textfield {
        self
    }
}

impl Is<Widget> for Textfield {}

impl AsRef<Widget> for Textfield {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for Textfield {}

impl AsRef<Actor> for Textfield {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait TextfieldExt: 'static {
    // /// get_clutter_text:
    // /// @entry: a #Textfield
    // ///
    // /// Retrieve the internal #Text so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #Text used by #Textfield. The entry is
    // /// owned by the #Textfield and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> &Option<Actor>;

    /// get_icon_highlight_suffix:
    /// @entry: a #Textfield
    ///
    /// Get the suffix appended to the filename to use for the highlighted version
    /// of the icon.
    ///
    /// Returns: the highlight filename suffix. This string is owned by the
    /// #Textfield and should not be freed or modified.
    ///
    fn get_icon_highlight_suffix(&self) -> Option<String>;

    /// get_password_char:
    /// @entry: a #Textfield
    ///
    /// Gets the character to display instead of the text.
    ///
    /// Return value: a character, or 0 if input should not be hidden.
    ///
    fn get_password_char(&self) -> char;

    /// get_placeholder:
    /// @entry: a #Textfield
    ///
    /// Gets the text that is displayed when the entry is empty and unfocused
    ///
    /// Returns: (transfer none): the current value of the placeholder property.
    /// This string is owned by the #Textfield and should not be freed or modified.
    ///
    fn get_placeholder(&self) -> Option<String>;

    /// get_text:
    /// @entry: a #Textfield
    ///
    /// Get the text displayed on the entry
    ///
    /// Returns: the text for the entry. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String>;

    /// set_icon_highlight_suffix:
    /// @entry: a #Textfield
    /// @suffix: the suffix to append to the filename for the highlight version
    ///
    /// Sets the suffix appended to the filename to use for the highlighted version
    /// of the icon. e.g. if you have set your primay icon to "primary-icon.png"
    /// and the suffix to "-highlight" #Textfield will look for "primary-icon-highlight.png"
    ///
    fn set_icon_highlight_suffix(&self, suffix: &str);

    /// set_password_char:
    /// @entry: a #Textfield
    /// @password_char: character to display instead of text
    ///
    /// Sets the character to display instead of the text. Use 0 to display
    /// the actual text.
    ///
    fn set_password_char(&self, password_char: char);

    /// set_placeholder:
    /// @entry: a #Textfield
    /// @text: text to set as the entry hint
    ///
    /// Sets the text to display when the entry is empty and unfocused. When the
    /// entry is displaying the hint, it has a pseudo class of "indeterminate".
    /// A value of None unsets the hint.
    ///
    fn set_placeholder(&self, text: &str);

    /// set_primary_icon_from_file:
    /// @entry: a #Textfield
    /// @filename: filename of an icon
    ///
    /// Set the primary icon of the entry to the given filename
    ///
    fn set_primary_icon_from_file(&self, filename: &str);

    /// set_primary_icon_tooltip:
    /// @entry: a #Textfield
    /// @text: the primary icon tooltip
    ///
    /// Set the primary icon tooltip text
    ///
    fn set_primary_icon_tooltip_text(&self, text: &str);

    /// set_secondary_icon_from_file:
    /// @entry: a #Textfield
    /// @filename: filename of an icon
    ///
    /// Set the secondary icon of the entry to the given filename
    ///
    fn set_secondary_icon_from_file(&self, filename: &str);

    /// set_secondary_icon_tooltip:
    /// @entry: a #Textfield
    /// @text: the secondary icon tooltip
    ///
    /// Set the secondary icon tooltip text
    ///
    fn set_secondary_icon_tooltip_text(&self, text: &str);

    /// set_text:
    /// @entry: a #Textfield
    /// @text: text to set the entry to
    ///
    /// Sets the text displayed on the entry
    ///
    fn set_text(&self, text: &str);

    /// get_primary_icon_tooltip_text:
    /// @entry: a #Textfield
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_primary_icon_tooltip_text(&self) -> Option<String>;

    /// get_secondary_icon_tooltip_text:
    /// @entry: a #Textfield
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_secondary_icon_tooltip_text(&self) -> Option<String>;

    fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Textfield>> TextfieldExt for O {
    // /// get_clutter_text:
    // /// @entry: a #Textfield
    // ///
    // /// Retrieve the internal #Text so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #Text used by #Textfield. The entry is
    // /// owned by the #Textfield and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> &Option<Actor> {
    //     let entry = self.as_ref();
    //     let props = entry.props.borrow();

    //     &props.entry
    // }

    /// get_icon_highlight_suffix:
    /// @entry: a #Textfield
    ///
    /// Get the suffix appended to the filename to use for the highlighted version
    /// of the icon.
    ///
    /// Returns: the highlight filename suffix. This string is owned by the
    /// #Textfield and should not be freed or modified.
    ///
    fn get_icon_highlight_suffix(&self) -> Option<String> {
        let entry = self.as_ref();
        let props = entry.props.borrow();

        props.icon_highlight_suffix.clone()
    }

    /// get_password_char:
    /// @entry: a #Textfield
    ///
    /// Gets the character to display instead of the text.
    ///
    /// Return value: a character, or 0 if input should not be hidden.
    ///
    fn get_password_char(&self) -> char {
        let entry = self.as_ref();
        let props = entry.props.borrow();

        props.password_char
    }

    /// get_placeholder:
    /// @entry: a #Textfield
    ///
    /// Gets the text that is displayed when the entry is empty and unfocused
    ///
    /// Returns: (transfer none): the current value of the placeholder property.
    /// This string is owned by the #Textfield and should not be freed or modified.
    ///
    fn get_placeholder(&self) -> Option<String> {
        let entry = self.as_ref();
        entry.props.borrow().placeholder.clone()
    }

    /// get_text:
    /// @entry: a #Textfield
    ///
    /// Get the text displayed on the entry
    ///
    /// Returns: the text for the entry. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String> {
        let entry = self.as_ref();
        // text_get_text (CLUTTER_TEXT (entry.entry));
        unimplemented!()
    }

    /// set_icon_highlight_suffix:
    /// @entry: a #Textfield
    /// @suffix: the suffix to append to the filename for the highlight version
    ///
    /// Sets the suffix appended to the filename to use for the highlighted version
    /// of the icon. e.g. if you have set your primay icon to "primary-icon.png"
    /// and the suffix to "-highlight" #Textfield will look for "primary-icon-highlight.png"
    ///
    fn set_icon_highlight_suffix(&self, suffix: &str) {
        let entry = self.as_ref();
        // if (g_strcmp0 (entry.icon_highlight_suffix, suffix) == 0) {
        //     return;
        // }

        // entry.icon_highlight_suffix = g_strdup (suffix);

        // _entry_create_highlight_icon (entry, 1);
        // _entry_create_highlight_icon (entry, 2);
    }

    /// set_password_char:
    /// @entry: a #Textfield
    /// @password_char: character to display instead of text
    ///
    /// Sets the character to display instead of the text. Use 0 to display
    /// the actual text.
    ///
    fn set_password_char(&self, password_char: char) {
        let entry = self.as_ref();
        // unsafe {
        //     ffi::entry_set_password_char(
        //         self.as_ref().to_glib_none().0,
        //         password_char.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// set_placeholder:
    /// @entry: a #Textfield
    /// @text: text to set as the entry hint
    ///
    /// Sets the text to display when the entry is empty and unfocused. When the
    /// entry is displaying the hint, it has a pseudo class of "indeterminate".
    /// A value of None unsets the hint.
    ///
    fn set_placeholder(&self, text: &str) {
        let entry = self.as_ref();

        // if !g_strcmp0(entry.placeholder, text) {
        //     return;
        // }

        // entry.placeholder = g_strdup(text);

        // if entry.placeholder {
        //     if !strcmp(text_get_text(CLUTTER_TEXT(entry.entry)), "") {
        //         stylable_style_pseudo_class_add(STYLABLE(entry), "indeterminate");
        //     } else {
        //         stylable_style_pseudo_class_remove(STYLABLE(entry), "indeterminate");
        //     }
        // }

        // actor_queue_redraw(CLUTTER_ACTOR(entry));
    }

    /// set_primary_icon_from_file:
    /// @entry: a #Textfield
    /// @filename: filename of an icon
    ///
    /// Set the primary icon of the entry to the given filename
    ///
    fn set_primary_icon_from_file(&self, filename: &str) {
        let entry = self.as_ref();

        // entry.primary_icon_filename = g_strdup (filename);

        // _entry_set_icon_from_file (entry, &entry.primary_icon, filename);
        // _entry_create_highlight_icon (entry, 1);
    }

    /// set_primary_icon_tooltip:
    /// @entry: a #Textfield
    /// @text: the primary icon tooltip
    ///
    /// Set the primary icon tooltip text
    ///
    fn set_primary_icon_tooltip_text(&self, text: &str) {
        let entry = self.as_ref();

        // if !entry.primary_icon_tooltip {
        //     entry.primary_icon_tooltip = g_object_new(TYPE_TOOLTIP, "text", text, None);

        //     tooltip_set_text(entry.primary_icon_tooltip, text);
        //     actor_add_child(
        //         ACTOR(entry),
        //         ACTOR(entry.primary_icon_tooltip),
        //     );
        // } else {
        //     tooltip_set_text(entry.primary_icon_tooltip, text);
        // }
    }

    /// set_secondary_icon_from_file:
    /// @entry: a #Textfield
    /// @filename: filename of an icon
    ///
    /// Set the secondary icon of the entry to the given filename
    ///
    fn set_secondary_icon_from_file(&self, filename: &str) {
        let entry = self.as_ref();

        // entry.secondary_icon_filename = g_strdup(filename);

        // _entry_set_icon_from_file(entry, &entry.secondary_icon, filename);
        // _entry_create_highlight_icon(entry, 2);
    }

    /// set_secondary_icon_tooltip:
    /// @entry: a #Textfield
    /// @text: the secondary icon tooltip
    ///
    /// Set the secondary icon tooltip text
    ///
    fn set_secondary_icon_tooltip_text(&self, text: &str) {
        let entry = self.as_ref();

        // if !entry.secondary_icon_tooltip {
        //     entry.secondary_icon_tooltip = g_object_new(TYPE_TOOLTIP, "text", text, None);

        //     tooltip_set_text(entry.secondary_icon_tooltip, text);
        //     actor_add_child(
        //         ACTOR(entry),
        //         ACTOR(entry.secondary_icon_tooltip),
        //     );
        // } else {
        //     tooltip_set_text(entry.secondary_icon_tooltip, text);
        // }
    }

    /// set_text:
    /// @entry: a #Textfield
    /// @text: text to set the entry to
    ///
    /// Sets the text displayed on the entry
    ///
    fn set_text(&self, text: &str) {
        let entry = self.as_ref();

        // let text = if entry_text {
        //     entry_text
        // } else {
        //     text = "";
        // };

        // text_set_text (CLUTTER_TEXT (entry.entry), text);
    }

    /// get_primary_icon_tooltip_text:
    /// @entry: a #Textfield
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_primary_icon_tooltip_text(&self) -> Option<String> {
        let entry = self.as_ref();
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"primary-icon-tooltip-text\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `primary-icon-tooltip-text` getter")
        // }
        unimplemented!()
    }

    /// get_secondary_icon_tooltip_text:
    /// @entry: a #Textfield
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_secondary_icon_tooltip_text(&self) -> Option<String> {
        let entry = self.as_ref();
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"secondary-icon-tooltip-text\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `secondary-icon-tooltip-text` getter")
        // }
        unimplemented!()
    }

    fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn primary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"primary-icon-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             primary_icon_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn secondary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"secondary-icon-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             secondary_icon_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_clutter_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clutter-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clutter_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_icon_highlight_suffix_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-highlight-suffix\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_highlight_suffix_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_password_char_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::password-char\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_password_char_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_placeholder_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::placeholder\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_placeholder_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_primary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::primary-icon-tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_primary_icon_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_secondary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::secondary-icon-tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_secondary_icon_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Textfield,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Textfield>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Textfield::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Textfield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Textfield")
    }
}
