#![allow(unused_variables)]

// use std::mem::transmute;
// use std::ptr;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use gobject_sys::GValue;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Default, Debug)]
pub struct StyleSheetValue {
    pub string: String,
    pub source: String,
}

#[derive(Clone, Default, Debug)]
pub struct StyleSheet {
    pub selectors: Vec<Selector>,
    pub filenames: Vec<String>,
}

#[derive(Clone, Default, Debug)]
pub struct Selector {
    pub selector_type: String,
    pub id: String,
    pub class: String,
    pub pseudo_class: String,
    // pub parent: Selector,
    // pub ancestor: Selector,
    // pub style: GHashTable,
    pub filename: String, // origin of this selector
    pub line: u32,
    pub position: u32,
    pub priority: i32,
}

#[derive(Clone, Default, Debug)]
pub struct SelectorMatch {
    pub selector: Selector,
    pub score: i32,
}

/// A style cache entry is the unique string representing all the properties
/// that can be matched against in CSS, and the matched properties themselves.
///
#[derive(Clone, Default, Debug)]
pub struct StyleCacheEntry {
    pub style_string: String,
    pub age: u32,
    // pub properties: GHashTable,
}

/// This is the per-stylable cache store. We need a reference back to the
/// parent style so that we can maintain the count of alive stylables.
///
#[derive(Clone, Default, Debug)]
pub struct StylableCache {
    pub styles: Vec<String>,
    pub string: String,
}

#[derive(Clone, Debug)]
pub struct StyleProperty {
    pub value_type: glib::types::Type,
    pub value_name: String,
    pub value: GValue,
}

#[derive(Clone, Default, Debug)]
pub struct Stylable;

#[derive(Clone, Default, Debug)]
pub struct StyleProps {
    pub stylesheet: StyleSheet,
    // pub style_hash: GHashTable,
    // pub node_hash: GHashTable,
    pub alive_stylables: i32,
    // pub cached_matches: GQueue,
    // pub cache_hash: GHashTable,
    pub age: u32,
}

#[derive(Clone, Debug)]
pub struct Style {
    props: RefCell<StyleProps>
}

impl Style {
    pub fn new() -> Style {
        Self {
            props: Default::default()
        }
    }

