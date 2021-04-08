#![allow(unused_variables)]

// use std::mem::transmute;
use super::{Tooltip, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct EntryProps {
    pub entry: Option<clutter::Actor>,
    pub placeholder: Option<String>,
    pub primary_icon: Option<clutter::Actor>,
    pub primary_icon_highlight: Option<clutter::Actor>,
    pub primary_icon_tooltip: Tooltip,
    pub secondary_icon: Option<clutter::Actor>,
    pub secondary_icon_highlight: Option<clutter::Actor>,
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

#[derive(Clone, Debug)]
pub struct Entry {
    props: RefCell<EntryProps>,
    widget: Widget,
}

impl Entry {
    pub fn new() -> Entry {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::entry_new()).unsafe_cast() }
        unimplemented!()
    }

    pub fn with_text(text: &str) -> Entry {
        // assert_initialized_main_thread!();
        // unsafe {
        //     clutter::Actor::from_glib_none(ffi::entry_new_with_text(text.to_glib_none().0))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Entry {}
impl Is<Entry> for Entry {}

impl AsRef<Entry> for Entry {
    fn as_ref(&self) -> &Entry {
        self
    }
}

impl Is<Widget> for Entry {}

impl AsRef<Widget> for Entry {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Entry {}

impl AsRef<clutter::Actor> for Entry {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_ENTRY: Option<&Entry> = None;

pub trait EntryExt: 'static {
    // /// get_clutter_text:
    // /// @entry: a #Entry
    // ///
    // /// Retrieve the internal #ClutterText so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #ClutterText used by #Entry. The entry is
    // /// owned by the #Entry and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> &Option<clutter::Actor>;

    /// get_icon_highlight_suffix:
    /// @entry: a #Entry
    ///
    /// Get the suffix appended to the filename to use for the highlighted version
    /// of the icon.
    ///
    /// Returns: the highlight filename suffix. This string is owned by the
    /// #Entry and should not be freed or modified.
    ///
    fn get_icon_highlight_suffix(&self) -> Option<String>;

    /// get_password_char:
    /// @entry: a #Entry
    ///
    /// Gets the character to display instead of the text.
    ///
    /// Return value: a character, or 0 if input should not be hidden.
    ///
    fn get_password_char(&self) -> char;

    /// get_placeholder:
    /// @entry: a #Entry
    ///
    /// Gets the text that is displayed when the entry is empty and unfocused
    ///
    /// Returns: (transfer none): the current value of the placeholder property.
    /// This string is owned by the #Entry and should not be freed or modified.
    ///
    fn get_placeholder(&self) -> Option<String>;

    /// get_text:
    /// @entry: a #Entry
    ///
    /// Get the text displayed on the entry
    ///
    /// Returns: the text for the entry. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String>;

    /// set_icon_highlight_suffix:
    /// @entry: a #Entry
    /// @suffix: the suffix to append to the filename for the highlight version
    ///
    /// Sets the suffix appended to the filename to use for the highlighted version
    /// of the icon. e.g. if you have set your primay icon to "primary-icon.png"
    /// and the suffix to "-highlight" #Entry will look for "primary-icon-highlight.png"
    ///
    fn set_icon_highlight_suffix(&self, suffix: &str);

    /// set_password_char:
    /// @entry: a #Entry
    /// @password_char: character to display instead of text
    ///
    /// Sets the character to display instead of the text. Use 0 to display
    /// the actual text.
    ///
    fn set_password_char(&self, password_char: char);

    /// set_placeholder:
    /// @entry: a #Entry
    /// @text: text to set as the entry hint
    ///
    /// Sets the text to display when the entry is empty and unfocused. When the
    /// entry is displaying the hint, it has a pseudo class of "indeterminate".
    /// A value of None unsets the hint.
    ///
    fn set_placeholder(&self, text: &str);

    /// set_primary_icon_from_file:
    /// @entry: a #Entry
    /// @filename: filename of an icon
    ///
    /// Set the primary icon of the entry to the given filename
    ///
    fn set_primary_icon_from_file(&self, filename: &str);

    /// set_primary_icon_tooltip:
    /// @entry: a #Entry
    /// @text: the primary icon tooltip
    ///
    /// Set the primary icon tooltip text
    ///
    fn set_primary_icon_tooltip_text(&self, text: &str);

    /// set_secondary_icon_from_file:
    /// @entry: a #Entry
    /// @filename: filename of an icon
    ///
    /// Set the secondary icon of the entry to the given filename
    ///
    fn set_secondary_icon_from_file(&self, filename: &str);

    /// set_secondary_icon_tooltip:
    /// @entry: a #Entry
    /// @text: the secondary icon tooltip
    ///
    /// Set the secondary icon tooltip text
    ///
    fn set_secondary_icon_tooltip_text(&self, text: &str);

    /// set_text:
    /// @entry: a #Entry
    /// @text: text to set the entry to
    ///
    /// Sets the text displayed on the entry
    ///
    fn set_text(&self, text: &str);

    /// get_primary_icon_tooltip_text:
    /// @entry: a #Entry
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_primary_icon_tooltip_text(&self) -> Option<String>;

    /// get_secondary_icon_tooltip_text:
    /// @entry: a #Entry
    ///
    /// Returns: the primary icon tooltip
    ///
    fn get_property_secondary_icon_tooltip_text(&self) -> Option<String>;

    fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Entry>> EntryExt for O {
    // /// get_clutter_text:
    // /// @entry: a #Entry
    // ///
    // /// Retrieve the internal #ClutterText so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #ClutterText used by #Entry. The entry is
    // /// owned by the #Entry and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> &Option<clutter::Actor> {
    //     let entry = self.as_ref();
    //     let props = entry.props.borrow();
        
    //     &props.entry
    // }

    /// get_icon_highlight_suffix:
    /// @entry: a #Entry
    ///
    /// Get the suffix appended to the filename to use for the highlighted version
    /// of the icon.
    ///
    /// Returns: the highlight filename suffix. This string is owned by the
    /// #Entry and should not be freed or modified.
    ///
    fn get_icon_highlight_suffix(&self) -> Option<String> {
        let entry = self.as_ref();
        let props = entry.props.borrow();
        
        props.icon_highlight_suffix.clone()
    }

    /// get_password_char:
    /// @entry: a #Entry
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
    /// @entry: a #Entry
    ///
    /// Gets the text that is displayed when the entry is empty and unfocused
    ///
    /// Returns: (transfer none): the current value of the placeholder property.
    /// This string is owned by the #Entry and should not be freed or modified.
    ///
    fn get_placeholder(&self) -> Option<String> {
        let entry = self.as_ref();
        entry.props.borrow().placeholder.clone()
    }

    /// get_text:
    /// @entry: a #Entry
    ///
    /// Get the text displayed on the entry
    ///
    /// Returns: the text for the entry. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String> {
        let entry = self.as_ref();
        // clutter_text_get_text (CLUTTER_TEXT (entry.entry));
        unimplemented!()
    }

    /// set_icon_highlight_suffix:
    /// @entry: a #Entry
    /// @suffix: the suffix to append to the filename for the highlight version
    ///
    /// Sets the suffix appended to the filename to use for the highlighted version
    /// of the icon. e.g. if you have set your primay icon to "primary-icon.png"
    /// and the suffix to "-highlight" #Entry will look for "primary-icon-highlight.png"
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
    /// @entry: a #Entry
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
    /// @entry: a #Entry
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
        //     if !strcmp(clutter_text_get_text(CLUTTER_TEXT(entry.entry)), "") {
        //         stylable_style_pseudo_class_add(STYLABLE(entry), "indeterminate");
        //     } else {
        //         stylable_style_pseudo_class_remove(STYLABLE(entry), "indeterminate");
        //     }
        // }

        // clutter_actor_queue_redraw(CLUTTER_ACTOR(entry));
    }

    /// set_primary_icon_from_file:
    /// @entry: a #Entry
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
    /// @entry: a #Entry
    /// @text: the primary icon tooltip
    ///
    /// Set the primary icon tooltip text
    ///
    fn set_primary_icon_tooltip_text(&self, text: &str) {
        let entry = self.as_ref();

        // if !entry.primary_icon_tooltip {
        //     entry.primary_icon_tooltip = g_object_new(TYPE_TOOLTIP, "text", text, None);

        //     tooltip_set_text(entry.primary_icon_tooltip, text);
        //     clutter_actor_add_child(
        //         CLUTTER_ACTOR(entry),
        //         CLUTTER_ACTOR(entry.primary_icon_tooltip),
        //     );
        // } else {
        //     tooltip_set_text(entry.primary_icon_tooltip, text);
        // }
    }

    /// set_secondary_icon_from_file:
    /// @entry: a #Entry
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
    /// @entry: a #Entry
    /// @text: the secondary icon tooltip
    ///
    /// Set the secondary icon tooltip text
    ///
    fn set_secondary_icon_tooltip_text(&self, text: &str) {
        let entry = self.as_ref();

        // if !entry.secondary_icon_tooltip {
        //     entry.secondary_icon_tooltip = g_object_new(TYPE_TOOLTIP, "text", text, None);

        //     tooltip_set_text(entry.secondary_icon_tooltip, text);
        //     clutter_actor_add_child(
        //         CLUTTER_ACTOR(entry),
        //         CLUTTER_ACTOR(entry.secondary_icon_tooltip),
        //     );
        // } else {
        //     tooltip_set_text(entry.secondary_icon_tooltip, text);
        // }
    }

    /// set_text:
    /// @entry: a #Entry
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

        // clutter_text_set_text (CLUTTER_TEXT (entry.entry), text);
    }

    /// get_primary_icon_tooltip_text:
    /// @entry: a #Entry
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
    /// @entry: a #Entry
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

    fn connect_primary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn primary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"primary-icon-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             primary_icon_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_secondary_icon_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn secondary_icon_clicked_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"secondary-icon-clicked\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             secondary_icon_clicked_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clutter_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clutter-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clutter_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_highlight_suffix_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_highlight_suffix_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-highlight-suffix\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_highlight_suffix_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_password_char_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::password-char\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_password_char_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_placeholder_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::placeholder\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_placeholder_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_primary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_primary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::primary-icon-tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_primary_icon_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_secondary_icon_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_secondary_icon_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::secondary-icon-tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_secondary_icon_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Entry,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Entry>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Entry::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry")
    }
}
