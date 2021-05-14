#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, HandlerId, Widget};
use std::{cell::RefCell, fmt};

#[derive(Default, Debug)]
pub struct IconProps {
    pub icon_set: bool,
    pub size_set: bool,
    pub is_content_image: bool,
    pub icon_texture: Option<dx::Texture>,
    pub icon_name: Option<String>,
    pub icon_suffix: Option<String>,
    pub icon_size: usize,
}

#[derive(Debug)]
pub struct Icon {
    props: RefCell<IconProps>,
    inner: Widget,
}

impl Icon {
    pub fn new() -> Icon {
        let props = Default::default();

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component
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
        &self.inner
    }
}

impl Is<Actor> for Icon {}

impl AsRef<Actor> for Icon {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.inner.as_ref();
        actor
    }
}

pub trait IconExt: 'static {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_size(&self) -> usize;

    fn set_icon_name(&self, icon_name: Option<String>);

    fn set_icon_size(&self, size: usize);

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Icon>> IconExt for O {
    fn get_icon_name(&self) -> Option<String> {
        let icon = self.as_ref();
        let props = icon.props.borrow();

        props.icon_name.clone()
    }

    fn get_icon_size(&self) -> usize {
        let icon = self.as_ref();
        let props = icon.props.borrow();

        props.icon_size
    }

    fn set_icon_name(&self, icon_name: Option<String>) {
        let icon = self.as_ref();
        let mut props = icon.props.borrow_mut();

        // // Unset the icon name if necessary
        // if !icon_name {
        //     if props.icon_set {
        //         props.icon_set = false;
        //         stylable_style_changed(STYLABLE (icon), STYLE_CHANGED_NONE);
        //     }

        //     return;
        // }

        props.icon_set = true;

        // // Check if there's no change
        // if props.icon_name && g_str_equal (props.icon_name, icon_name) {
        //     return;
        // }

        // props.icon_name = g_strdup(icon_name);

        // icon_update(icon);

        // g_object_notify(G_OBJECT(icon), "icon-name");
    }

    fn set_icon_size(&self, size: usize) {
        let icon = self.as_ref();
        let mut props = icon.props.borrow_mut();

        if size == 0 {
            if props.size_set {
                props.size_set = false;
                // stylable_style_changed(STYLABLE(icon), STYLE_CHANGED_NONE);
            }
            return;
        } else if props.icon_size != size {
            props.icon_size = size;
            // icon_update(icon);
            // g_object_notify(G_OBJECT(icon), "icon-size");
        }

        props.size_set = true;
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
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
