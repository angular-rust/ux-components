#![allow(unused_variables)]

// use std::mem::transmute;
use super::Align;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct BoxLayoutChildProps {
    pub expand: bool,
    pub x_fill: bool,
    pub y_fill: bool,
    pub x_align: Align,
    pub y_align: Align,
    pub parent: clutter::ChildMeta,
}

#[derive(Clone, Debug)]
pub struct BoxLayoutChild {
    props: RefCell<BoxLayoutChildProps>,
}

impl Object for BoxLayoutChild {}
impl Is<BoxLayoutChild> for BoxLayoutChild {}

impl AsRef<BoxLayoutChild> for BoxLayoutChild {
    fn as_ref(&self) -> &BoxLayoutChild {
        self
    }
}

pub const NONE_BOX_LAYOUT_CHILD: Option<&BoxLayoutChild> = None;

// TODO: should implement with childs
pub trait BoxLayoutChildExt: 'static {
    /// get_expand:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:expand property
    ///
    /// Returns: the current value of the "expand" property
    ///
    fn get_property_expand(&self) -> bool;

    /// set_expand:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @expand: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:expand property.
    ///
    fn set_property_expand(&self, expand: bool);

    /// get_x_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:x-align property
    ///
    /// Returns: the current value of the "x-align" property
    ///
    fn get_property_x_align(&self) -> Align;

    /// set_x_align:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @x_align: An #Align
    ///
    /// Set the value of the #BoxLayoutChild:x-align property.
    ///
    fn set_property_x_align(&self, x_align: Align);

    /// get_x_fill:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:x-fill property.
    ///
    /// Returns: the current value of the "x-fill" property.
    ///
    fn get_property_x_fill(&self) -> bool;

    /// set_x_fill:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @x_fill: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:x-fill property.
    ///
    fn set_property_x_fill(&self, x_fill: bool);

    /// get_y_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:y-align property.
    ///
    /// Returns: the current value of the "y-align" property.
    ///
    fn get_property_y_align(&self) -> Align;

    /// set_y_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    /// @y_align: An #Align
    ///
    /// Set the value of the #BoxLayoutChild:y-align property.
    ///
    fn set_property_y_align(&self, y_align: Align);

    /// get_y_fill:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:y-fill property
    ///
    /// Returns: the current value of the "y-fill" property
    ///
    fn get_property_y_fill(&self) -> bool;

    /// set_y_fill:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    /// @y_fill: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:y-fill property.
    ///
    fn set_property_y_fill(&self, y_fill: bool);

    fn connect_property_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<BoxLayoutChild>> BoxLayoutChildExt for O {
    /// get_expand:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:expand property
    ///
    /// Returns: the current value of the "expand" property
    ///
    fn get_property_expand(&self) -> bool {
        let boxlayoutchild = self.as_ref();
        boxlayoutchild.props.borrow().expand
    }

    /// set_expand:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @expand: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:expand property.
    ///
    fn set_property_expand(&self, expand: bool) {
        let boxlayoutchild = self.as_ref();
        let mut props = boxlayoutchild.props.borrow_mut();
        props.expand = expand;
    }

    /// get_x_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:x-align property
    ///
    /// Returns: the current value of the "x-align" property
    ///
    fn get_property_x_align(&self) -> Align {
        let boxlayoutchild = self.as_ref();
        boxlayoutchild.props.borrow().x_align
    }

    /// set_x_align:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @x_align: An #Align
    ///
    /// Set the value of the #BoxLayoutChild:x-align property.
    ///
    fn set_property_x_align(&self, x_align: Align) {
        let boxlayoutchild = self.as_ref();
        let mut props = boxlayoutchild.props.borrow_mut();
        props.x_align = x_align;
    }

    /// get_x_fill:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:x-fill property.
    ///
    /// Returns: the current value of the "x-fill" property.
    ///
    fn get_property_x_fill(&self) -> bool {
        let boxlayoutchild = self.as_ref();
        boxlayoutchild.props.borrow().x_fill
    }

    /// set_x_fill:
    /// @box_layout: A #BoxLayout
    /// @child: A #ClutterActor
    /// @x_fill: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:x-fill property.
    ///
    fn set_property_x_fill(&self, x_fill: bool) {
        let boxlayoutchild = self.as_ref();
        let mut props = boxlayoutchild.props.borrow_mut();
        props.x_fill = x_fill;
    }

    /// get_y_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:y-align property.
    ///
    /// Returns: the current value of the "y-align" property.
    ///
    fn get_property_y_align(&self) -> Align {
        let boxlayoutchild = self.as_ref();
        boxlayoutchild.props.borrow().y_align
    }

    /// set_y_align:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    /// @y_align: An #Align
    ///
    /// Set the value of the #BoxLayoutChild:y-align property.
    ///
    fn set_property_y_align(&self, y_align: Align) {
        let boxlayoutchild = self.as_ref();
        let mut props = boxlayoutchild.props.borrow_mut();
        props.y_align = y_align;
    }

    /// get_y_fill:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    ///
    /// Get the value of the #BoxLayoutChild:y-fill property
    ///
    /// Returns: the current value of the "y-fill" property
    ///
    fn get_property_y_fill(&self) -> bool {
        let boxlayoutchild = self.as_ref();
        boxlayoutchild.props.borrow().y_fill
    }

    /// set_y_fill:
    /// @box_layout: An #BoxLayout
    /// @child: A #ClutterActor
    /// @y_fill: A #gboolean
    ///
    /// Set the value of the #BoxLayoutChild:y-fill property.
    ///
    fn set_property_y_fill(&self, y_fill: bool) {
        let boxlayoutchild = self.as_ref();
        let mut props = boxlayoutchild.props.borrow_mut();
        props.y_fill = y_fill;
    }

    fn connect_property_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_expand_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayoutChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayoutChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::expand\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_expand_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayoutChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayoutChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_x_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayoutChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayoutChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayoutChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayoutChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_y_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayoutChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayoutChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayoutChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for BoxLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxLayoutChild")
    }
}
