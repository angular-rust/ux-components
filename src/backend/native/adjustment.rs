#![allow(unused_variables)]

// use std::mem;
// use std::mem::transmute;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::{borrow::BorrowMut, fmt};
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct AdjustmentProps {
    // Do not sanity-check values while constructing, not all properties may be set yet.
    pub is_constructing: bool,
    pub clamp_value: bool,
    pub elastic: bool,

    pub lower: f64,
    pub upper: f64,
    pub value: f64,
    pub step_increment: f64,
    pub page_increment: f64,
    pub page_size: f64,

    // For signal emission/notification
    pub lower_source: u32,
    pub upper_source: u32,
    pub value_source: u32,
    pub step_inc_source: u32,
    pub page_inc_source: u32,
    pub page_size_source: u32,
    pub changed_source: u32,

    // For interpolation
    pub old_position: f64,
    pub new_position: f64,
}

#[derive(Clone, Debug)]
pub struct Adjustment {
    props: RefCell<AdjustmentProps>,
    pub interpolation: Option<clutter::Timeline>,
}

impl Adjustment {
    pub fn new() -> Adjustment {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::adjustment_new()) }
        unimplemented!()
    }

    pub fn with_values(
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    ) -> Adjustment {
        // assert_initialized_main_thread!();
        // unsafe {
        //     from_glib_full(ffi::adjustment_new_with_values(
        //         value,
        //         lower,
        //         upper,
        //         step_increment,
        //         page_increment,
        //         page_size,
        //     ))
        // }
        unimplemented!()
    }

    /// set_lower:
    /// @adjustment: A #Adjustment
    /// @lower: A #gdouble
    ///
    /// Set the value of the #Adjustment:lower property.
    ///
    fn set_lower(&self, lower: f64) -> bool {
        let mut props = self.props.borrow_mut();

        if props.lower != lower {
            props.lower = lower;

            // adjustment_emit_changed (adjustment);

            // if !props.lower_source {
            //     props.lower_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_lower_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // /* Defer clamp until after construction. */
            // if !props.is_constructing && props.clamp_value {
            //     self.clamp_page(props.lower, props.upper);
            // }

            return true;
        }

        false
    }

    /// set_upper:
    /// @adjustment: A #Adjustment
    /// @upper: A #gdouble
    ///
    /// Set the value of the #Adjustment:upper property.
    ///
    fn set_upper(&self, upper: f64) -> bool {
        let adjustment = self.as_ref();
        let mut props = self.props.borrow_mut();

        if props.upper != upper {
            props.upper = upper;

            // adjustment_emit_changed (adjustment);

            // if !props.upper_source {
            //     props.upper_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_upper_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // // Defer clamp until after construction.
            // if !props.is_constructing && props.clamp_value {
            //     adjustment_clamp_page (adjustment, props.lower, props.upper);
            // }

            return true;
        }

        false
    }

    /// set_step_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:step-increment property.
    ///
    fn set_step_increment(&self, increment: f64) -> bool {
        let adjustment = self.as_ref();
        let mut props = self.props.borrow_mut();

        if props.step_increment != increment {
            props.step_increment = increment;

            // adjustment_emit_changed (adjustment);

            // if !props.step_inc_source {
            //     props.step_inc_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_step_inc_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            return true;
        }

        false
    }

    /// set_page_size:
    /// @adjustment: A #Adjustment
    /// @page_size: A #gdouble
    ///
    /// Set the #Adjustment:page-size property.
    ///
    fn set_page_size(&self, page_size: f64) -> bool {
        let adjustment = self.as_ref();
        let mut props = self.props.borrow_mut();

        if props.page_size != page_size {
            props.page_size = page_size;

            // adjustment_emit_changed (adjustment);

            // if !props.page_size_source {
            //     props.page_size_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_page_size_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // // Well explicitely clamp after construction.
            // if !props.is_constructing && props.clamp_value {
            //         adjustment_clamp_page (adjustment, props.lower, props.upper);
            // }

            return true;
        }

        false
    }

    /// set_page_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:page-increment property.
    ///
    fn set_page_increment(&self, increment: f64) -> bool {
        let mut props = self.props.borrow_mut();

        if props.page_increment != increment {
            props.page_increment = increment;

            // adjustment_emit_changed (adjustment);

            // if !props.page_inc_source {
            //     props.page_inc_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_page_inc_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            return true;
        }

        false
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Adjustment {}
impl Is<Adjustment> for Adjustment {}

impl AsRef<Adjustment> for Adjustment {
    fn as_ref(&self) -> &Adjustment {
        self
    }
}

pub const NONE_ADJUSTMENT: Option<&Adjustment> = None;

pub trait AdjustmentExt: 'static {
    /// get_clamp_value:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:clamp-value property.
    ///
    /// Returns: the current value of the "clamp-value" property.
    ///
    fn get_clamp_value(&self) -> bool;

    /// get_elastic:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:elastic property.
    ///
    /// Returns: the current value of the "elastic" property.
    ///
    fn get_elastic(&self) -> bool;

    /// get_lower:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:lower property.
    ///
    /// Returns: the current value of the "lower" property.
    ///
    fn get_lower(&self) -> f64;

    /// get_page_increment:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the Adjustment:page-increment property.
    ///
    /// Returns: the current value of the "page-increment" property.
    ///
    fn get_page_increment(&self) -> f64;

    /// get_page_size:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:page-size property.
    ///
    /// Returns: the current value of the "page-size" property.
    ///
    fn get_page_size(&self) -> f64;

    /// get_step_increment:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the Adjustment:step-increment property.
    ///
    /// Returns: the current value of the "step-increment" property.
    ///
    fn get_step_increment(&self) -> f64;

    /// get_upper:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:upper property.
    ///
    /// Returns: the current value of the "upper" property.
    ///
    fn get_upper(&self) -> f64;

    /// get_value:
    /// @adjustment: An #Adjustment
    ///
    /// Get the current value of the #Adjustment:value property
    ///
    /// Returns: the current value of the "value" property
    ///
    fn get_value(&self) -> f64;

    /// get_values:
    /// @adjustment: A #Adjustment
    /// @value: (out) (allow-none): A #gdouble
    /// @lower: (out) (allow-none): A #gdouble
    /// @upper: (out) (allow-none): A #gdouble
    /// @step_increment: (out) (allow-none): A #gdouble
    /// @page_increment: (out) (allow-none): A #gdouble
    /// @page_size: (out) (allow-none): A #gdouble
    ///
    /// Get the various properties of Adjustment.
    ///
    fn get_values(&self) -> (f64, f64, f64, f64, f64, f64);

    /// interpolate:
    /// @adjustment: A #Adjustment
    /// @value: A #gdouble
    /// @duration: duration in milliseconds
    /// @mode: A #ClutterAnimationMode
    ///
    /// Interpolate #Adjustment:value to the new value specified by @value, using
    /// the mode and duration given.
    ///
    fn interpolate(&self, value: f64, duration: u32, mode: u64);

    /// interpolate_relative:
    /// @adjustment: A #Adjustment
    /// @offset: A #gdouble
    /// @duration: duration in milliseconds
    /// @mode: A #ClutterAnimationMode
    ///
    /// Interpolate the value of #Adjustment:value to a new value calculated from
    /// @offset.
    ///
    fn interpolate_relative(&self, offset: f64, duration: u32, mode: u64);

    /// set_clamp_value:
    /// @adjustment: A #Adjustment
    /// @clamp: a #gboolean
    ///
    /// Set the value of the #Adjustment:clamp-value property.
    ///
    fn set_clamp_value(&self, clamp: bool);

    /// set_elastic:
    /// @adjustment: A #Adjustment
    /// @elastic: A #gboolean
    ///
    /// Set the value of the #Adjustment:elastic property.
    ///
    fn set_elastic(&self, elastic: bool);

    /// set_lower:
    /// @adjustment: A #Adjustment
    /// @lower: A #gdouble
    ///
    /// Set the value of the #Adjustment:lower property.
    ///
    fn set_lower(&self, lower: f64);

    /// set_page_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:page-increment property.
    ///
    fn set_page_increment(&self, increment: f64);

    /// set_page_size:
    /// @adjustment: A #Adjustment
    /// @page_size: A #gdouble
    ///
    /// Set the #Adjustment:page-size property.
    ///
    fn set_page_size(&self, page_size: f64);

    /// set_step_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:step-increment property.
    ///
    fn set_step_increment(&self, increment: f64);

    /// set_upper:
    /// @adjustment: A #Adjustment
    /// @upper: A #gdouble
    ///
    /// Set the value of the #Adjustment:upper property.
    ///
    fn set_upper(&self, upper: f64);

    /// set_value:
    /// @adjustment: An #Adjustment
    /// @value: A #gdouble
    ///
    /// Set the value of the #Adjustment:value property.
    ///
    fn set_value(&self, value: f64);

    /// set_values:
    /// @adjustment: A #Adjustment
    /// @value: A #gdouble
    /// @lower: A #gdouble
    /// @upper: A #gdouble
    /// @step_increment: A #gdouble
    /// @page_increment: A #gdouble
    /// @page_size: A #gdouble
    ///
    /// Set the various properties of Adjustment.
    ///
    fn set_values(
        &self,
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    );

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_changed_immediate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_interpolation_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clamp_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_elastic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Adjustment>> AdjustmentExt for O {
    /// get_clamp_value:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:clamp-value property.
    ///
    /// Returns: the current value of the "clamp-value" property.
    ///
    fn get_clamp_value(&self) -> bool {
        let adjustment = self.as_ref();
        adjustment.props.borrow().clamp_value
    }

    /// get_elastic:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:elastic property.
    ///
    /// Returns: the current value of the "elastic" property.
    ///
    fn get_elastic(&self) -> bool {
        let adjustment = self.as_ref();
        adjustment.props.borrow().elastic
    }

    /// get_lower:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:lower property.
    ///
    /// Returns: the current value of the "lower" property.
    ///
    fn get_lower(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().lower
    }

    /// get_page_increment:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the Adjustment:page-increment property.
    ///
    /// Returns: the current value of the "page-increment" property.
    ///
    fn get_page_increment(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().page_increment
    }

    /// get_page_size:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:page-size property.
    ///
    /// Returns: the current value of the "page-size" property.
    ///
    fn get_page_size(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().page_size
    }

    /// get_step_increment:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the Adjustment:step-increment property.
    ///
    /// Returns: the current value of the "step-increment" property.
    ///
    fn get_step_increment(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().step_increment
    }

    /// get_upper:
    /// @adjustment: A #Adjustment
    ///
    /// Get the value of the #Adjustment:upper property.
    ///
    /// Returns: the current value of the "upper" property.
    ///
    fn get_upper(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().upper
    }

    /// get_value:
    /// @adjustment: An #Adjustment
    ///
    /// Get the current value of the #Adjustment:value property
    ///
    /// Returns: the current value of the "value" property
    ///
    fn get_value(&self) -> f64 {
        let adjustment = self.as_ref();
        adjustment.props.borrow().value
    }

    /// get_values:
    /// @adjustment: A #Adjustment
    /// @value: (out) (allow-none): A #gdouble
    /// @lower: (out) (allow-none): A #gdouble
    /// @upper: (out) (allow-none): A #gdouble
    /// @step_increment: (out) (allow-none): A #gdouble
    /// @page_increment: (out) (allow-none): A #gdouble
    /// @page_size: (out) (allow-none): A #gdouble
    ///
    /// Get the various properties of Adjustment.
    ///
    fn get_values(&self) -> (f64, f64, f64, f64, f64, f64) {
        let adjustment = self.as_ref();
        let props = adjustment.props.borrow();
        (
            props.value,
            props.lower,
            props.upper,
            props.step_increment,
            props.page_increment,
            props.page_size,
        )
    }

    /// interpolate:
    /// @adjustment: A #Adjustment
    /// @value: A #gdouble
    /// @duration: duration in milliseconds
    /// @mode: A #ClutterAnimationMode
    ///
    /// Interpolate #Adjustment:value to the new value specified by @value, using
    /// the mode and duration given.
    ///
    fn interpolate(&self, value: f64, duration: u32, mode: u64) {
        let adjustment = self.as_ref();

        // g_return_if_fail (isfinite (value));

        if duration <= 1 {
            // stop_interpolation (adjustment);
            adjustment.set_value(value);
            return;
        }

        let mut props = adjustment.props.borrow_mut();
        props.old_position = props.value;
        props.new_position = value;

        // if !props.interpolation {
        //     adjustment.interpolation = clutter_timeline_new(duration);

        //     g_signal_connect(
        //         adjustment.interpolation,
        //         "new-frame",
        //         G_CALLBACK(interpolation_new_frame_cb),
        //         adjustment,
        //     );
        //     g_signal_connect(
        //         adjustment.interpolation,
        //         "completed",
        //         G_CALLBACK(interpolation_completed_cb),
        //         adjustment,
        //     );
        // } else {
        //     // Extend the animation if it gets interrupted, otherwise frequent calls
        //     // to this function will end up with no advancements until the calls
        //     // finish (as the animation never gets a chance to start).
        //     clutter_timeline_set_direction(adjustment.interpolation, CLUTTER_TIMELINE_FORWARD);
        //     clutter_timeline_rewind(adjustment.interpolation);
        //     clutter_timeline_set_duration(adjustment.interpolation, duration);
        // }
        // clutter_timeline_set_progress_mode(adjustment.interpolation, mode);
        // clutter_timeline_start(adjustment.interpolation);
    }

    /// interpolate_relative:
    /// @adjustment: A #Adjustment
    /// @offset: A #gdouble
    /// @duration: duration in milliseconds
    /// @mode: A #ClutterAnimationMode
    ///
    /// Interpolate the value of #Adjustment:value to a new value calculated from
    /// @offset.
    ///
    fn interpolate_relative(&self, offset: f64, duration: u32, mode: u64) {
        let adjustment = self.as_ref();
        let props = adjustment.props.borrow();
        let offset = if adjustment.interpolation.is_some() {
            offset + props.new_position
        } else {
            offset + props.value
        };

        self.interpolate(offset, duration, mode);
    }

    /// set_clamp_value:
    /// @adjustment: A #Adjustment
    /// @clamp: a #gboolean
    ///
    /// Set the value of the #Adjustment:clamp-value property.
    ///
    fn set_clamp_value(&self, clamp: bool) {
        let adjustment = self.as_ref();
        let mut props = adjustment.props.borrow_mut();
        props.clamp_value = clamp;
    }

    /// set_elastic:
    /// @adjustment: A #Adjustment
    /// @elastic: A #gboolean
    ///
    /// Set the value of the #Adjustment:elastic property.
    ///
    fn set_elastic(&self, elastic: bool) {
        let adjustment = self.as_ref();
        let mut props = adjustment.props.borrow_mut();
        props.elastic = elastic;
    }

    /// set_lower:
    /// @adjustment: A #Adjustment
    /// @lower: A #gdouble
    ///
    /// Set the value of the #Adjustment:lower property.
    ///
    fn set_lower(&self, lower: f64) {
        let adjustment = self.as_ref();
        Adjustment::set_lower(adjustment, lower);
    }

    /// set_page_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:page-increment property.
    ///
    fn set_page_increment(&self, increment: f64) {
        let adjustment = self.as_ref();
        Adjustment::set_page_increment(adjustment, increment);
    }

    /// set_page_size:
    /// @adjustment: A #Adjustment
    /// @page_size: A #gdouble
    ///
    /// Set the #Adjustment:page-size property.
    ///
    fn set_page_size(&self, page_size: f64) {
        let adjustment = self.as_ref();
        Adjustment::set_page_size(adjustment, page_size);
    }

    /// set_step_increment:
    /// @adjustment: A #Adjustment
    /// @increment: A #gdouble
    ///
    /// Set the value of the #Adjustment:step-increment property.
    ///
    fn set_step_increment(&self, increment: f64) {
        let adjustment = self.as_ref();
        Adjustment::set_step_increment(adjustment, increment);
    }

    /// set_upper:
    /// @adjustment: A #Adjustment
    /// @upper: A #gdouble
    ///
    /// Set the value of the #Adjustment:upper property.
    ///
    fn set_upper(&self, upper: f64) {
        let adjustment = self.as_ref();
        Adjustment::set_upper(adjustment, upper);
    }

    /// set_value:
    /// @adjustment: An #Adjustment
    /// @value: A #gdouble
    ///
    /// Set the value of the #Adjustment:value property.
    ///
    fn set_value(&self, value: f64) {
        let adjustment = self.as_ref();
        let mut props = adjustment.props.borrow_mut();
        
        // Defer clamp until after construction.
        if !props.is_constructing {
            if !props.elastic && props.clamp_value {
                // value = CLAMP(
                //     value,
                //     adjustment.lower,
                //     MAX(adjustment.lower, adjustment.upper - adjustment.page_size),
                // );
            }
        }

        if props.value != value {
            // stop_interpolation(adjustment);
            props.value = value;
            // g_object_notify(G_OBJECT(adjustment), "value");
            // adjustment_emit_changed(adjustment);
        }
    }

    /// set_values:
    /// @adjustment: A #Adjustment
    /// @value: A #gdouble
    /// @lower: A #gdouble
    /// @upper: A #gdouble
    /// @step_increment: A #gdouble
    /// @page_increment: A #gdouble
    /// @page_size: A #gdouble
    ///
    /// Set the various properties of Adjustment.
    ///
    fn set_values(
        &self,
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    ) {
        // let adjustment = self.as_ref();

        // let emit_changed = false;

        // g_return_if_fail (page_size >= 0 && page_size <= G_MAXDOUBLE);
        // g_return_if_fail (step_increment >= 0 && step_increment <= G_MAXDOUBLE);
        // g_return_if_fail (page_increment >= 0 && page_increment <= G_MAXDOUBLE);

        // g_object_freeze_notify (G_OBJECT (adjustment));

        // emit_changed |= Adjustment::set_lower (adjustment, lower);
        // emit_changed |= Adjustment::set_upper (adjustment, upper);
        // emit_changed |= Adjustment::set_step_increment (adjustment,
        //                                                     step_increment);
        // emit_changed |= Adjustment::set_page_increment (adjustment,
        //                                                     page_increment);
        // emit_changed |= Adjustment::set_page_size (adjustment, page_size);

        // if value != adjustment.value {
        //     adjustment_set_value (adjustment, value);
        //     emit_changed = true;
        // }

        // if emit_changed {
        //     adjustment_emit_changed (adjustment);
        // }

        // g_object_thaw_notify(G_OBJECT (adjustment));
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_changed_immediate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn changed_immediate_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"changed-immediate\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             changed_immediate_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_interpolation_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn interpolation_completed_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"interpolation-completed\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             interpolation_completed_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_clamp_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_clamp_value_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clamp-value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clamp_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_elastic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_elastic_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::elastic\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_elastic_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_lower_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::lower\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_lower_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_increment_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-increment\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_increment_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_page_size_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::page-size\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_page_size_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_step_increment_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::step-increment\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_step_increment_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_upper_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::upper\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_upper_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Adjustment,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Adjustment>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::value\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_value_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Adjustment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adjustment")
    }
}
