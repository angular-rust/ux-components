#![allow(unused_variables)]

use crate::prelude::*;
use crate::{
    Actor, AutomaticScroll, Event, EventSequence, Geometry, InputDevice, KineticScrollViewState,
    ScrollPolicy, Timeline, Widget,
};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct KineticScrollViewMotion {
    // Units to store the origin of a click when scrolling
    pub x: f32,
    pub y: f32,
    pub time: u64, // GTimeVal,
}

#[derive(Clone, Debug)]
pub struct KineticScrollViewProps {
    pub child: Option<Actor>,
    pub use_captured: bool,
    pub use_grab: bool,
    pub in_drag: bool,
    pub hmoving: bool,
    pub vmoving: bool,
    pub align_tested: bool,
    pub hclamping: bool,
    pub vclamping: bool,
    pub clamp_to_center: bool,
    pub snap_on_page: bool,
    pub button: u32,
    pub device: Option<InputDevice>,
    pub sequence: Option<EventSequence>,
    pub source_press_actor: Option<Actor>,
    pub cancel_event: Option<Event>,
    pub in_automatic_scroll: AutomaticScroll,

    // Mouse motion event information
    // pub motion_buffer: Vec<Event>,
    pub last_motion: u32,

    // Variables for storing acceleration information
    pub deceleration_timeline: Option<Timeline>,
    pub dx: f32,
    pub dy: f32,
    pub decel_rate: f64,
    pub overshoot: f64,
    pub accumulated_delta: f64,
    pub acceleration_factor: f64,
    pub scroll_policy: ScrollPolicy,
    pub clamp_duration: u32,
    pub clamp_mode: u64,
    pub state: KineticScrollViewState,
}

#[derive(Clone, Debug)]
pub struct KineticScrollView {
    props: RefCell<KineticScrollViewProps>,
    widget: Widget,
}

