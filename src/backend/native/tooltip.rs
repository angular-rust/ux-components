#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, ActorBox, BorderImage, FloatingWidget, Geometry, Widget};
use glib::signal::SignalHandlerId;
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct TooltipProps {
    pub label: Option<Actor>,
    pub arrow_box: Option<ActorBox>,
    pub arrow_offset: f32,
    pub actor_below: f32,
    pub tip_area: Option<Geometry>,
    pub stage_matrix: dx::Matrix,
    pub border_image: BorderImage,
    pub text_allocation: Option<ActorBox>,
    pub border_image_texture: Option<dx::Handle>,
}

#[derive(Clone, Debug)]
pub struct Tooltip {
    props: RefCell<TooltipProps>,
    widget: FloatingWidget,
}

impl Tooltip {
    /// is_in_browse_mode:
    ///
    /// Browse mode is entered whenever a tooltip is displayed and it is
    /// left after a short delay when a tooltip is hidden. This is used to
    /// make tooltips display quicker when a previous tooltip is already
    /// displayed.
    ///
    /// Returns: %true if the app is in tooltip browse mode or %false
    /// otherwise.
    ///
    pub fn is_in_browse_mode() -> bool {
        // assert_initialized_main_thread!();
        // unsafe { from_glib(ffi::tooltip_is_in_browse_mode()) }
        unimplemented!()
    }
}

impl Object for Tooltip {}
impl Is<Tooltip> for Tooltip {}

impl AsRef<Tooltip> for Tooltip {
    fn as_ref(&self) -> &Tooltip {
        self
    }
}

impl Is<FloatingWidget> for Tooltip {}

impl AsRef<FloatingWidget> for Tooltip {
    fn as_ref(&self) -> &FloatingWidget {
        &self.widget
    }
}

impl Is<Widget> for Tooltip {}

impl AsRef<Widget> for Tooltip {
    fn as_ref(&self) -> &Widget {
        let widget: &Widget = self.widget.as_ref();
        widget
    }
}

impl Is<Actor> for Tooltip {}

impl AsRef<Actor> for Tooltip {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait TooltipExt: 'static {
    /// get_text:
    /// @tooltip: a #Tooltip
    ///
    /// Get the text displayed on the tooltip
    ///
    /// Returns: the text for the tooltip. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String>;

    /// get_tip_area:
    /// @tooltip: A #Tooltip
    ///
    /// Retrieve the area on the stage that the tooltip currently applies to
    ///
    /// Returns: the #Geometry, owned by the tooltip which must not be freed
    /// by the application.
    ///
    fn get_tip_area(&self) -> Option<Geometry>;

    /// hide:
    /// @tooltip: a #Tooltip
    ///
    /// Hide the tooltip
    ///
    fn hide(&self);

    /// set_text:
    /// @tooltip: a #Tooltip
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the tooltip
    ///
    fn set_text(&self, text: &str);

    /// set_tip_area:
    /// @tooltip: A #Tooltip
    /// @area: A #Geometry
    ///
    /// Set the area on the stage that the tooltip applies to.
    ///
    fn set_tip_area(&self, area: &Geometry);

    /// set_tip_area_from_actor:
    /// @tooltip: A #Tooltip
    /// @actor: A #Actor
    ///
    /// Utility function to set the geometry of the tooltip area
    /// from an existing actor.
    /// See also set_tip_area
    ///
    fn set_tip_area_from_actor<P: Is<Actor>>(&self, actor: &P);

    /// show:
    /// @tooltip: a #Tooltip
    ///
    /// Show the tooltip relative to the associated widget.
    ///
    fn show(&self);

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tip_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Tooltip>> TooltipExt for O {
    /// get_text:
    /// @tooltip: a #Tooltip
    ///
    /// Get the text displayed on the tooltip
    ///
    /// Returns: the text for the tooltip. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String> {
        let tooltip = self.as_ref();
        // clutter_text_get_text(CLUTTER_TEXT(tooltip.label));
        unimplemented!()
    }

    /// get_tip_area:
    /// @tooltip: A #Tooltip
    ///
    /// Retrieve the area on the stage that the tooltip currently applies to
    ///
    /// Returns: the #Geometry, owned by the tooltip which must not be freed
    /// by the application.
    ///
    fn get_tip_area(&self) -> Option<Geometry> {
        let tooltip = self.as_ref();
        let props = tooltip.props.borrow();

        props.tip_area.clone()
    }

