#![allow(unused_variables)]

// Settings overlap
use crate::prelude::*;
use crate::{Actor, ActorBox, BorderImage, Event, HandlerId, Padding, Style};
use std::{cell::RefCell, fmt};

// should be located in concrete widget
// tooltip: Option<Tooltip>,

// should be located in concrete widget
// menu: Option<Menu>,

// should be located in concrete widget
// tooltip_timeout: u32,

// should be located in concrete widget
// tooltip_delay: u32,

#[derive(Debug)]
pub struct WidgetProps {
    border: Padding,
    padding: Padding,
    style: Style,
    pseudo_class: String,
    style_class: String,
    ux_border_image: BorderImage,
    ux_background_image: BorderImage,

    border_image: Option<dx::core::Handle>,
    old_border_image: Option<dx::core::Handle>,
    background_image: Option<dx::core::Handle>,
    background_image_box: Option<ActorBox>,
    bg_color: Option<Color>,
    opacity: f64,
    is_disabled: bool,
    parent_disabled: bool,
    long_press_source: u32,
    in_dispose: bool,
    // sequences: GHashTable,

    // width/height set by css
    css_width: f64,
    css_height: f64,

    // previous size state before css width/height were applied
    old_min_width: f64,
    old_min_height: f64,
    old_nat_width: f64,
    old_nat_height: f64,

    old_min_width_set: bool,
    old_min_height_set: bool,
    old_nat_width_set: bool,
    old_nat_height_set: bool,

    // the previous opacity value if "visibility" style property was set to "hidden"
    old_opacity: i64,
    // previous visible state if the "display" style property was set to "none"
    old_visible: bool,
}

#[derive(Debug)]
pub struct Widget {
    props: RefCell<WidgetProps>,
    inner: Actor,
}

impl Widget {
    pub fn new() -> Self {
        let props = WidgetProps {
            border: Default::default(),
            padding: Default::default(),
            style: Default::default(),
            pseudo_class: Default::default(),
            style_class: Default::default(),
            ux_border_image: Default::default(),
            ux_background_image: Default::default(),

            border_image: None,
            old_border_image: None,
            background_image: None,
            background_image_box: None,
            bg_color: None,
            opacity: 0.0,
            is_disabled: false,
            parent_disabled: false,
            long_press_source: 0,
            in_dispose: false,
            // sequences: GHashTable,

            // width/height set by css
            css_width: 0.0,
            css_height: 0.0,

            // previous size state before css width/height were applied
            old_min_width: 0.0,
            old_min_height: 0.0,
            old_nat_width: 0.0,
            old_nat_height: 0.0,

            old_min_width_set: false,
            old_min_height_set: false,
            old_nat_width_set: false,
            old_nat_height_set: false,
            old_opacity: 0,
            old_visible: false,
        };

        Self {
            props: RefCell::new(props),
            inner: Actor::new(),
        }
    }

    // fn remove_tooltip_timeout(&self) {
    //     //    self.tooltip_timeout = 0;
    // }
}

impl Default for Widget {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Widget {}
impl Is<Widget> for Widget {}

impl AsRef<Widget> for Widget {
    fn as_ref(&self) -> &Widget {
        self
    }
}

impl Is<Actor> for Widget {}

impl AsRef<Actor> for Widget {
    fn as_ref(&self) -> &Actor {
        &self.inner
    }
}

pub trait WidgetExt: 'static {
    /// widget_apply_style:
    /// @widget: A #Widget
    /// @style: A #Style
    ///
    /// Used to implement how a new style instance should be applied in the widget.
    /// For instance, setting style instance on stylable internal children.
    ///
    fn apply_style<P: Is<Style>>(&self, style: &P);

    /// get_available_area:
    /// @widget: A #Widget
    /// @allocation: A #ActorBox
    /// @area: A #ActorBox
    ///
    /// Copies @allocation into @area and accounts for the padding values. This
    /// gives the area that is available in which to allocate children with respect
    /// to padding.
    ///
    fn get_available_area(&self, allocation: &ActorBox, area: &mut ActorBox);

