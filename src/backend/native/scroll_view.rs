#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Geometry, ScrollPolicy, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct ScrollViewProps {
    pub child: Option<Actor>,
    pub hscroll: Option<Actor>,
    pub vscroll: Option<Actor>,
    pub mouse_scroll: bool,
    pub scrollbar_width: u32,
    pub scrollbar_height: u32,
    pub scroll_policy: ScrollPolicy,
    pub scroll_visibility: ScrollPolicy,
}

#[derive(Clone, Debug)]
pub struct ScrollView {
    props: RefCell<ScrollViewProps>,
    widget: Widget,
}

impl ScrollView {
    pub fn new() -> ScrollView {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::scroll_view_new()).unsafe_cast() }
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

impl Is<Widget> for ScrollView {}

impl AsRef<Widget> for ScrollView {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for ScrollView {}

impl AsRef<Actor> for ScrollView {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait ScrollViewExt: 'static {
    /// scroll_view_ensure_visible:
    /// @scroll: A #ScrollView
    /// @geometry: The region to make visible
    ///
    /// Ensures that a given region is visible in the ScrollView, with the top-left
    /// taking precedence.
    ///
    fn ensure_visible(&self, geometry: &Geometry);

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
    fn ensure_visible(&self, geometry: &Geometry) {
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
        let props = scrollview.props.borrow();

        props.mouse_scroll
    }

    fn get_scroll_policy(&self) -> ScrollPolicy {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.scroll_policy
    }

    fn get_scroll_visibility(&self) -> ScrollPolicy {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.scroll_visibility
    }

    fn set_enable_mouse_scrolling(&self, enabled: bool) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.mouse_scroll != enabled {
            props.mouse_scroll = enabled;

            // make sure we can receive mouse wheel events */
            if enabled {
                // actor_set_reactive((ClutterActor *)scroll, true);
            }
            // g_object_notify(G_OBJECT(scroll), "enable-mouse-scrolling");
        }
    }

    fn set_scroll_policy(&self, policy: ScrollPolicy) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.scroll_policy != policy {
            props.scroll_policy = policy;
            // g_object_notify(G_OBJECT(scroll), "scroll-policy");
            // actor_queue_relayout(CLUTTER_ACTOR(scroll));
        }
    }

    fn set_scroll_visibility(&self, visibility: ScrollPolicy) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.scroll_visibility != visibility {
            props.scroll_visibility = visibility;
            // g_object_notify(G_OBJECT(scroll), "scroll-visibility");
            // actor_queue_relayout(CLUTTER_ACTOR(scroll));
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
