#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Align, ChildMeta, HandlerId};
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct StackChildProps {
    pub parent: ChildMeta,
    pub x_fill: bool,
    pub y_fill: bool,
    pub fit: bool,
    pub crop: bool,
    pub x_align: Align,
    pub y_align: Align,
}

#[derive(Clone, Debug)]
pub struct StackChild {
    props: RefCell<StackChildProps>,
}

impl Object for StackChild {}
impl Is<StackChild> for StackChild {}

impl AsRef<StackChild> for StackChild {
    fn as_ref(&self) -> &StackChild {
        self
    }
}

pub trait StackChildExt: 'static {
    /// get_crop:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:fit property.
    ///
    /// Returns: the current value of the #StackChild:crop property
    ///
    fn get_crop(&self) -> bool;

    /// set_crop:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @crop: A #gboolean
    ///
    /// Set the value of the #StackChild:crop property.
    ///
    fn set_crop(&self, crop: bool);

    /// get_fit:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    ///* Get the value of the #StackChild:fit property.
    ///
    /// Returns: the current value of the #StackChild:fit property
    ///
    fn get_fit(&self) -> bool;

    /// set_fit:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @fit: A #gboolean
    ///
    /// Set the value of the #StackChild:fit property.
    ///
    fn set_fit(&self, fit: bool);

    /// get_x_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:x-align property
    ///
    /// Returns: the current value of the "x-align" property
    ///
    fn get_x_align(&self) -> Align;

    /// set_x_align:
    /// @stack: A #Stack
    /// @child: A #Actor
    /// @x_align: An #Align
    ///
    /// Set the value of the #StackChild:x-align property.
    ///
    fn set_x_align(&self, x_align: Align);

    /// get_x_fill:
    /// @stack: A #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:x-fill property.
    ///
    /// Returns: the current value of the "x-fill" property.
    ///
    fn get_x_fill(&self) -> bool;

    /// set_x_fill:
    /// @stack: A #Stack
    /// @child: A #Actor
    /// @x_fill: A #gboolean
    ///
    /// Set the value of the #StackChild:x-fill property.
    ///
    fn set_x_fill(&self, x_fill: bool);

    /// get_y_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:y-align property.
    ///
    /// Returns: the current value of the "y-align" property.
    ///
    fn get_y_align(&self) -> Align;

    /// set_y_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @y_align: An #Align
    ///
    /// Set the value of the #StackChild:y-align property.
    ///
    fn set_y_align(&self, y_align: Align);

    /// get_y_fill:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:y-fill property
    ///
    /// Returns: the current value of the "y-fill" property
    ///
    fn get_y_fill(&self) -> bool;

    /// set_y_fill:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @y_fill: A #gboolean
    ///
    /// Set the value of the #StackChild:y-fill property.
    ///
    fn set_y_fill(&self, y_fill: bool);

    fn connect_property_crop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<StackChild>> StackChildExt for O {
    /// get_crop:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:fit property.
    ///
    /// Returns: the current value of the #StackChild:crop property
    ///
    fn get_crop(&self) -> bool {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.crop
    }

    /// set_crop:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @crop: A #gboolean
    ///
    /// Set the value of the #StackChild:crop property.
    ///
    fn set_crop(&self, crop: bool) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.crop = crop;
        // actor_queue_relayout(child);
    }

    /// get_fit:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    ///* Get the value of the #StackChild:fit property.
    ///
    /// Returns: the current value of the #StackChild:fit property
    ///
    fn get_fit(&self) -> bool {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.fit
    }

    /// set_fit:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @fit: A #gboolean
    ///
    /// Set the value of the #StackChild:fit property.
    ///
    fn set_fit(&self, fit: bool) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.fit = fit;
        // actor_queue_relayout(child);
    }

    /// get_x_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:x-align property
    ///
    /// Returns: the current value of the "x-align" property
    ///
    fn get_x_align(&self) -> Align {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.x_align
    }

    /// set_x_align:
    /// @stack: A #Stack
    /// @child: A #Actor
    /// @x_align: An #Align
    ///
    /// Set the value of the #StackChild:x-align property.
    ///
    fn set_x_align(&self, x_align: Align) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.x_align = x_align;
        // actor_queue_relayout(child);
    }

    /// get_x_fill:
    /// @stack: A #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:x-fill property.
    ///
    /// Returns: the current value of the "x-fill" property.
    ///
    fn get_x_fill(&self) -> bool {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.x_fill
    }

    /// set_x_fill:
    /// @stack: A #Stack
    /// @child: A #Actor
    /// @x_fill: A #gboolean
    ///
    /// Set the value of the #StackChild:x-fill property.
    ///
    fn set_x_fill(&self, x_fill: bool) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.x_fill = x_fill;
        // actor_queue_relayout(child);
    }

    /// get_y_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:y-align property.
    ///
    /// Returns: the current value of the "y-align" property.
    ///
    fn get_y_align(&self) -> Align {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.y_align
    }

    /// set_y_align:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @y_align: An #Align
    ///
    /// Set the value of the #StackChild:y-align property.
    ///
    fn set_y_align(&self, y_align: Align) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.y_align = y_align;
        // actor_queue_relayout(child);
    }

    /// get_y_fill:
    /// @stack: An #Stack
    /// @child: A #Actor
    ///
    /// Get the value of the #StackChild:y-fill property
    ///
    /// Returns: the current value of the "y-fill" property
    ///
    fn get_y_fill(&self) -> bool {
        let stackchild = self.as_ref();
        let props = stackchild.props.borrow();

        props.y_fill
    }

    /// set_y_fill:
    /// @stack: An #Stack
    /// @child: A #Actor
    /// @y_fill: A #gboolean
    ///
    /// Set the value of the #StackChild:y-fill property.
    ///
    fn set_y_fill(&self, y_fill: bool) {
        let stackchild = self.as_ref();
        let mut props = stackchild.props.borrow_mut();

        props.y_fill = y_fill;
        // actor_queue_relayout(child);
    }

    fn connect_property_crop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_crop_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::crop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_crop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_fit_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fit\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fit_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::x-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_x_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-align\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_align_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_y_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_fill_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::StackChild,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<StackChild>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&StackChild::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::y-fill\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_y_fill_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for StackChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackChild")
    }
}