    /// get_background_color:
    /// @actor: A #Widget
    ///
    /// Get the color used as the background. This is set using the
    /// "background-color" CSS property. This function should normally only
    /// be used by subclasses.
    ///
    /// Returns: (transfer none): a #Color
    fn get_background_color(&self) -> Option<Color>;

    fn get_background_texture(&self) -> Option<dx::core::Handle>;

    /// get_disabled:
    /// @widget: an #Widget
    ///
    /// Get the value of the "disabled" property.
    ///
    fn get_disabled(&self) -> bool;

    // should be located in concrete widget
    // /// get_menu:
    // /// @widget: A #Widget
    // ///
    // /// Get the object in the #Widget:menu property.
    // ///
    // /// Returns: (transfer none): The current object in the "menu" property.
    // ///
    // fn get_menu(&self) -> Option<Menu>;

    /// get_padding:
    /// @widget: A #Widget
    /// @padding: (out): A pointer to an #Padding to fill
    ///
    /// Gets the padding of the widget, set using the "padding" CSS property. This
    /// function should normally only be used by subclasses.
    ///
    fn get_padding(&self) -> Padding;

    // should be located in concrete widget
    // /// get_tooltip_delay:
    // /// @widget: an #Widget
    // ///
    // /// Get the value of the "tooltip-delay" property.
    // ///
    // /// Returns: the current delay value in milliseconds
    // ///
    // fn get_tooltip_delay(&self) -> u32;

    // should be located in concrete widget
    // /// get_tooltip_text:
    // /// @widget: A #Widget
    // ///
    // /// Get the current tooltip string
    // ///
    // /// Returns: The current tooltip string, owned by the #Widget
    // ///
    // fn get_tooltip_text(&self) -> Option<String>;

    // should be located in concrete widget
    // /// hide_tooltip:
    // /// @widget: A #Widget
    // ///
    // /// Hide the tooltip for @widget
    // ///
    // fn hide_tooltip(&self);

    /// long_press_cancel:
    /// @widget: An Widget
    ///
    /// Cancel a long-press timeout if one is running and emit the signal to notify
    /// that the long-press has been cancelled.
    ///
    fn long_press_cancel(&self);

    /// long_press_query:
    /// @widget: An Widget
    /// @event: the event used to determine whether to run a long-press
    ///
    /// Emit the long-press query signal and start a long-press timeout if required.
    ///
    fn long_press_query(&self, event: &mut Event);

    /// widget_set_disabled:
    /// @widget: an #Widget
    /// @disabled: value to set
    ///
    /// Set the disabled property. Disabled widgets have a "disabled" pseudo-class
    /// until disabled is set to #false.
    ///
    fn set_disabled(&self, disabled: bool);

    // should be located in concrete widget
    // /// widget_set_menu:
    // /// @widget: A #Widget
    // /// @menu: A #Menu
    // ///
    // /// Set the value of the #Widget:menu property.
    // ///
    // fn set_menu<P: Is<Menu>>(&self, menu: &P);

    // should be located in concrete widget
    // /// set_tooltip_delay:
    // /// @widget: an #Widget
    // ///
    // /// Set the value, in milliseconds, of the "tooltip-delay" property.
    // /// This is initially set to WIDGET_TOOLTIP_TIMEOUT.
    // ///
    // fn set_tooltip_delay(&self, delay: u32);

    // should be located in concrete widget
    // /// set_tooltip_text:
    // /// @widget: A #Widget
    // /// @text: text to set as the tooltip
    // ///
    // /// Set the tooltip text of the widget. Note that setting tooltip text will cause
    // /// the widget to be set reactive. If you no longer need tooltips and you do not
    // /// need the widget to be reactive, you must set ClutterActor::reactive to
    // /// %false.
    // ///
    // fn set_tooltip_text(&self, text: &str);

    // should be located in concrete widget
    // /// show_tooltip:
    // /// @widget: A #Widget
    // ///
    // /// Show the tooltip for @widget
    // ///
    // fn show_tooltip(&self);