impl KineticScrollView {
    pub fn new() -> KineticScrollView {
        // assert_initialized_main_thread!();
        // unsafe {
        //     Actor::from_glib_none(ffi::kinetic_scroll_view_new()).unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Default for KineticScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for KineticScrollView {}
impl Is<KineticScrollView> for KineticScrollView {}

impl AsRef<KineticScrollView> for KineticScrollView {
    fn as_ref(&self) -> &KineticScrollView {
        self
    }
}

impl Is<Widget> for KineticScrollView {}

impl AsRef<Widget> for KineticScrollView {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<Actor> for KineticScrollView {}

impl AsRef<Actor> for KineticScrollView {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait KineticScrollViewExt: 'static {
    /// ensure_visible:
    /// @scroll: A #KineticScrollView
    /// @geometry: The region to make visible
    ///
    /// Ensures that a given region is visible in the ScrollView, with the top-left
    /// taking precedence.
    ///
    fn ensure_visible(&self, geometry: &Geometry);

    /// get_acceleration_factor:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the initial acceleration factor of the kinetic scroll-view.
    ///
    /// Returns: The initial acceleration factor of the kinetic scroll-view
    ///
    fn get_acceleration_factor(&self) -> f64;

    /// get_clamp_duration:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the duration of the adjustment clamp animation.
    ///
    /// Returns: Clamp duration
    ///
    fn get_clamp_duration(&self) -> u32;

    /// get_clamp_mode:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the animation mode to use for the adjustment clamp animation.
    ///
    /// Returns: Clamp mode
    ///
    fn get_clamp_mode(&self) -> u64;

    /// get_clamp_to_center:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves whether to clamp to step increments based on the center of the page.
    ///
    /// Returns: Clamp to center
    ///
    fn get_clamp_to_center(&self) -> bool;

    /// get_deceleration:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the deceleration rate of the kinetic scroll-view.
    ///
    /// Returns: The deceleration rate of the kinetic scroll-view
    ///
    fn get_deceleration(&self) -> f64;

    /// get_input:
    /// @scroll: A #KineticScrollView
    /// @device: (allow-none) (out) (transfer none): a pointer to a #InputDevice pointer
    /// @sequence: (allow-none) (out) (transfer none): a pointer to a #EventSequence pointer
    ///
    /// Retrieves informations about the current input device driving the
    /// scrolling.
    ///
    fn get_input(&self) -> (Option<InputDevice>, Option<EventSequence>);

    /// get_mouse_button:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:mouse-button property
    ///
    /// Returns: The mouse button number used to initiate drag events on the
    ///          kinetic scroll-view
    ///
    fn get_mouse_button(&self) -> u32;

    /// get_overshoot:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the deceleration rate multiplier used when the scroll-view is
    /// scrolling beyond its boundaries.
    ///
    fn get_overshoot(&self) -> f64;

    /// get_scroll_policy:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the scrolling policy of the kinetic scroll-view.
    ///
    fn get_scroll_policy(&self) -> ScrollPolicy;

    /// get_snap_on_page:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves whether animations end on step increments.
    ///
    /// Returns: #true if animations end on step increments, #false otherwise.
    ///
    fn get_snap_on_page(&self) -> bool;

    /// get_use_captured:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:use-captured property.
    ///
    /// Returns: %true if captured-events should be used to initiate scrolling
    ///
    fn get_use_captured(&self) -> bool;

    /// get_use_grab:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:use-grab property.
    ///
    /// Returns: %true if grab-events should be used to initiate scrolling
    ///
    fn get_use_grab(&self) -> bool;

    /// set_acceleration_factor:
    /// @scroll: A #KineticScrollView
    /// @acceleration_factor: The acceleration factor
    ///
    /// Factor applied to the initial momentum.
    ///
    fn set_acceleration_factor(&self, acceleration_factor: f64);

    /// set_clamp_duration:
    /// @scroll: A #KineticScrollView
    /// @clamp_duration: Clamp duration
    ///
    /// Duration of the adjustment clamp animation.
    ///
    fn set_clamp_duration(&self, clamp_duration: u32);

    /// set_clamp_mode:
    /// @scroll: A #KineticScrollView
    /// @clamp_mode: Clamp mode
    ///
    /// Animation mode to use for the adjustment clamp animation.
    ///
    fn set_clamp_mode(&self, clamp_mode: u64);

    /// set_clamp_to_center:
    /// @scroll: A #KineticScrollView
    /// @clamp_to_center: Clamp to center
    ///
    /// Set whether to clamp to step increments based on the center of the page.
    ///
    fn set_clamp_to_center(&self, clamp_to_center: bool);

    /// set_deceleration:
    /// @scroll: A #KineticScrollView
    /// @rate: The deceleration rate
    ///
    /// Sets the deceleration rate when a drag is finished on the kinetic
    /// scroll-view. This is the value that the momentum is divided by
    /// every 60th of a second.
    ///
    fn set_deceleration(&self, rate: f64);

    /// set_mouse_button:
    /// @scroll: A #KineticScrollView
    /// @button: A mouse button number
    ///
    /// Sets the mouse button number used to initiate drag events on the kinetic
    /// scroll-view.
    ///
    fn set_mouse_button(&self, button: u32);

    /// set_overshoot:
    /// @scroll: A #KineticScrollView
    /// @overshoot: The rate at which the view will decelerate when scrolling beyond
    ///             its boundaries.
    ///
    /// Sets the rate at which the view will decelerate when scrolling beyond its
    /// boundaries. The deceleration rate will be multiplied by this value every
    /// 60th of a second when the view is scrolling outside of the range set by its
    /// adjustments.
    ///
    /// See set_deceleration()
    ///
    fn set_overshoot(&self, overshoot: f64);

    /// set_scroll_policy:
    /// @scroll: A #KineticScrollView
    /// @policy: A #ScrollPolicy
    ///
    /// Sets the scrolling policy for the kinetic scroll-view. This controls the
    /// possible axes of movement, and can affect the minimum size of the widget.
    ///
    fn set_scroll_policy(&self, policy: ScrollPolicy);

    /// set_snap_on_page:
    /// @scroll: A #KineticScrollView
    /// @snap_on_page: #true to stop animations on step increments
    ///
    /// Set whether to stop animations on step increments.
    ///
    fn set_snap_on_page(&self, snap_on_page: bool);

    /// set_use_captured:
    /// @scroll: A #KineticScrollView
    /// @use_captured: %true to use captured events
    ///
    /// Sets whether to use captured events to initiate drag events. This can be
    /// used to block events that would initiate scrolling from reaching the child
    /// actor.
    ///
    fn set_use_captured(&self, use_captured: bool);

    /// set_use_grab:
    /// @scroll: A #KineticScrollView
    /// @use_grab: %true to use grab events
    ///
    /// Sets whether to use grab events to initiate drag events. This can be
    /// used to block events that would initiate scrolling from reaching the child
    /// actor.
    ///
    fn set_use_grab(&self, use_grab: bool);

    /// stop:
    /// @scroll: A #KineticScrollView
    ///
    /// Stops any current movement due to kinetic scrolling.
    ///
    fn stop(&self);

    fn get_property_snap_on_page(&self) -> bool;

    fn set_property_snap_on_page(&self, snap_on_page: bool);

    fn get_property_state(&self) -> KineticScrollViewState;

    fn get_property_use_grab(&self) -> bool;

    fn set_property_use_grab(&self, use_grab: bool);

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_clamp_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_clamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clamp_to_center_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_overshoot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scroll_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_snap_on_page_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_captured_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_use_grab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<KineticScrollView>> KineticScrollViewExt for O {
    /// ensure_visible:
    /// @scroll: A #KineticScrollView
    /// @geometry: The region to make visible
    ///
    /// Ensures that a given region is visible in the ScrollView, with the top-left
    /// taking precedence.
    ///
    fn ensure_visible(&self, geometry: &Geometry) {
        let scrollview = self.as_ref();
        // Adjustment *hadjustment, *vadjustment;

        // scrollable_get_adjustments(SCROLLABLE(scrollview.child),
        //                                &hadjustment,
        //                                &vadjustment);

        // _scroll_view_ensure_visible_axis (hadjustment,
        //                                      geometry.x,
        //                                      geometry.x + geometry.width);
        // _scroll_view_ensure_visible_axis (vadjustment,
        //                                      geometry.y,
        //                                      geometry.y + geometry.height);
    }

    /// get_acceleration_factor:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the initial acceleration factor of the kinetic scroll-view.
    ///
    /// Returns: The initial acceleration factor of the kinetic scroll-view
    ///
    fn get_acceleration_factor(&self) -> f64 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.acceleration_factor
    }

    /// get_clamp_duration:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the duration of the adjustment clamp animation.
    ///
    /// Returns: Clamp duration
    ///
    fn get_clamp_duration(&self) -> u32 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.clamp_duration
    }

    /// get_clamp_mode:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the animation mode to use for the adjustment clamp animation.
    ///
    /// Returns: Clamp mode
    ///
    fn get_clamp_mode(&self) -> u64 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.clamp_mode
    }

    /// get_clamp_to_center:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves whether to clamp to step increments based on the center of the page.
    ///
    /// Returns: Clamp to center
    ///
    fn get_clamp_to_center(&self) -> bool {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.clamp_to_center
    }

    /// get_deceleration:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the deceleration rate of the kinetic scroll-view.
    ///
    /// Returns: The deceleration rate of the kinetic scroll-view
    ///
    fn get_deceleration(&self) -> f64 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.decel_rate
    }