    pub fn get_default() -> Option<Style> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::style_get_default()) }
        unimplemented!()
    }

    pub fn real_load_from_file(&self, filename: &str, data: &str) -> bool {
        // GError *internal_error;
        // gboolean result;

        // g_return_val_if_fail(filename != None, FALSE);

        // if !data && !g_file_test(filename, G_FILE_TEST_IS_REGULAR) {
        //     internal_error = g_error_new (STYLE_ERROR,
        //                                   STYLE_ERROR_INVALID_FILE,
        //                                   "Invalid theme file '%s'", filename);
        //     g_propagate_error (error, internal_error);
        //     return false;
        //   }

        // if !self.stylesheet {
        //     self.stylesheet = style_sheet_new();
        // }

        // if data {
        //   result = style_sheet_add_from_data(self.stylesheet, filename, data,
        //                                          None);
        // } else {
        //   result = style_sheet_add_from_file(self.stylesheet, filename,
        //                                          None);
        // }

        // if !result  {
        //     internal_error = g_error_new(STYLE_ERROR,
        //                                   STYLE_ERROR_PARSE_ERROR,
        //                                   "Could not parse '%s'", filename);
        //     g_propagate_error(error, internal_error);
        //     return FALSE;
        // }

        // // Increment the age so we know if a style cache entry is valid
        // self.age++;

        // g_signal_emit(style, style_signals[CHANGED], 0, None);

        // if !data {
        //     GFile *file;
        //     GFileMonitor *monitor;

        //     file = g_file_new_for_path(filename);
        //     monitor = g_file_monitor(file, G_FILE_MONITOR_NONE, None, None);

        //     if monitor {
        //         g_signal_connect(monitor, "changed", G_CALLBACK(css_file_changed),
        //                           style);
        //     }
        // }

        return true;
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Style {}
impl Is<Style> for Style {}

impl AsRef<Style> for Style {
    fn as_ref(&self) -> &Style {
        self
    }
}

pub const NONE_STYLE: Option<&Style> = None;

pub trait StyleExt: 'static {
    //fn get(&self, stylable: &Stylable, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    //fn get_property(&self, stylable: &Stylable, pspec: &glib::ParamSpec) -> glib::Value;

    //fn get_valist(&self, stylable: &Stylable, first_property_name: &str, va_args: /*Unknown conversion*/Unsupported);

    /// load_from_data:
    /// @style: a #Style
    /// @d: identifier of the style sheet to load
    /// @data: CSS data to parse
    /// @error: a #GError or #None
    ///
    /// Load style information from @data, using @id to identify the stylesheet.
    /// @id is usually the file name of the style sheet, which is used in the search
    /// path when loading url resources.
    ///
    /// returns: #true if the style information was loaded successfully. Returns
    /// #false on error.
    ///
    fn load_from_data(&self, id: &str, data: &str) -> Result<(), glib::Error>;

    /// load_from_file:
    /// @style: a #Style
    /// @filename: filename of the style sheet to load
    /// @error: a #GError or #None
    ///
    /// Load style information from the specified file.
    ///
    /// returns: #true if the style information was loaded successfully. Returns
    /// #false on error.
    ///
    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error>;

    fn load_from_resource(&self, path: &str) -> Result<(), glib::Error>;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Style>> StyleExt for O {
    //fn get(&self, stylable: &Stylable, first_property_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:style_get() }
    //}

    //fn get_property(&self, stylable: &Stylable, pspec: &glib::ParamSpec) -> glib::Value {
    //    unsafe { TODO: call ffi:style_get_property() }
    //}

    //fn get_valist(&self, stylable: &Stylable, first_property_name: &str, va_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi:style_get_valist() }
    //}

    /// load_from_data:
    /// @style: a #Style
    /// @d: identifier of the style sheet to load
    /// @data: CSS data to parse
    /// @error: a #GError or #None
    ///
    /// Load style information from @data, using @id to identify the stylesheet.
    /// @id is usually the file name of the style sheet, which is used in the search
    /// path when loading url resources.
    ///
    /// returns: #true if the style information was loaded successfully. Returns
    /// #false on error.
    ///
    fn load_from_data(&self, id: &str, data: &str) -> Result<(), glib::Error> {
        let style = self.as_ref();

        // style_real_load_from_file(style, id, data, error, 0);
        unimplemented!()
    }

    /// load_from_file:
    /// @style: a #Style
    /// @filename: filename of the style sheet to load
    /// @error: a #GError or #None
    ///
    /// Load style information from the specified file.
    ///
    /// returns: #true if the style information was loaded successfully. Returns
    /// #false on error.
    ///
    fn load_from_file(&self, filename: &str) -> Result<(), glib::Error> {
        let style = self.as_ref();
        // style_real_load_from_file (style, filename, None, error, 0);
        unimplemented!()
    }

    fn load_from_resource(&self, path: &str) -> Result<(), glib::Error> {
        // GBytes *bytes;
        // GError *internal_error = None;
        // gchar *id;

        // bytes = g_resources_lookup_data(path, G_RESOURCE_LOOKUP_FLAGS_NONE,
        //                                 &internal_error);

        // if !bytes && internal_error {
        //     g_propagate_error(error, internal_error);

        //     return false;
        // }

        // id = g_strconcat("resource://", path, None);

        // style_real_load_from_file(style, id, g_bytes_get_data (bytes, None),  error, 0);
        // g_bytes_unref(bytes);

        // return true;
        unimplemented!()
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Style,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Style>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Style::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"changed\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             changed_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Style")
    }
}