    //fn connect_long_press<Unsupported or ignored types>(&self, f: F) -> HandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_tooltip_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Widget>> WidgetExt for O {
    /// widget_apply_style:
    /// @widget: A #Widget
    /// @style: A #Style
    ///
    /// Used to implement how a new style instance should be applied in the widget.
    /// For instance, setting style instance on stylable internal children.
    ///
    fn apply_style<P: Is<Style>>(&self, style: &P) {
        // stylable_set_style (STYLABLE (widget), style);
    }

    /// get_available_area:
    /// @widget: A #Widget
    /// @allocation: A #ActorBox
    /// @area: A #ActorBox
    ///
    /// Copies @allocation into @area and accounts for the padding values. This
    /// gives the area that is available in which to allocate children with respect
    /// to padding.
    ///
    fn get_available_area(&self, allocation: &ActorBox, area: &mut ActorBox) {
        let widget = self.as_ref();
        let props = widget.props.borrow();

        let x1 = props.padding.left;
        let y1 = props.padding.top;

        let (width, height) = allocation.get_size();
        let x2 = f64::max(x1, width as f64 - props.padding.right);
        let y2 = f64::max(y1, height as f64 - props.padding.bottom);
        // TODO: put x1,y2,x2,y2 into area
    }

    /// get_background_color:
    /// @actor: A #Widget
    ///
    /// Get the color used as the background. This is set using the
    /// "background-color" CSS property. This function should normally only
    /// be used by subclasses.
    ///
    /// Returns: (transfer none): a #Color
    fn get_background_color(&self) -> Option<Color> {
        let widget = self.as_ref();
        let props = widget.props.borrow();

        props.bg_color
    }

    fn get_background_texture(&self) -> Option<dx::core::Handle> {
        // unsafe { TODO: call ffi:widget_get_background_texture() }
        unimplemented!()
    }

    /// get_disabled:
    /// @widget: an #Widget
    ///
    /// Get the value of the "disabled" property.
    ///
    fn get_disabled(&self) -> bool {
        let widget = self.as_ref();
        let props = widget.props.borrow();

        props.is_disabled || props.parent_disabled
    }

    // should be located in concrete widget
    // /// get_menu:
    // /// @widget: A #Widget
    // ///
    // /// Get the object in the #Widget:menu property.
    // ///
    // /// Returns: (transfer none): The current object in the "menu" property.
    // ///
    // fn get_menu(&self) -> Option<Menu> {
    //     let widget = self.as_ref();
    //     widget.menu.clone()
    // }

    /// get_padding:
    /// @widget: A #Widget
    /// @padding: (out): A pointer to an #Padding to fill
    ///
    /// Gets the padding of the widget, set using the "padding" CSS property. This
    /// function should normally only be used by subclasses.
    ///
    fn get_padding(&self) -> Padding {
        let widget = self.as_ref();
        let props = widget.props.borrow();

        props.padding
    }

    // should be located in concrete widget
    // /// get_tooltip_delay:
    // /// @widget: an #Widget
    // ///
    // /// Get the value of the "tooltip-delay" property.
    // ///
    // /// Returns: the current delay value in milliseconds
    // ///
    // fn get_tooltip_delay(&self) -> u32 {
    //     let widget = self.as_ref();
    //     widget.tooltip_delay
    // }

    // should be located in concrete widget
    // /// get_tooltip_text:
    // /// @widget: A #Widget
    // ///
    // /// Get the current tooltip string
    // ///
    // /// Returns: The current tooltip string, owned by the #Widget
    // ///
    // fn get_tooltip_text(&self) -> Option<String> {
    //     let widget = self.as_ref();
    //     match &widget.tooltip {
    //         Some(tooltip) => tooltip.get_text(),
    //         None => None,
    //     }
    // }

