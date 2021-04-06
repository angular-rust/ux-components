#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Adjustment {
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
    pub interpolation: clutter::Timeline,
    pub old_position: f64,
    pub new_position: f64,
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
        if self.lower != lower {
            // self.lower = lower;

            // adjustment_emit_changed (adjustment);

            // if !self.lower_source {
            //     self.lower_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_lower_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // /* Defer clamp until after construction. */
            // if !self.is_constructing && self.clamp_value {
            //     self.clamp_page(self.lower, self.upper);
            // }

            // return true;
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

        if self.upper != upper {
            // self.upper = upper;

            // adjustment_emit_changed (adjustment);

            // if !self.upper_source {
            //     self.upper_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_upper_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // // Defer clamp until after construction.
            // if !self.is_constructing && self.clamp_value {
            //     adjustment_clamp_page (adjustment, self.lower, self.upper);
            // }

            // return true;
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

        if self.step_increment != increment {
            // self.step_increment = increment;

            // adjustment_emit_changed (adjustment);

            // if !self.step_inc_source {
            //     self.step_inc_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_step_inc_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // return true;
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

        if self.page_size != page_size {
            // self.page_size = page_size;

            // adjustment_emit_changed (adjustment);

            // if !self.page_size_source {
            //     self.page_size_source =
            //     g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
            //                     (GSourceFunc)adjustment_page_size_notify_cb,
            //                     adjustment,
            //                     None);
            // }

            // // Well explicitely clamp after construction.
            // if !self.is_constructing && self.clamp_value {
            //         adjustment_clamp_page (adjustment, self.lower, self.upper);
            // }

            // return true;
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
        // if self.page_increment != increment {
        //     self.page_increment = increment;

        //     adjustment_emit_changed (adjustment);

        //     if !self.page_inc_source {
        //         self.page_inc_source =
        //         g_idle_add_full (CLUTTER_PRIORITY_REDRAW,
        //                         (GSourceFunc)adjustment_page_inc_notify_cb,
        //                         adjustment,
        //                         None);
        //     }

        //     return true;
        // }

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
        adjustment.clamp_value
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
        adjustment.elastic
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
        adjustment.lower
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
        adjustment.page_increment
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
        adjustment.page_size
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
        adjustment.step_increment
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
        adjustment.upper
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
        adjustment.value
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
        (
            adjustment.value,
            adjustment.lower,
            adjustment.upper,
            adjustment.step_increment,
            adjustment.page_increment,
            adjustment.page_size,
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

        // adjustment.old_position = adjustment.value;
        // adjustment.new_position = value;

        // if !adjustment.interpolation {
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

        // TODO: ...
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

        // if adjustment.interpolation {
        //     offset += adjustment.new_position;
        // } else {
        //     offset += adjustment.value;
        // }

        // adjustment.interpolate(offset, duration, mode);
    }

    /// set_clamp_value:
    /// @adjustment: A #Adjustment
    /// @clamp: a #gboolean
    ///
    /// Set the value of the #Adjustment:clamp-value property.
    ///
    fn set_clamp_value(&self, clamp: bool) {
        let adjustment = self.as_ref();
        // adjustment.clamp_value = clamp;
        // TODO: ...
    }

    /// set_elastic:
    /// @adjustment: A #Adjustment
    /// @elastic: A #gboolean
    ///
    /// Set the value of the #Adjustment:elastic property.
    ///
    fn set_elastic(&self, elastic: bool) {
        let adjustment = self.as_ref();
        // adjustment.elastic = elastic;
        // TODO: ...
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

        // // Defer clamp until after construction.
        // if !adjustment.is_constructing {
        //     if !adjustment.elastic && adjustment.clamp_value {
        //         value = CLAMP(
        //             value,
        //             adjustment.lower,
        //             MAX(adjustment.lower, adjustment.upper - adjustment.page_size),
        //         );
        //     }
        // }

        // if adjustment.value != value {
        //     stop_interpolation(adjustment);

        //     adjustment.value = value;

        //     g_object_notify(G_OBJECT(adjustment), "value");
        //     adjustment_emit_changed(adjustment);
        // }
        unimplemented!()
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
        // TODO: ...
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