    /// get_input:
    /// @scroll: A #KineticScrollView
    /// @device: (allow-none) (out) (transfer none): a pointer to a #InputDevice pointer
    /// @sequence: (allow-none) (out) (transfer none): a pointer to a #EventSequence pointer
    ///
    /// Retrieves informations about the current input device driving the
    /// scrolling.
    ///
    fn get_input(&self) -> (Option<InputDevice>, Option<EventSequence>) {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        (props.device.clone(), props.sequence.clone())
    }

    /// get_mouse_button:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:mouse-button property
    ///
    /// Returns: The mouse button number used to initiate drag events on the
    ///          kinetic scroll-view
    ///
    fn get_mouse_button(&self) -> u32 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.button
    }

    /// get_overshoot:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the deceleration rate multiplier used when the scroll-view is
    /// scrolling beyond its boundaries.
    ///
    fn get_overshoot(&self) -> f64 {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.overshoot
    }

    /// get_scroll_policy:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves the scrolling policy of the kinetic scroll-view.
    ///
    fn get_scroll_policy(&self) -> ScrollPolicy {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.scroll_policy
    }

    /// get_snap_on_page:
    /// @scroll: A #KineticScrollView
    ///
    /// Retrieves whether animations end on step increments.
    ///
    /// Returns: #true if animations end on step increments, #false otherwise.
    ///
    fn get_snap_on_page(&self) -> bool {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.snap_on_page
    }

    /// get_use_captured:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:use-captured property.
    ///
    /// Returns: %true if captured-events should be used to initiate scrolling
    ///
    fn get_use_captured(&self) -> bool {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.use_captured
    }

    /// get_use_grab:
    /// @scroll: A #KineticScrollView
    ///
    /// Gets the #KineticScrollView:use-grab property.
    ///
    /// Returns: %true if grab-events should be used to initiate scrolling
    ///
    fn get_use_grab(&self) -> bool {
        let scrollview = self.as_ref();
        let props = scrollview.props.borrow();

        props.use_grab
    }

    /// set_acceleration_factor:
    /// @scroll: A #KineticScrollView
    /// @acceleration_factor: The acceleration factor
    ///
    /// Factor applied to the initial momentum.
    ///
    fn set_acceleration_factor(&self, acceleration_factor: f64) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.acceleration_factor != acceleration_factor {
            props.acceleration_factor = acceleration_factor;
            // g_object_notify(G_OBJECT(scroll), "acceleration-factor");
        }
    }

    /// set_clamp_duration:
    /// @scroll: A #KineticScrollView
    /// @clamp_duration: Clamp duration
    ///
    /// Duration of the adjustment clamp animation.
    ///
    fn set_clamp_duration(&self, clamp_duration: u32) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.clamp_duration != clamp_duration {
            props.clamp_duration = clamp_duration;
            // g_object_notify(G_OBJECT(scroll), "clamp-duration");
        }
    }

    /// set_clamp_mode:
    /// @scroll: A #KineticScrollView
    /// @clamp_mode: Clamp mode
    ///
    /// Animation mode to use for the adjustment clamp animation.
    ///
    fn set_clamp_mode(&self, clamp_mode: u64) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.clamp_mode != clamp_mode {
            props.clamp_mode = clamp_mode;
            // g_object_notify(G_OBJECT(scroll), "clamp-mode");
        }
    }

    /// set_clamp_to_center:
    /// @scroll: A #KineticScrollView
    /// @clamp_to_center: Clamp to center
    ///
    /// Set whether to clamp to step increments based on the center of the page.
    ///
    fn set_clamp_to_center(&self, clamp_to_center: bool) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.clamp_to_center != clamp_to_center {
            props.clamp_to_center = clamp_to_center;
            // g_object_notify(G_OBJECT(scroll), "clamp-to-center");
        }
    }

    /// set_deceleration:
    /// @scroll: A #KineticScrollView
    /// @rate: The deceleration rate
    ///
    /// Sets the deceleration rate when a drag is finished on the kinetic
    /// scroll-view. This is the value that the momentum is divided by
    /// every 60th of a second.
    ///
    fn set_deceleration(&self, rate: f64) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.decel_rate != rate {
            props.decel_rate = rate;
            // g_object_notify(G_OBJECT(scroll), "deceleration");
        }
    }

    /// set_mouse_button:
    /// @scroll: A #KineticScrollView
    /// @button: A mouse button number
    ///
    /// Sets the mouse button number used to initiate drag events on the kinetic
    /// scroll-view.
    ///
    fn set_mouse_button(&self, button: u32) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.button != button {
            props.button = button;
            // g_object_notify(G_OBJECT(scroll), "mouse-button");
        }
    }

    /// set_overshoot:
    /// @scroll: A #KineticScrollView
    /// @overshoot: The rate at which the view will decelerate when scrolling beyond
    ///             its boundaries.
    ///
    /// Sets the rate at which the view will decelerate when scrolling beyond its
    /// boundaries. The deceleration rate will be multiplied by this value every
    /// 60th of a second when the view is scrolling outside of the range set by its
    /// adjustments.
    ///
    /// See set_deceleration()
    ///
    fn set_overshoot(&self, overshoot: f64) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.overshoot != overshoot {
            props.overshoot = overshoot;
            // g_object_notify(G_OBJECT(scroll), "overshoot");
        }
    }

    /// set_scroll_policy:
    /// @scroll: A #KineticScrollView
    /// @policy: A #ScrollPolicy
    ///
    /// Sets the scrolling policy for the kinetic scroll-view. This controls the
    /// possible axes of movement, and can affect the minimum size of the widget.
    ///
    fn set_scroll_policy(&self, policy: ScrollPolicy) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.scroll_policy != policy {
            props.scroll_policy = policy;
            // g_object_notify(G_OBJECT(scroll), "scroll-policy");
        }
    }

    /// set_snap_on_page:
    /// @scroll: A #KineticScrollView
    /// @snap_on_page: #true to stop animations on step increments
    ///
    /// Set whether to stop animations on step increments.
    ///
    fn set_snap_on_page(&self, snap_on_page: bool) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.snap_on_page != snap_on_page {
            props.snap_on_page = snap_on_page;
            // g_object_notify(G_OBJECT(scroll), "snap-on-page");
        }
    }

    /// set_use_captured:
    /// @scroll: A #KineticScrollView
    /// @use_captured: %true to use captured events
    ///
    /// Sets whether to use captured events to initiate drag events. This can be
    /// used to block events that would initiate scrolling from reaching the child
    /// actor.
    ///
    fn set_use_captured(&self, use_captured: bool) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.use_captured != use_captured {
            props.use_captured = use_captured;

            // g_signal_handlers_disconnect_by_func(scroll, button_press_event_cb, scroll);

            if use_captured {
                // g_signal_connect(
                //     scroll,
                //     "captured-event",
                //     G_CALLBACK(button_press_event_cb),
                //     scroll,
                // );
            } else {
                // g_signal_connect(
                //     scroll,
                //     "button-press-event",
                //     G_CALLBACK(button_press_event_cb),
                //     scroll,
                // );
                // g_signal_connect(
                //     scroll,
                //     "touch-event",
                //     G_CALLBACK(button_press_event_cb),
                //     scroll,
                // );
            }

            // g_object_notify(G_OBJECT(scroll), "use-captured");
        }
    }

    /// set_use_grab:
    /// @scroll: A #KineticScrollView
    /// @use_grab: %true to use grab events
    ///
    /// Sets whether to use grab events to initiate drag events. This can be
    /// used to block events that would initiate scrolling from reaching the child
    /// actor.
    ///
    fn set_use_grab(&self, use_grab: bool) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.use_grab != use_grab {
            props.use_grab = use_grab;
            // g_object_notify(G_OBJECT(scroll), "use-grab");
        }
    }

    /// stop:
    /// @scroll: A #KineticScrollView
    ///
    /// Stops any current movement due to kinetic scrolling.
    ///
    fn stop(&self) {
        let scrollview = self.as_ref();
        let mut props = scrollview.props.borrow_mut();

        if props.deceleration_timeline.is_some() {
            //     timeline_stop(scrollview.deceleration_timeline);
            //     g_object_unref(scrollview.deceleration_timeline);
            props.deceleration_timeline = None;
        }
    }

    fn get_property_snap_on_page(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"snap-on-page\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `snap-on-page` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_snap_on_page(&self, snap_on_page: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"snap-on-page\0".as_ptr() as *const _,
        //         Value::from(&snap_on_page).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_state(&self) -> KineticScrollViewState {
        //    unsafe {
        //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
        //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"state\0".as_ptr() as *const _, value.to_glib_none_mut().0);
        //        value.get().expect("Return Value for property `state` getter").unwrap()
        //    }
        unimplemented!()
    }

    fn get_property_use_grab(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"use-grab\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `use-grab` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_use_grab(&self, use_grab: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"use-grab\0".as_ptr() as *const _,
        //         Value::from(&use_grab).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_acceleration_factor_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::acceleration-factor\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_acceleration_factor_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clamp_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clamp_duration_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clamp-duration\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clamp_duration_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clamp_mode_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clamp-mode\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clamp_mode_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clamp_to_center_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clamp_to_center_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clamp-to-center\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clamp_to_center_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_deceleration_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::deceleration\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_deceleration_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_mouse_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_mouse_button_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::mouse-button\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_mouse_button_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_overshoot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_overshoot_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::overshoot\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_overshoot_trampoline::<Self, F> as *const (),
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
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_snap_on_page_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_snap_on_page_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::snap-on-page\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_snap_on_page_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::state\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_state_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_use_captured_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_use_captured_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::use-captured\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_use_captured_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_use_grab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_use_grab_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::KineticScrollView,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<KineticScrollView>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&KineticScrollView::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::use-grab\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_use_grab_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for KineticScrollView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KineticScrollView")
    }
}