    // should be located in concrete widget
    // /// hide_tooltip:
    // /// @widget: A #Widget
    // ///
    // /// Hide the tooltip for @widget
    // ///
    // fn hide_tooltip(&self) {
    //     let widget = self.as_ref();
    //     widget.remove_tooltip_timeout();
    //     if let Some(tooltip) = &widget.tooltip {
    //         tooltip.hide();
    //     }
    // }

    /// long_press_cancel:
    /// @widget: An Widget
    ///
    /// Cancel a long-press timeout if one is running and emit the signal to notify
    /// that the long-press has been cancelled.
    ///
    fn long_press_cancel(&self) {
        let widget = self.as_ref();
        let mut props = widget.props.borrow_mut();

        if props.long_press_source != 0 {
            props.long_press_source = 0;
            // g_source_remove (widget.long_press_source);
            // TODO: emit signal LONG_PRESS_CANCEL
        }
    }

    /// long_press_query:
    /// @widget: An Widget
    /// @event: the event used to determine whether to run a long-press
    ///
    /// Emit the long-press query signal and start a long-press timeout if required.
    ///
    fn long_press_query(&self, event: &mut Event) {
        let widget = self.as_ref();
        // let query_result = false;
        // let settings = Settings::get_default();
        // let timeout: usize = 0;

        // // g_object_get (settings, "long-press-timeout", &timeout, None);
        // let event_type = event.get_event_type();
        // match event_type {
        //     ButtonPress => {
        //             // g_signal_emit (widget, widget_signals[LONG_PRESS], 0,
        //             //             event->button.x, event->button.y,
        //             //             LONG_PRESS_QUERY, &query_result);
        //     }

        //     TouchBegin => {
        //         // g_signal_emit (widget, widget_signals[LONG_PRESS], 0,
        //         //                 event->touch.x, event->touch.y,
        //         //                 LONG_PRESS_QUERY, &query_result);
        //     }

        //     _ => {
        //         g_assert_not_reached ();
        //     }
        // }

        // if query_result {
        //     // widget.long_press_source = g_timeout_add (timeout, (GSourceFunc) widget_emit_long_press, widget);
        // }
    }

    /// widget_set_disabled:
    /// @widget: an #Widget
    /// @disabled: value to set
    ///
    /// Set the disabled property. Disabled widgets have a "disabled" pseudo-class
    /// until disabled is set to #false.
    ///
    fn set_disabled(&self, disabled: bool) {
        let widget = self.as_ref();
        let mut props = widget.props.borrow_mut();

        if props.is_disabled != disabled {
            props.is_disabled = disabled;
            if disabled {
                // stylable_style_pseudo_class_add (STYLABLE (widget), "disabled");
            } else {
                // stylable_style_pseudo_class_remove (STYLABLE (widget), "disabled");
            }

            // Propagate the disabled state to our children, if necessary
            if !props.parent_disabled {
                // propogate_disabled((ClutterActor*)widget, disabled)
            }

            // when a widget is disabled, get_style_pseudo_class will always return "disabled"
            // clutter_actor_queue_relayout (CLUTTER_ACTOR (widget));

            // stylable_style_changed (STYLABLE (widget), 0);

            // g_object_notify_by_pspec (G_OBJECT (widget),
            //                             widget_properties[PROP_DISABLED]);
        }
    }

    // should be located in concrete widget
    // /// widget_set_menu:
    // /// @widget: A #Widget
    // /// @menu: A #Menu
    // ///
    // /// Set the value of the #Widget:menu property.
    // ///
    // fn set_menu<P: Is<Menu>>(&self, menu: &P) {
    //     let widget = self.as_ref();

    //     if let Some(menu) = &widget.menu {
    //         // clutter_actor_destroy (CLUTTER_ACTOR (menu));
    //         // widget.menu = None;
    //     }

    //     let menu = menu.as_ref();
    //     // TODO: menu should be option to remove menu
    //     {
    //         // widget.menu = menu;
    //         // clutter_actor_add_child (CLUTTER_ACTOR (widget), CLUTTER_ACTOR (menu));
    //     }

    //     // clutter_actor_queue_relayout (CLUTTER_ACTOR (widget));
    // }