    /// hide:
    /// @tooltip: a #Tooltip
    ///
    /// Hide the tooltip
    ///
    fn hide(&self) {
        let tooltip = self.as_ref();

        // tooltip_set_opacity(tooltip, 0x0);

        // g_signal_connect(tooltip, "transition-stopped::opacity",
        //                     G_CALLBACK(tooltip_hide_complete), None);

        // // Leave browse mode after a short delay
        // if tooltip_browse_mode_timeout {
        //     g_source_remove (tooltip_browse_mode_timeout);
        // }
        // tooltip_browse_mode_timeout =
        //     g_timeout_add (TOOLTIP_BROWSE_MODE_TIMEOUT,
        //                 tooltip_browse_mode_timeout_cb,
        //                 None);
    }

    /// set_text:
    /// @tooltip: a #Tooltip
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the tooltip
    ///
    fn set_text(&self, text: &str) {
        let tooltip = self.as_ref();

        // clutter_text_set_text(CLUTTER_TEXT(tooltip.label), text);

        // if CLUTTER_ACTOR_IS_VISIBLE(tooltip) {
        //   tooltip_update_position(tooltip);
        // }

        // g_object_notify(G_OBJECT(tooltip), "text");
    }

    /// set_tip_area:
    /// @tooltip: A #Tooltip
    /// @area: A #Geometry
    ///
    /// Set the area on the stage that the tooltip applies to.
    ///
    fn set_tip_area(&self, area: &Geometry) {
        let tooltip = self.as_ref();
        let props = tooltip.props.borrow();

        if props.tip_area.is_some() {
            // g_boxed_free(CLUTTER_TYPE_GEOMETRY, tooltip.tip_area);
        }
        // tooltip.tip_area = g_boxed_copy(CLUTTER_TYPE_GEOMETRY, area);
    }

    /// set_tip_area_from_actor:
    /// @tooltip: A #Tooltip
    /// @actor: A #Actor
    ///
    /// Utility function to set the geometry of the tooltip area
    /// from an existing actor.
    /// See also set_tip_area
    ///
    fn set_tip_area_from_actor<P: Is<Actor>>(&self, actor: &P) {
        let tooltip = self.as_ref();
        let actor = actor.as_ref();

        // ClutterVertex verts[4];
        // ClutterGeometry area;
        // gfloat x, y, x2, y2;
        // gint i;

        // /* Work out the bounding box */

        // clutter_actor_get_abs_allocation_vertices (actor, verts);

        // let x: f32 = y = G_MAXFLOAT;
        // let x2: f32 = -G_MAXFLOAT;
        // let y2: f32 = -G_MAXFLOAT;

        // for idx in 0..G_N_ELEMENTS(verts) {
        //     if verts[idx].x < x {
        //         x = verts[idx].x;
        //     }
        //     if verts[idx].x > x2 {
        //         x2 = verts[idx].x;
        //     }
        //     if verts[idx].y < y {
        //         y = verts[idx].y;
        //     }
        //     if verts[idx].y > y2 {
        //         y2 = verts[idx].y;
        //     }
        // }

        // area.x = x;
        // area.y = y;
        // area.width = x2 - x;
        // area.height = y2 - y;

        // tooltip_set_tip_area(tooltip, &area);
    }

    /// show:
    /// @tooltip: a #Tooltip
    ///
    /// Show the tooltip relative to the associated widget.
    ///
    fn show(&self) {
        let tooltip = self.as_ref();

        // tooltip_update_position(tooltip);

        // // finally show the tooltip...
        // CLUTTER_ACTOR_CLASS(tooltip_parent_class)->show(CLUTTER_ACTOR (tooltip));

        // tooltip_set_opacity(tooltip, 0xff);

        // // Enter browse mode
        // tooltip_in_browse_mode = true;
        // // Disable any previous queued attempts to leave browse mode
        // if tooltip_browse_mode_timeout {
        //     g_source_remove(tooltip_browse_mode_timeout);
        //     tooltip_browse_mode_timeout = 0;
        // }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Tooltip,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Tooltip>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Tooltip::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_tip_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_tip_area_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Tooltip,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Tooltip>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Tooltip::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::tip-area\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_tip_area_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Tooltip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tooltip")
    }
}
