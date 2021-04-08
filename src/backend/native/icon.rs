#![allow(unused_variables)]

// use std::mem::transmute;
use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Icon {
    pub icon_set: bool,
    pub size_set: bool,
    pub is_content_image: bool,
    pub icon_texture: Option<cogl::Texture>,
    pub icon_name: Option<String>,
    pub icon_suffix: Option<String>,
    pub icon_size: i32,
    widget: Widget,
}

impl Icon {
    pub fn new() -> Icon {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::icon_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Icon {}
impl Is<Icon> for Icon {}

impl AsRef<Icon> for Icon {
    fn as_ref(&self) -> &Icon {
        self
    }
}

impl Is<Widget> for Icon {}

impl AsRef<Widget> for Icon {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for Icon {}

impl AsRef<clutter::Actor> for Icon {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_ICON: Option<&Icon> = None;

pub trait IconExt: 'static {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_size(&self) -> i32;

    fn set_icon_name(&self, icon_name: &str);

    fn set_icon_size(&self, size: i32);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Icon>> IconExt for O {
    fn get_icon_name(&self) -> Option<String> {
        let icon = self.as_ref();
        icon.icon_name.clone()
    }

    fn get_icon_size(&self) -> i32 {
        let icon = self.as_ref();
        icon.icon_size
    }

    fn set_icon_name(&self, icon_name: &str) {
        let icon = self.as_ref();

        // // Unset the icon name if necessary
        // if !icon_name {
        //     if icon.icon_set {
        //         icon.icon_set = false;
        //         stylable_style_changed(STYLABLE (icon), STYLE_CHANGED_NONE);
        //     }

        //     return;
        // }

        // icon.icon_set = true;

        // // Check if there's no change
        // if icon.icon_name && g_str_equal (icon.icon_name, icon_name) {
        //     return;
        // }

        // icon.icon_name = g_strdup(icon_name);

        // icon_update(icon);

        // g_object_notify(G_OBJECT(icon), "icon-name");
    }

    fn set_icon_size(&self, size: i32) {
        let icon = self.as_ref();

        // if size < 0 {
        //     if icon.size_set {
        //         icon.size_set = false;
        //         stylable_style_changed(STYLABLE(icon), STYLE_CHANGED_NONE);
        //     }

        //     return;
        // } else if icon.icon_size != size {
        //     icon.icon_size = size;
        //     icon_update(icon);

        //     g_object_notify(G_OBJECT(icon), "icon-size");
        // }

        // icon.size_set = true;
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Icon,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Icon>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Icon,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Icon>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Icon::from_glib_borrow(this).unsafe_cast_ref())
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
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Icon")
    }
}