    // should be located in concrete widget
    // /// set_tooltip_delay:
    // /// @widget: an #Widget
    // ///
    // /// Set the value, in milliseconds, of the "tooltip-delay" property.
    // /// This is initially set to WIDGET_TOOLTIP_TIMEOUT.
    // ///
    // fn set_tooltip_delay(&self, delay: u32) {
    //     let widget = self.as_ref();
    //     if widget.tooltip_delay != delay {
    //         // widget.tooltip_delay = delay;
    //         // g_object_notify_by_pspec (G_OBJECT (widget),
    //         //                     widget_properties[PROP_TOOLTIP_DELAY]);
    //     }
    // }

    // should be located in concrete widget
    // /// set_tooltip_text:
    // /// @widget: A #Widget
    // /// @text: text to set as the tooltip
    // ///
    // /// Set the tooltip text of the widget. Note that setting tooltip text will cause
    // /// the widget to be set reactive. If you no longer need tooltips and you do not
    // /// need the widget to be reactive, you must set Actor::reactive to
    // /// %false.
    // ///
    // fn set_tooltip_text(&self, text: &str) {
    //     let widget = self.as_ref();
    //     // let mut old_text: Option<String> = None;

    //     // if let Some(tooltip) = &widget.tooltip {
    //     //     old_text = tooltip.get_text();
    //     // }

    //     // Don't do anything if the text hasn't changed
    //     // if (text == old_text) ||
    //     //     (text && old_text && g_str_equal (text, old_text)) {
    //     //         return;
    //     // }

    //     // if text == None {
    //     //     widget.set_has_tooltip(false);
    //     // } else {
    //     //     widget.set_has_tooltip(true);
    //     // }

    //     if let Some(tooltip) = &widget.tooltip {
    //         tooltip.set_text(text);
    //     }

    //     // g_object_notify_by_pspec (G_OBJECT (widget),
    //     //                             widget_properties[PROP_TOOLTIP_TEXT]);
    // }

    // should be located in concrete widget
    // /// show_tooltip:
    // /// @widget: A #Widget
    // ///
    // /// Show the tooltip for @widget
    // ///
    // fn show_tooltip(&self) {
    //     let widget = self.as_ref();

    //     // Geometry area;
    //     // Vertex verts[4];

    //     /* Remove any timeout so we don't show the tooltip again */
    //     // widget.remove_tooltip_timeout();

    //     /* XXX not necceary, but first allocate transform is wrong */
    //     /* Work out the bounding box */
    //     // clutter_actor_get_abs_allocation_vertices ((ClutterActor*) widget,
    //     //                                             verts);

    //     // let mut x: f64;
    //     // let mut y: f64;
    //     // let mut x2: f64;
    //     // let mut y2: f64;

    //     // x = y = G_MAXFLOAT;
    //     // x2 = y2 = -G_MAXFLOAT;
    //     // for idx in 0..verts.len() {

    //     //     if verts[idx].x < x {
    //     //         x = verts[idx].x;
    //     //     }
    //     //     if verts[idx].x > x2 {
    //     //         x2 = verts[i].x;
    //     //     }
    //     //     if verts[idx].y < y {
    //     //         y = verts[idx].y;
    //     //     }

    //     //     if verts[idx].y > y2 {
    //     //         y2 = verts[idx].y;
    //     //     }
    //     // }

    //     // area.x = x;
    //     // area.y = y;
    //     // area.width = x2 - x;
    //     // area.height = y2 - y;

    //     // if let Some(tooltip) = &widget.tooltip {
    //     //     tooltip.set_tip_area(&area);
    //     //     tooltip.show();
    //     // }
    // }

    //fn connect_long_press<Unsupported or ignored types>(&self, f: F) -> HandlerId {
    //    Ignored p1: LongPressAction
    //}

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_disabled_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::disabled\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_disabled_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_menu_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::menu\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_menu_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_tooltip_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_tooltip_delay_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::tooltip-delay\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_tooltip_delay_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Widget")
    }
}
