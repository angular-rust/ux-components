#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Entry, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct PathBarProps {
    pub crumbs: Vec<Actor>,
    pub current_level: usize,
    pub overlap: i32,
    pub editable: bool,
    pub clear_on_change: bool,
    pub entry: Option<Entry>,
}

#[derive(Clone, Debug)]
pub struct PathBar {
    props: RefCell<PathBarProps>,
    widget: Widget,
}

impl PathBar {
    pub fn new() -> PathBar {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::path_bar_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for PathBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for PathBar {}
impl Is<PathBar> for PathBar {}

impl AsRef<PathBar> for PathBar {
    fn as_ref(&self) -> &PathBar {
        self
    }
}

impl Is<Widget> for PathBar {}

impl AsRef<Widget> for PathBar {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for PathBar {}

impl AsRef<Actor> for PathBar {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait PathBarExt: 'static {
    /// clear:
    /// @bar: An #PathBar
    ///
    /// Remove all the current buttons
    ///
    fn clear(&self);

    /// get_clear_on_change:
    /// @bar: A #PathBar
    ///
    /// Get the value of the #PathBar:clear-on-change property
    ///
    /// Returns: the value of the "clear-on-change" property
    ///
    fn get_clear_on_change(&self) -> bool;

    /// get_editable:
    /// @bar: A #PathBar
    ///
    /// Get the value of the #PathBar:editable property.
    ///
    /// Returns: the current value of the "editable" property.
    ///
    fn get_editable(&self) -> bool;

    /// get_entry:
    /// @bar: A #PathBar
    ///
    /// Get the Entry used as the editable area in the PathBar.
    ///
    /// Returns: (transfer none): Entry *
    ///
    fn get_entry(&self) -> Option<Entry>;

    fn get_label(&self, level: usize) -> Option<String>;

    fn get_level(&self) -> usize;

    fn get_text(&self) -> Option<String>;

    fn pop(&self) -> usize;

    fn push(&self, name: &str) -> usize;

    /// set_clear_on_change:
    /// @bar: A #PathBar
    /// @clear_on_change: the new value of the property
    ///
    /// Set theh value of the #PathBar:clear-on-change property
    ///
    fn set_clear_on_change(&self, clear_on_change: bool);

    /// set_editable:
    /// @bar: A #PathBar
    /// @editable: #true if the path bar should be editable
    ///
    /// Set the value of the #PathBar:editable property.
    ///
    fn set_editable(&self, editable: bool);

    /// set_label:
    /// @bar: A #PathBar
    /// @level: A #gint
    /// @label: A #gchar
    ///
    /// Set the text on the button specified by @level
    ///
    fn set_label(&self, level: usize, label: &str);

    /// set_text:
    /// @bar: A #PathBar
    /// @text: string to set the editable text to.
    ///
    /// Set the text in the editable area of the #PathBar
    ///
    fn set_text(&self, text: &str);

    fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<PathBar>> PathBarExt for O {
    /// clear:
    /// @bar: An #PathBar
    ///
    /// Remove all the current buttons
    ///
    fn clear(&self) {
        let pathbar = self.as_ref();

        // while pathbar.current_level {
        //     path_bar_pop(bar);
        // }
    }

    /// get_clear_on_change:
    /// @bar: A #PathBar
    ///
    /// Get the value of the #PathBar:clear-on-change property
    ///
    /// Returns: the value of the "clear-on-change" property
    ///
    fn get_clear_on_change(&self) -> bool {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        props.clear_on_change
    }

    /// get_editable:
    /// @bar: A #PathBar
    ///
    /// Get the value of the #PathBar:editable property.
    ///
    /// Returns: the current value of the "editable" property.
    ///
    fn get_editable(&self) -> bool {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        props.editable
    }

    /// get_entry:
    /// @bar: A #PathBar
    ///
    /// Get the Entry used as the editable area in the PathBar.
    ///
    /// Returns: (transfer none): Entry *
    ///
    fn get_entry(&self) -> Option<Entry> {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        props.entry.clone()
    }

    fn get_label(&self, level: usize) -> Option<String> {
        let pathbar = self.as_ref();

        // let crumb = (ClutterActor *)g_list_nth_data(pathbar.crumbs, level - 1);

        // if crumb {
        //     button_get_label(BUTTON(crumb));
        // } else {
        //     None
        // }
        unimplemented!()
    }

    fn get_level(&self) -> usize {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        props.current_level
    }

    fn get_text(&self) -> Option<String> {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        if !props.editable {
            return None;
        }

        // entry_get_text(ENTRY(pathbar.entry))
        unimplemented!()
    }

    fn pop(&self) -> usize {
        let pathbar = self.as_ref();
        let mut props = pathbar.props.borrow_mut();

        if props.clear_on_change {
            // path_bar_set_text(bar, "");
        }

        if props.current_level == 0 {
            return 0;
        }

        // let crumb = g_list_nth_data(pathbar.crumbs, pathbar.current_level - 1);

        // path_bar_animate_button(bar, crumb, true);

        props.current_level -= 1; // FIXME: warn on usize
                                  // path_bar_reset_last_crumb(bar);
                                  // g_object_notify (G_OBJECT (bar), "level");

        props.current_level
    }

    fn push(&self, name: &str) -> usize {
        let pathbar = self.as_ref();
        let mut props = pathbar.props.borrow_mut();

        if props.clear_on_change {
            // path_bar_set_text(bar, "");
        }

        // let crumb = path_bar_button_new(name);
        // actor_add_child(CLUTTER_ACTOR (bar), crumb);
        // pathbar.crumbs = g_list_insert(pathbar.crumbs, crumb, pathbar.current_level);

        // if !pathbar.entry {
        //     if pathbar.current_level {
        //         let old_last_crumb =
        //             g_list_nth_data(pathbar.crumbs, pathbar.current_level - 1);

        //         stylable_set_style_class(STYLABLE(old_last_crumb), None);
        //     }

        //     stylable_set_style_class(STYLABLE(crumb), "End");
        // }

        props.current_level += 1;

        // g_signal_connect(crumb, "clicked",
        //                     G_CALLBACK(path_bar_crumb_clicked_cb), bar);

        // path_bar_animate_button(bar, crumb, false);
        // actor_queue_relayout(CLUTTER_ACTOR(bar));
        // g_object_notify(G_OBJECT(bar), "level");

        props.current_level
    }

    /// set_clear_on_change:
    /// @bar: A #PathBar
    /// @clear_on_change: the new value of the property
    ///
    /// Set theh value of the #PathBar:clear-on-change property
    ///
    fn set_clear_on_change(&self, clear_on_change: bool) {
        let pathbar = self.as_ref();
        let mut props = pathbar.props.borrow_mut();

        if props.clear_on_change != clear_on_change {
            props.clear_on_change = clear_on_change;
            // g_object_notify(G_OBJECT(bar), "clear-on-change");
        }
    }

    /// set_editable:
    /// @bar: A #PathBar
    /// @editable: #true if the path bar should be editable
    ///
    /// Set the value of the #PathBar:editable property.
    ///
    fn set_editable(&self, editable: bool) {
        let pathbar = self.as_ref();
        let mut props = pathbar.props.borrow_mut();

        if props.editable == editable {
            return;
        }

        props.editable = editable;

        if !editable {
            // actor_save_easing_state(pathbar.entry);
            // actor_set_easing_mode(pathbar.entry, CLUTTER_EASE_OUT_QUAD);
            // actor_set_easing_duration(pathbar.entry, 150);
            // actor_set_opacity(pathbar.entry, 0x00);
            // actor_restore_easing_state(pathbar.entry);

            // g_signal_connect_after(pathbar.entry, "transition-stopped::opacity",
            //                         G_CALLBACK(path_bar_entry_faded_cb),
            //                         bar);
        } else {
            // if props.entry {
            //     g_signal_handlers_disconnect_by_func(pathbar.entry,
            //                                             path_bar_entry_faded_cb,
            //                                             bar);
            // } else {
            //     props.entry = entry_new();
            //     actor_add_child(CLUTTER_ACTOR (bar), pathbar.entry);
            //     if CLUTTER_ACTOR_IS_VISIBLE(pathbar.entry) {
            //         actor_set_opacity(pathbar.entry, 0x00);
            //     }
            // }

            // actor_save_easing_state(pathbar.entry);
            // actor_set_easing_mode(pathbar.entry, CLUTTER_EASE_OUT_QUAD);
            // actor_set_easing_duration(pathbar.entry, 150);
            // actor_set_opacity(pathbar.entry, 0xff);
            // actor_restore_easing_state(pathbar.entry);
        }

        // path_bar_reset_last_crumb(bar);

        // g_object_notify(G_OBJECT(bar), "editable");
        // actor_queue_relayout(CLUTTER_ACTOR(bar));
    }

    /// set_label:
    /// @bar: A #PathBar
    /// @level: A #gint
    /// @label: A #gchar
    ///
    /// Set the text on the button specified by @level
    ///
    fn set_label(&self, level: usize, label: &str) {
        let pathbar = self.as_ref();

        // let crumb = (ClutterActor *)g_list_nth_data(pathbar.crumbs, level - 1);

        // if crumb {
        //     button_set_label(BUTTON(crumb), label);
        // }
    }

    /// set_text:
    /// @bar: A #PathBar
    /// @text: string to set the editable text to.
    ///
    /// Set the text in the editable area of the #PathBar
    ///
    fn set_text(&self, text: &str) {
        let pathbar = self.as_ref();
        let props = pathbar.props.borrow();

        if props.editable {
            // entry_set_text(ENTRY(pathbar.entry), text);
        }
    }

    fn connect_property_clear_on_change_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clear_on_change_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clear-on-change\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clear_on_change_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_editable_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::editable\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_editable_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_entry_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::entry\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_entry_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_level_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::PathBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<PathBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&PathBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::level\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_level_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for PathBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathBar")
    }
}
