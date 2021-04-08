#![allow(unused_variables)]

// use std::mem::transmute;
use super::{Adjustment, Orientation, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Copy, Debug)]
pub enum ScrollBarDirection {
    None,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct ScrollBar {
    pub adjustment: Option<Adjustment>,
    pub capture_handler: u64,
    pub x_origin: f32,
    pub y_origin: f32,
    pub bw_stepper: Option<clutter::Actor>,
    pub fw_stepper: Option<clutter::Actor>,
    pub trough: Option<clutter::Actor>,
    pub handle: Option<clutter::Actor>,
    pub move_x: f32,
    pub move_y: f32,
    pub handle_min_size: u32,

    // Trough-click handling.
    pub paging_direction: ScrollBarDirection,
    pub paging_source_id: u32,
    pub paging_event_no: u32,
    pub stepper_forward: bool,
    pub stepper_source_id: u32,
    pub orientation: Orientation,
    widget: Widget,
}

impl ScrollBar {
    pub fn new() -> ScrollBar {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::scroll_bar_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn with_adjustment<P: Is<Adjustment>>(adjustment: &P) -> ScrollBar {
    //     // skip_assert_initialized!();
    //     // unsafe {
    //     //     clutter::Actor::from_glib_none(ffi::scroll_bar_new_with_adjustment(
    //     //         adjustment.as_ref().to_glib_none().0,
    //     //     ))
    //     //     .unsafe_cast()
    //     // }
    //     unimplemented!()
    // }
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ScrollBar {}
impl Is<ScrollBar> for ScrollBar {}

impl AsRef<ScrollBar> for ScrollBar {
    fn as_ref(&self) -> &ScrollBar {
        self
    }
}

impl Is<Widget> for ScrollBar {}

impl AsRef<Widget> for ScrollBar {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for ScrollBar {}

impl AsRef<clutter::Actor> for ScrollBar {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_SCROLL_BAR: Option<&ScrollBar> = None;

pub trait ScrollBarExt: 'static {
    /// get_adjustment:
    /// @bar: a #ScrollBar
    ///
    /// Gets the adjustment object that stores the current position
    /// of the scrollbar.
    ///
    fn get_adjustment(&self) -> Option<Adjustment>;

    fn get_orientation(&self) -> Orientation;

    fn set_adjustment<P: Is<Adjustment>>(&self, adjustment: &P);

    fn set_orientation(&self, orientation: Orientation);

    fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ScrollBar>> ScrollBarExt for O {
    /// get_adjustment:
    /// @bar: a #ScrollBar
    ///
    /// Gets the adjustment object that stores the current position
    /// of the scrollbar.
    ///
    fn get_adjustment(&self) -> Option<Adjustment> {
        let scrollbar = self.as_ref();
        scrollbar.adjustment.clone()
    }

    fn get_orientation(&self) -> Orientation {
        let scrollbar = self.as_ref();
        scrollbar.orientation
    }

    fn set_adjustment<P: Is<Adjustment>>(&self, adjustment: &P) {
        let scrollbar = self.as_ref();

        // if scrollbar.adjustment {
        //     g_signal_handlers_disconnect_by_func(
        //         scrollbar.adjustment,
        //         clutter_actor_queue_relayout,
        //         bar,
        //     );
        //     g_signal_handlers_disconnect_by_func(
        //         scrollbar.adjustment,
        //         clutter_actor_queue_relayout,
        //         bar,
        //     );
        //     g_object_unref(scrollbar.adjustment);
        //     scrollbar.adjustment = None;
        // }

        // if adjustment {
        //     scrollbar.adjustment = g_object_ref(adjustment);

        //     g_signal_connect_swapped(
        //         scrollbar.adjustment,
        //         "notify::value",
        //         G_CALLBACK(clutter_actor_queue_relayout),
        //         bar,
        //     );
        //     g_signal_connect_swapped(
        //         scrollbar.adjustment,
        //         "changed",
        //         G_CALLBACK(clutter_actor_queue_relayout),
        //         bar,
        //     );

        //     clutter_actor_queue_relayout(CLUTTER_ACTOR(bar));
        // }
    }

    fn set_orientation(&self, orientation: Orientation) {
        let scrollbar = self.as_ref();

        if orientation != scrollbar.orientation {
            // scrollbar.orientation = orientation;

            // if scrollbar.orientation {
            //     stylable_set_style_class(STYLABLE(scrollbar.bw_stepper), "up-stepper");
            //     stylable_set_style_class(STYLABLE(scrollbar.fw_stepper), "down-stepper");
            //     stylable_set_style_class(STYLABLE(scrollbar.handle), "vhandle");
            //     stylable_set_style_class(STYLABLE(scrollbar.trough), "vtrough");
            // } else {
            //     stylable_set_style_class(STYLABLE(scrollbar.fw_stepper), "forward-stepper");
            //     stylable_set_style_class(STYLABLE(scrollbar.bw_stepper), "backward-stepper");
            //     stylable_set_style_class(STYLABLE(scrollbar.handle), "hhandle");
            //     stylable_set_style_class(STYLABLE(scrollbar.trough), "htrough");
            // }

            // clutter_actor_queue_relayout(CLUTTER_ACTOR(bar));
            // g_object_notify(G_OBJECT(bar), "orientation");
        }
    }

    fn connect_scroll_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn scroll_start_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"scroll-start\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             scroll_start_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_scroll_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn scroll_stop_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"scroll-stop\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             scroll_stop_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::adjustment\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_adjustment_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollBar,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollBar>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollBar::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::orientation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_orientation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ScrollBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollBar")
    }
}
