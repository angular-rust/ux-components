#![allow(unused_variables)]

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct IconThemeProps {
    pub override_theme: bool,
    pub search_paths: Vec<String>,
    // pub icon_hash: GHashTable,
    // pub theme_path_hash: GHashTable,
    pub theme: Option<String>,
    // pub theme_file: File, // GKeyFile
    pub theme_fallbacks: Vec<String>,
    // pub hicolor_file: File, // GKeyFile
}

#[derive(Clone, Debug)]
pub struct IconTheme {
    props: RefCell<IconThemeProps>,
}

impl IconTheme {
    pub fn new() -> IconTheme {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::icon_theme_new()) }
        unimplemented!()
    }

    pub fn get_default() -> Option<IconTheme> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::icon_theme_get_default()) }
        unimplemented!()
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for IconTheme {}
impl Is<IconTheme> for IconTheme {}

impl AsRef<IconTheme> for IconTheme {
    fn as_ref(&self) -> &IconTheme {
        self
    }
}

pub trait IconThemeExt: 'static {
    ///get_search_paths:
    /// @theme: a #IconTheme
    ///
    /// Gets the directories the #IconTheme will search in to find icons.
    ///
    /// Return value: (element-type utf8) (transfer none): the search paths
    ///
    fn get_search_paths(&self) -> Vec<String>;

    ///get_theme_name:
    /// @theme: A #IconTheme
    ///
    /// Get the value of the #IconTheme:theme-name property.
    ///
    /// Returns: the current value of the "theme-name" property.
    ///
    fn get_theme_name(&self) -> Option<String>;

    fn has_icon(&self, icon_name: &str) -> bool;

    ///lookup:
    /// @theme: an #IconTheme
    /// @icon_name: The name of the icon
    /// @size: The desired size of the icon
    ///
    /// If the icon is available, returns a #CoglHandle of the icon.
    ///
    /// Return value: (transfer none): a #CoglHandle of the icon, or %None.
    ///
    fn lookup(&self, icon_name: &str, size: i32) -> Option<dx::Handle>;

    ///set_search_paths:
    /// @theme: a #IconTheme
    /// @paths: (element-type utf8): a list of search paths
    ///
    /// Sets the directories the #IconTheme will search in to find icons.
    /// By default, it will look in the default system and local icon directories.
    ///
    fn set_search_paths(&self, paths: &[&str]);

    ///set_theme_name:
    /// @theme: A #IconTheme
    /// @theme_name: the name of an icon theme to load, or %None
    ///
    /// Set the value of the #IconTheme:theme-name property. This will cause the
    /// icon theme to be loaded if it differs from the existing theme name. If the
    /// theme could not be loaded, it will fall back to using the default icon theme
    /// (hicolor).
    ///
    /// This will override the system's theme setting. To revert to the system
    /// icon theme, this function can be called with a %None @theme_name argument.
    ///
    fn set_theme_name(&self, theme_name: &str);

    fn connect_property_theme_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<IconTheme>> IconThemeExt for O {
    ///get_search_paths:
    /// @theme: a #IconTheme
    ///
    /// Gets the directories the #IconTheme will search in to find icons.
    ///
    /// Return value: (element-type utf8) (transfer none): the search paths
    ///
    fn get_search_paths(&self) -> Vec<String> {
        let icontheme = self.as_ref();
        let props = icontheme.props.borrow();

        props.search_paths.clone()
    }

    ///get_theme_name:
    /// @theme: A #IconTheme
    ///
    /// Get the value of the #IconTheme:theme-name property.
    ///
    /// Returns: the current value of the "theme-name" property.
    ///
    fn get_theme_name(&self) -> Option<String> {
        let icontheme = self.as_ref();
        let props = icontheme.props.borrow();

        props.theme.clone()
    }

    fn has_icon(&self, icon_name: &str) -> bool {
        let icontheme = self.as_ref();
        // if icon_theme_get_icons(theme, icon_name) {
        //     return true;
        // }

        false
    }

    /// lookup:
    /// @theme: an #IconTheme
    /// @icon_name: The name of the icon
    /// @size: The desired size of the icon
    ///
    /// If the icon is available, returns a #CoglHandle of the icon.
    ///
    /// Return value: (transfer none): a #CoglHandle of the icon, or %None.
    ///
    fn lookup(&self, icon_name: &str, size: i32) -> Option<dx::Handle> {
        let icontheme = self.as_ref();
        unimplemented!()
    }

    ///set_search_paths:
    /// @theme: a #IconTheme
    /// @paths: (element-type utf8): a list of search paths
    ///
    /// Sets the directories the #IconTheme will search in to find icons.
    /// By default, it will look in the default system and local icon directories.
    ///
    fn set_search_paths(&self, paths: &[&str]) {
        let icontheme = self.as_ref();

        // icontheme.search_paths.clear()

        // icontheme.search_paths = g_list_copy ((GList *)paths);
        // for (p = priv->search_paths; p; p = p->next) {
        //     p->data = g_strdup ((const gchar *)p->data);
        // }
    }

    ///set_theme_name:
    /// @theme: A #IconTheme
    /// @theme_name: the name of an icon theme to load, or %None
    ///
    /// Set the value of the #IconTheme:theme-name property. This will cause the
    /// icon theme to be loaded if it differs from the existing theme name. If the
    /// theme could not be loaded, it will fall back to using the default icon theme
    /// (hicolor).
    ///
    /// This will override the system's theme setting. To revert to the system
    /// icon theme, this function can be called with a %None @theme_name argument.
    ///
    fn set_theme_name(&self, theme_name: &str) {
        let icontheme = self.as_ref();

        // if !theme_name {
        //     if icontheme.override_theme {
        //         gchar *system_theme = None;
        //         Settings *settings = settings_get_default();

        //         g_object_get(settings, "icon-theme", &system_theme, None);
        //         icontheme.override_theme = false;
        //         icon_theme_set_theme_name(theme, system_theme);
        //         icontheme.override_theme = false;
        //     }

        //     return;
        // }

        // icontheme.override_theme = true;

        // if g_str_equal(theme_name, "hicolor") {
        //     return;
        // }

        // if icontheme.theme && g_str_equal(icontheme.theme, theme_name) {
        //     return;
        // }

        // // Clear old data
        // g_hash_table_remove_all(icontheme.icon_hash);

        // if icontheme.theme_file {
        //     g_hash_table_remove(icontheme.theme_path_hash, icontheme.theme_file);
        //     g_key_file_free(icontheme.theme_file);
        // }

        // while icontheme.theme_fallbacks  {
        //     g_hash_table_remove(icontheme.theme_path_hash, icontheme.theme_fallbacks.data);
        //     g_key_file_free((GKeyFile *)icontheme.theme_fallbacks.data);
        //     icontheme.theme_fallbacks = g_list_delete_link(icontheme.theme_fallbacks,
        //         icontheme.theme_fallbacks);
        // }

        // // Load new theme file
        // icontheme.theme = g_strdup (theme_name);
        // icontheme.theme_file = icon_theme_load_theme (theme, theme_name);

        // if !icontheme.theme_file {
        //     g_warning("Error loading \"%s\" icon theme", icontheme.theme);
        //     return;
        // }

        // // Load fallbacks
        // icon_theme_load_fallbacks(theme, icontheme.theme_file, true);

        // g_object_notify(G_OBJECT(theme), "theme-name");
    }

    fn connect_property_theme_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_theme_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::IconTheme,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<IconTheme>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&IconTheme::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::theme-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_theme_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconTheme")
    }
}
