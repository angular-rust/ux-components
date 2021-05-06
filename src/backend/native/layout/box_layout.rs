#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Adjustment, Align, Alpha, Orientation, Timeline, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct BoxLayoutProps {
    // GList        *children;
    /// Should we ignore spacing from CSS because
    /// the application set it via set_spacing
    pub ignore_css_spacing: bool,
    pub spacing: u32,
    pub hadjustment: Adjustment,
    pub vadjustment: Adjustment,
    // pub start_allocations: GHashTable,
    pub alpha: Alpha,
    pub is_animating: bool,
    pub enable_animations: bool,
    pub scroll_to_focused: bool,
    pub orientation: Orientation,
    // pub last_focus: Focusable,
    pub timeline: Timeline,
}

#[derive(Clone, Debug)]
pub struct BoxLayout {
    props: RefCell<BoxLayoutProps>,
    widget: Widget,
}

impl BoxLayout {
    pub fn new() -> BoxLayout {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::box_layout_new()).unsafe_cast() }

        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::box_layout_new()) }
        unimplemented!()
    }

    pub fn with_orientation(orientation: Orientation) -> BoxLayout {
        //    unsafe { TODO: call ffi:box_layout_new_with_orientation() }
        unimplemented!()
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for BoxLayout {}
impl Is<BoxLayout> for BoxLayout {}

impl AsRef<BoxLayout> for BoxLayout {
    fn as_ref(&self) -> &BoxLayout {
        self
    }
}

impl Is<Widget> for BoxLayout {}

impl AsRef<Widget> for BoxLayout {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for BoxLayout {}

impl AsRef<Actor> for BoxLayout {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait BoxLayoutExt: 'static {
    fn child_get_expand<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_get_x_align<P: Is<Actor>>(&self, child: &P) -> Align;

    fn child_get_x_fill<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_get_y_align<P: Is<Actor>>(&self, child: &P) -> Align;

    fn child_get_y_fill<P: Is<Actor>>(&self, child: &P) -> bool;

    fn child_set_expand<P: Is<Actor>>(&self, child: &P, expand: bool);

    fn child_set_x_align<P: Is<Actor>>(&self, child: &P, x_align: Align);

    fn child_set_x_fill<P: Is<Actor>>(&self, child: &P, x_fill: bool);

    fn child_set_y_align<P: Is<Actor>>(&self, child: &P, y_align: Align);

    fn child_set_y_fill<P: Is<Actor>>(&self, child: &P, y_fill: bool);

    /// get_enable_animations:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:enable-animations property.
    ///
    /// Returns: #true if animations enabled
    ///
    fn get_enable_animations(&self) -> bool;

    /// get_orientation:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:orientation property.
    ///
    fn get_orientation(&self) -> Orientation;

    /// get_scroll_to_focused:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:scroll-to-focused property.
    ///
    /// Returns: #true if automatically scrolling to the focused actor is enabled
    ///
    fn get_scroll_to_focused(&self) -> bool;

    /// get_spacing:
    /// @box: A #BoxLayout
    ///
    /// Get the spacing between children in pixels
    ///
    /// Returns: the spacing value
    ///
    fn get_spacing(&self) -> u32;

    /// insert_actor:
    /// @box: a #BoxLayout
    /// @actor: the #Actor actor to add to the box layout
    /// @position: the position where to insert the actor
    ///
    /// Inserts @actor at @position in @box.
    ///
    fn insert_actor<P: Is<Actor>>(&self, actor: &P, position: i32);

    //fn insert_actor_with_properties<P: Is<Actor>>(&self, actor: &P, position: i32, first_property: &str, : /*Unknown conversion*/Fundamental: VarArgs);

    /// set_enable_animations:
    /// @box: A #BoxLayout
    /// @enable_animations: #true to enable animations
    ///
    /// Enable animations when certain properties change.
    ///
    fn set_enable_animations(&self, enable_animations: bool);

    /// set_orientation:
    /// @box: A #BoxLayout
    /// @orientation: orientation value for the layout
    ///
    /// Set the orientation of the box layout.
    ///
    fn set_orientation(&self, orientation: Orientation);

    /// set_scroll_to_focused:
    /// @box: A #BoxLayout
    /// @scroll_to_focused: #true to enable automatically scrolling to the
    ///   focused actor
    ///
    /// Enables or disables automatic scrolling to the focused actor.
    ///
    fn set_scroll_to_focused(&self, scroll_to_focused: bool);

    /// set_spacing:
    /// @box: A #BoxLayout
    /// @spacing: the spacing value
    ///
    /// Set the amount of spacing between children in pixels
    ///
    fn set_spacing(&self, spacing: u32);

    fn connect_property_enable_animations_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scroll_to_focused_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<BoxLayout>> BoxLayoutExt for O {
    fn child_get_expand<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::box_layout_child_get_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_x_align<P: Is<Actor>>(&self, child: &P) -> Align {
        //    unsafe { TODO: call ffi:box_layout_child_get_x_align() }
        unimplemented!()
    }

    fn child_get_x_fill<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::box_layout_child_get_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_get_y_align<P: Is<Actor>>(&self, child: &P) -> Align {
        //    unsafe { TODO: call ffi:box_layout_child_get_y_align() }
        unimplemented!()
    }

    fn child_get_y_fill<P: Is<Actor>>(&self, child: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::box_layout_child_get_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn child_set_expand<P: Is<Actor>>(&self, child: &P, expand: bool) {
        // unsafe {
        //     ffi::box_layout_child_set_expand(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         expand.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_x_align<P: Is<Actor>>(&self, child: &P, x_align: Align) {
        //    unsafe { TODO: call ffi:box_layout_child_set_x_align() }
    }

    fn child_set_x_fill<P: Is<Actor>>(&self, child: &P, x_fill: bool) {
        // unsafe {
        //     ffi::box_layout_child_set_x_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         x_fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn child_set_y_align<P: Is<Actor>>(&self, child: &P, y_align: Align) {
        //    unsafe { TODO: call ffi:box_layout_child_set_y_align() }
    }

    fn child_set_y_fill<P: Is<Actor>>(&self, child: &P, y_fill: bool) {
        // unsafe {
        //     ffi::box_layout_child_set_y_fill(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         y_fill.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// get_enable_animations:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:enable-animations property.
    ///
    /// Returns: #true if animations enabled
    ///
    fn get_enable_animations(&self) -> bool {
        let boxlayout = self.as_ref();
        boxlayout.props.borrow().enable_animations
    }

    /// get_orientation:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:orientation property.
    ///
    fn get_orientation(&self) -> Orientation {
        let boxlayout = self.as_ref();
        boxlayout.props.borrow().orientation
    }

    /// get_scroll_to_focused:
    /// @box: A #BoxLayout
    ///
    /// Get the value of the #BoxLayout:scroll-to-focused property.
    ///
    /// Returns: #true if automatically scrolling to the focused actor is enabled
    ///
    fn get_scroll_to_focused(&self) -> bool {
        let boxlayout = self.as_ref();
        boxlayout.props.borrow().scroll_to_focused
    }

    /// get_spacing:
    /// @box: A #BoxLayout
    ///
    /// Get the spacing between children in pixels
    ///
    /// Returns: the spacing value
    ///
    fn get_spacing(&self) -> u32 {
        let boxlayout = self.as_ref();
        boxlayout.props.borrow().spacing
    }

    /// insert_actor:
    /// @box: a #BoxLayout
    /// @actor: the #Actor actor to add to the box layout
    /// @position: the position where to insert the actor
    ///
    /// Inserts @actor at @position in @box.
    ///
    fn insert_actor<P: Is<Actor>>(&self, actor: &P, position: i32) {
        let boxlayout = self.as_ref();
        let actor = actor.as_ref();

        // actor_insert_child_at_index (CLUTTER_ACTOR (box), actor, position);
    }

    //fn insert_actor_with_properties<P: Is<Actor>>(&self, actor: &P, position: i32, first_property: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:box_layout_insert_actor_with_properties() }
    //}

    /// set_enable_animations:
    /// @box: A #BoxLayout
    /// @enable_animations: #true to enable animations
    ///
    /// Enable animations when certain properties change.
    ///
    fn set_enable_animations(&self, enable_animations: bool) {
        let boxlayout = self.as_ref();
        let mut props = boxlayout.props.borrow_mut();

        if props.enable_animations != enable_animations {
            props.enable_animations = enable_animations;
            // actor_queue_relayout ((ClutterActor*) box);
            // g_object_notify (G_OBJECT (box), "enable-animations");
        }
    }

    /// set_orientation:
    /// @box: A #BoxLayout
    /// @orientation: orientation value for the layout
    ///
    /// Set the orientation of the box layout.
    ///
    fn set_orientation(&self, orientation: Orientation) {
        let boxlayout = self.as_ref();
        let mut props = boxlayout.props.borrow_mut();

        if props.orientation != orientation {
            props.orientation = orientation;
            // boxlayout.start_animation();
            // actor_queue_relayout (CLUTTER_ACTOR (box));

            // g_object_notify (G_OBJECT (box), "orientation");
        }
    }

    /// set_scroll_to_focused:
    /// @box: A #BoxLayout
    /// @scroll_to_focused: #true to enable automatically scrolling to the
    ///   focused actor
    ///
    /// Enables or disables automatic scrolling to the focused actor.
    ///
    fn set_scroll_to_focused(&self, scroll_to_focused: bool) {
        let boxlayout = self.as_ref();
        let mut props = boxlayout.props.borrow_mut();

        if props.scroll_to_focused != scroll_to_focused {
            props.scroll_to_focused = scroll_to_focused;
            // g_object_notify (G_OBJECT (box), "scroll-to-focused");
        }
    }

    /// set_spacing:
    /// @box: A #BoxLayout
    /// @spacing: the spacing value
    ///
    /// Set the amount of spacing between children in pixels
    ///
    fn set_spacing(&self, spacing: u32) {
        let boxlayout = self.as_ref();
        let mut props = boxlayout.props.borrow_mut();

        if props.spacing != spacing {
            props.spacing = spacing;
            props.ignore_css_spacing = true;
            // actor_queue_relayout (CLUTTER_ACTOR (box));
            // g_object_notify (G_OBJECT (box), "spacing");
        }
    }

    fn connect_property_enable_animations_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_enable_animations_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayout,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayout>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::enable-animations\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_enable_animations_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayout,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayout>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_scroll_to_focused_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_scroll_to_focused_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayout,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayout>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scroll-to-focused\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scroll_to_focused_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::BoxLayout,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<BoxLayout>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::spacing\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_spacing_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxLayout")
    }
}
