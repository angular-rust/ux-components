#![allow(unused_variables)]

// use std::mem::transmute;
use super::{ScrollPolicy, Widget};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct ScrollView {
    pub child: Option<clutter::Actor>,
    pub hscroll: Option<clutter::Actor>,
    pub vscroll: Option<clutter::Actor>,
    pub mouse_scroll: bool,
    pub scrollbar_width: u32,
    pub scrollbar_height: u32,
    pub scroll_policy: ScrollPolicy,
    pub scroll_visibility: ScrollPolicy,
}

impl ScrollView {
    pub fn new() -> ScrollView {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::scroll_view_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for ScrollView {}
impl Is<ScrollView> for ScrollView {}

impl AsRef<ScrollView> for ScrollView {
    fn as_ref(&self) -> &ScrollView {
        self
    }
}

pub const NONE_SCROLL_VIEW: Option<&ScrollView> = None;

pub trait ScrollViewExt: 'static {
    /// scroll_view_ensure_visible:
    /// @scroll: A #ScrollView
    /// @geometry: The region to make visible
    ///
    /// Ensures that a given region is visible in the ScrollView, with the top-left
    /// taking precedence.
    ///
    fn ensure_visible(&self, geometry: &clutter::Geometry);

    fn get_enable_mouse_scrolling(&self) -> bool;

    fn get_scroll_policy(&self) -> ScrollPolicy;

    fn get_scroll_visibility(&self) -> ScrollPolicy;

    fn set_enable_mouse_scrolling(&self, enabled: bool);

    fn set_scroll_policy(&self, policy: ScrollPolicy);

    fn set_scroll_visibility(&self, visibility: ScrollPolicy);

    fn connect_property_enable_mouse_scrolling_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_scroll_visibility_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<ScrollView>> ScrollViewExt for O {
    /// scroll_view_ensure_visible:
    /// @scroll: A #ScrollView
    /// @geometry: The region to make visible
    ///
    /// Ensures that a given region is visible in the ScrollView, with the top-left
    /// taking precedence.
    ///
    fn ensure_visible(&self, geometry: &clutter::Geometry) {
        let scrollview = self.as_ref();

        // _scroll_view_ensure_visible_axis(SCROLL_BAR(scrollview.hscroll),
        //                                geometry.x,
        //                                geometry.x + geometry.width);
        // _scroll_view_ensure_visible_axis(SCROLL_BAR(scrollview.vscroll),
        //                                geometry.y,
        //                                geometry.y + geometry.height);
    }

    fn get_enable_mouse_scrolling(&self) -> bool {
        let scrollview = self.as_ref();
        scrollview.mouse_scroll
    }

    fn get_scroll_policy(&self) -> ScrollPolicy {
        let scrollview = self.as_ref();
        scrollview.scroll_policy
    }

    fn get_scroll_visibility(&self) -> ScrollPolicy {
        let scrollview = self.as_ref();
        scrollview.scroll_visibility
    }

    fn set_enable_mouse_scrolling(&self, enabled: bool) {
        let scrollview = self.as_ref();

        if scrollview.mouse_scroll != enabled {
            // scrollview.mouse_scroll = enabled;

            // // make sure we can receive mouse wheel events */
            // if enabled {
            //     clutter_actor_set_reactive((ClutterActor *)scroll, true);
            // }
            // g_object_notify(G_OBJECT(scroll), "enable-mouse-scrolling");
        }
    }

    fn set_scroll_policy(&self, policy: ScrollPolicy) {
        let scrollview = self.as_ref();

        if scrollview.scroll_policy != policy {
            // scrollview.scroll_policy = policy;

            // g_object_notify(G_OBJECT(scroll), "scroll-policy");

            // clutter_actor_queue_relayout(CLUTTER_ACTOR(scroll));
        }
    }

    fn set_scroll_visibility(&self, visibility: ScrollPolicy) {
        let scrollview = self.as_ref();

        if scrollview.scroll_visibility != visibility {
            // scrollview.scroll_visibility = visibility;

            // g_object_notify(G_OBJECT(scroll), "scroll-visibility");

            // clutter_actor_queue_relayout(CLUTTER_ACTOR(scroll));
        }
    }

    fn connect_property_enable_mouse_scrolling_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_enable_mouse_scrolling_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::enable-mouse-scrolling\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_enable_mouse_scrolling_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_scroll_policy_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scroll-policy\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scroll_policy_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_scroll_visibility_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_scroll_visibility_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::ScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<ScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&ScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scroll-visibility\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scroll_visibility_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for ScrollView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollView")
    }
}
