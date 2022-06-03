#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::prelude::*;
use crate::{Actor, Align, Effect, HandlerId, StyleClass, Text, Theme, Timeline, Widget};
use std::{cell::RefCell, fmt};

#[derive(Default, Debug, Clone)]
pub struct LabelProps {
    pub text: Option<String>,
    pub fade_effect: Option<Effect>,
    pub x_align: Align,
    pub y_align: Align,
    pub fade_timeline: Option<Timeline>,
    pub em_width: i32,
    pub fade_out: bool,
    pub label_should_fade: bool,
    pub show_tooltip: bool,
    pub label: Option<Text>,
}

#[derive(Debug)]
pub struct Label {
    props: RefCell<LabelProps>,
    inner: Widget,
}

impl Label {
    pub fn new() -> Label {
        let props = LabelProps::default();

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    pub fn with_text(text: &str) -> Label {
        let mut props = LabelProps::default();

        if !text.is_empty() {
            props.text = Some(text.into())
        }

        let component = Self {
            props: RefCell::new(props),
            inner: Widget::new(),
        };

        component.init();
        component
    }

    fn init(&self) {
        let mut props = self.props.borrow_mut();
        if let Some(text) = &props.text {
            let style = Theme::global().get(StyleClass::MdcButton).unwrap();
            let mut fontfamily = if let Some(fontfamily) = style.fontfamily {
                fontfamily
            } else {
                "Roboto".into()
            };

            // Sets the font used by a ClutterText.
            //The font_name string must either be NULL,
            // which means that the font name from the default ClutterBackend will be used;
            // or be something that can be parsed by the
            // pango_font_description_from_string() function, like:
            // "Sans 10"
            // "Serif 16px"
            // "Helvetica 10"
            // "sans bold 12"
            // "serif,monospace bold italic condensed 16"
            // "normal 10"

            // The format of the string representation is:
            // "[FAMILY-LIST] [STYLE-OPTIONS] [SIZE]"

            // The commonly available font families are: Normal, Sans, Serif and Monospace. The available styles are:

            // Normal | the font is upright.
            // Oblique | the font is slanted, but in a roman style.
            // Italic | the font is slanted in an italic style.

            // The available weights are:

            // Ultra-Light | the ultralight weight (= 200)
            // Light |	the light weight (=300)
            // Normal | the default weight (= 400)
            // Bold | the bold weight (= 700)
            // Ultra-Bold | the ultra-bold weight (= 800)
            // Heavy | the heavy weight (= 900)

            // The available variants are:

            // - Normal
            // - Small-Caps

            // The available stretch styles are:

            // Ultra-Condensed | the smallest width
            // Extra-Condensed |
            // Condensed |
            // Semi-Condensed |
            // Normal | the normal width
            // Semi-Expanded |
            // Expanded |
            // Extra-Expanded |
            // Ultra-Expanded | the widest width

            fontfamily.push_str(" Normal 14px");
            println!("ADD TEXT TO LABEL [{}] [{}]", text, fontfamily);

            let label = Text::with_text(Some(fontfamily.as_str()), text.as_str());
            label.set_color(color::WHITE);
            // self.inner.add_child(&label);

            // props.label = Some(label);
            unimplemented!()
        }
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Label {}
impl Is<Label> for Label {}

impl AsRef<Label> for Label {
    fn as_ref(&self) -> &Label {
        self
    }
}

impl Is<Widget> for Label {}

impl AsRef<Widget> for Label {
    fn as_ref(&self) -> &Widget {
        &self.inner
    }
}

impl Is<Actor> for Label {}

impl AsRef<Actor> for Label {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.inner.as_ref();
        actor
    }
}

pub trait LabelExt: 'static {
    /// get_alignment:
    /// @label: An #Label
    /// @x_align: (out) (allow-none): return location for x alignment value
    /// @y_align: (out) (allow-none): return location for y alignment value
    ///
    /// Returns the text alignment on x and y axis.
    ///
    fn get_alignment(&self) -> (Align, Align);

    // /// get_clutter_text:
    // /// @label: a #Label
    // ///
    // /// Retrieve the internal #Text so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #Text used by #Label. The label
    // /// is owned by the #Label and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> Option<Actor>;

    /// get_fade_out:
    /// @label: A #Label
    ///
    /// Determines whether the label has been set to fade out when there isn't
    /// enough space allocated to display the entire label.
    ///
    /// Returns: %true if the label is set to fade out, %false otherwise
    ///
    fn get_fade_out(&self) -> bool;

    /// get_line_wrap:
    /// @label: An #Label
    ///
    /// Get the value of the #Label:line-wrap property.
    ///
    /// Returns: %true if the "line-wrap" property is set.
    ///
    fn get_line_wrap(&self) -> bool;

    /// get_show_tooltip:
    /// @label: A #Label
    ///
    /// Returns the current value of the #Label:show-tooltip property.
    ///
    /// Returns: %true if the #Label:show-tooltip property is enabled
    ///
    fn get_show_tooltip(&self) -> bool;

    /// get_text:
    /// @label: a #Label
    ///
    /// Get the text displayed on the label
    ///
    /// Returns: the text for the label. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String>;

    /// get_use_markup:
    /// @label: a #Label
    ///
    /// Determines whether the text of the label is being treated as Pango markup.
    ///
    /// Returns: %true if the text of the label is treated as Pango markup,
    ///   %false otherwise.
    ///
    fn get_use_markup(&self) -> bool;

    fn get_x_align(&self) -> Align;

    fn get_y_align(&self) -> Align;

    /// set_alignment:
    /// @label: An #Label
    /// @x_align: x alignment value
    /// @y_align: y alignment value
    ///
    /// Set the text alignment on x and y axis.
    ///
    fn set_alignment(&self, x_align: Align, y_align: Align);

    /// set_fade_out:
    /// @label: A #Label
    /// @fade: %true to fade out, %false otherwise
    ///
    /// Set whether to fade out the end of the label, instead of ellipsizing.
    /// Enabling this mode will also set the #Text:single-line-mode and
    /// #Text:ellipsize properties.
    ///
    fn set_fade_out(&self, fade: bool);

    /// set_line_wrap:
    /// @label: An #Label
    /// @line_wrap: new value of the line-wrap property.
    ///
    /// Set the value of the #Label:line-wrap property.
    ///
    fn set_line_wrap(&self, line_wrap: bool);

    /// set_show_tooltip:
    /// @label: A #Label
    /// @show_tooltip: %true if the tooltip should be shown
    ///
    /// Set the value of the #Label:show-tooltip property
    ///
    fn set_show_tooltip(&self, show_tooltip: bool);

    /// set_text:
    /// @label: a #Label
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the label
    ///
    fn set_text(&self, text: &str);

    /// set_use_markup:
    /// @label: a #Label
    /// @use_markup: %true to use Pango markup, %false otherwise
    ///
    /// Sets whether the text of the label should be treated as Pango markup.
    ///
    fn set_use_markup(&self, use_markup: bool);

    fn set_x_align(&self, align: Align);

    fn set_y_align(&self, align: Align);

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_fade_out_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_show_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<Label>> LabelExt for O {
    /// get_alignment:
    /// @label: An #Label
    /// @x_align: (out) (allow-none): return location for x alignment value
    /// @y_align: (out) (allow-none): return location for y alignment value
    ///
    /// Returns the text alignment on x and y axis.
    ///
    fn get_alignment(&self) -> (Align, Align) {
        let label = self.as_ref();
        let props = label.props.borrow();

        (props.x_align, props.y_align)
    }

    // /// get_clutter_text:
    // /// @label: a #Label
    // ///
    // /// Retrieve the internal #Text so that extra parameters can be set
    // ///
    // /// Returns: (transfer none): the #Text used by #Label. The label
    // /// is owned by the #Label and should not be unref'ed by the application.
    // ///
    // fn get_clutter_text(&self) -> Option<Actor> {
    //     let label = self.as_ref();
    //     let props = label.props.borrow();

    //     props.label.clone()
    // }

    /// get_fade_out:
    /// @label: A #Label
    ///
    /// Determines whether the label has been set to fade out when there isn't
    /// enough space allocated to display the entire label.
    ///
    /// Returns: %true if the label is set to fade out, %false otherwise
    ///
    fn get_fade_out(&self) -> bool {
        let label = self.as_ref();
        let props = label.props.borrow();

        props.fade_out
    }

    /// get_line_wrap:
    /// @label: An #Label
    ///
    /// Get the value of the #Label:line-wrap property.
    ///
    /// Returns: %true if the "line-wrap" property is set.
    ///
    fn get_line_wrap(&self) -> bool {
        let label = self.as_ref();

        // text_get_line_wrap(CLUTTER_TEXT (label.label));
        unimplemented!()
    }

    /// get_show_tooltip:
    /// @label: A #Label
    ///
    /// Returns the current value of the #Label:show-tooltip property.
    ///
    /// Returns: %true if the #Label:show-tooltip property is enabled
    ///
    fn get_show_tooltip(&self) -> bool {
        let label = self.as_ref();
        let props = label.props.borrow();

        props.show_tooltip
    }

    /// get_text:
    /// @label: a #Label
    ///
    /// Get the text displayed on the label
    ///
    /// Returns: the text for the label. This must not be freed by the application
    ///
    fn get_text(&self) -> Option<String> {
        let label = self.as_ref();

        // text_get_text(CLUTTER_TEXT(label.label));
        unimplemented!()
    }

    /// get_use_markup:
    /// @label: a #Label
    ///
    /// Determines whether the text of the label is being treated as Pango markup.
    ///
    /// Returns: %true if the text of the label is treated as Pango markup,
    ///   %false otherwise.
    ///
    fn get_use_markup(&self) -> bool {
        let label = self.as_ref();
        // text_get_use_markup(CLUTTER_TEXT(label.label));
        unimplemented!()
    }

    fn get_x_align(&self) -> Align {
        let label = self.as_ref();
        let props = label.props.borrow();

        props.x_align
    }

    fn get_y_align(&self) -> Align {
        let label = self.as_ref();
        let props = label.props.borrow();

        props.y_align
    }

    /// set_alignment:
    /// @label: An #Label
    /// @x_align: x alignment value
    /// @y_align: y alignment value
    ///
    /// Set the text alignment on x and y axis.
    ///
    fn set_alignment(&self, x_align: Align, y_align: Align) {
        let label = self.as_ref();
        let mut props = label.props.borrow_mut();

        if x_align != props.x_align {
            props.x_align = x_align;
            // actor_queue_relayout(CLUTTER_ACTOR(label));
            // g_object_notify(G_OBJECT(label), "x-align");
        }

        if y_align != props.y_align {
            props.y_align = y_align;
            // actor_queue_relayout(CLUTTER_ACTOR(label));
            // g_object_notify(G_OBJECT(label), "y-align");
        }
    }

    /// set_fade_out:
    /// @label: A #Label
    /// @fade: %true to fade out, %false otherwise
    ///
    /// Set whether to fade out the end of the label, instead of ellipsizing.
    /// Enabling this mode will also set the #Text:single-line-mode and
    /// #Text:ellipsize properties.
    ///
    fn set_fade_out(&self, fade: bool) {
        let label = self.as_ref();
        let mut props = label.props.borrow_mut();

        if props.fade_out != fade {
            props.fade_out = fade;
            // g_object_notify(G_OBJECT (label), "fade-out");

            // // Enable the fade-effect
            if fade {
                props.label_should_fade = false;
                // text_set_single_line_mode(CLUTTER_TEXT(label.label), true);
                // text_set_ellipsize(CLUTTER_TEXT(label.label),
                //                             PANGO_ELLIPSIZE_NONE);
            }

            // If we need to fade, listen for the font-description changing so
            // we can keep track of the em-width of the label.
            if fade {
                // g_signal_connect(label.label, "notify::font-description",
                //                     G_CALLBACK(label_font_description_cb), label);
                // label_font_description_cb(CLUTTER_TEXT(label.label),
                //                                 None, label);
            } else {
                // g_signal_handlers_disconnect_by_func(label.label,
                //                                     label_font_description_cb,
                //                                     label);
            }
        }
    }

    /// set_line_wrap:
    /// @label: An #Label
    /// @line_wrap: new value of the line-wrap property.
    ///
    /// Set the value of the #Label:line-wrap property.
    ///
    fn set_line_wrap(&self, line_wrap: bool) {
        let label = self.as_ref();

        // text_set_line_wrap(CLUTTER_TEXT(label.label), line_wrap);
        // g_object_notify(G_OBJECT(label), "line-wrap");
    }

    /// set_show_tooltip:
    /// @label: A #Label
    /// @show_tooltip: %true if the tooltip should be shown
    ///
    /// Set the value of the #Label:show-tooltip property
    ///
    fn set_show_tooltip(&self, show_tooltip: bool) {
        let label = self.as_ref();
        let mut props = label.props.borrow_mut();

        if props.show_tooltip != show_tooltip {
            props.show_tooltip = show_tooltip;
            // actor_queue_relayout(CLUTTER_ACTOR(label));
            // g_object_notify(G_OBJECT(label), "show-tooltip");
        }
    }

    /// set_text:
    /// @label: a #Label
    /// @text: text to set the label to
    ///
    /// Sets the text displayed on the label
    ///
    fn set_text(&self, text: &str) {
        let label = self.as_ref();

        // if text_get_use_markup(CLUTTER_TEXT(label.label)) {
        //     text_set_markup(CLUTTER_TEXT(label.label), (text) ? text : "");
        // } else {
        //     text_set_text(CLUTTER_TEXT(label.label), (text) ? text : "");
        // }

        // g_object_notify(G_OBJECT(label), "text");
    }

    /// set_use_markup:
    /// @label: a #Label
    /// @use_markup: %true to use Pango markup, %false otherwise
    ///
    /// Sets whether the text of the label should be treated as Pango markup.
    ///
    fn set_use_markup(&self, use_markup: bool) {
        let label = self.as_ref();

        // text_set_use_markup(CLUTTER_TEXT(label.label), use_markup);
        // g_object_notify(G_OBJECT(label), "use-markup");
    }

    fn set_x_align(&self, align: Align) {
        let label = self.as_ref();
        let mut props = label.props.borrow_mut();

        if align != props.x_align {
            props.x_align = align;
            // actor_queue_relayout(CLUTTER_ACTOR(label));
            // g_object_notify(G_OBJECT(label), "x-align");
        }
    }

    fn set_y_align(&self, align: Align) {
        let label = self.as_ref();
        let mut props = label.props.borrow_mut();

        if align != props.y_align {
            props.y_align = align;
            // actor_queue_relayout(CLUTTER_ACTOR(label));
            // g_object_notify(G_OBJECT(label), "y-align");
        }
    }

    fn connect_property_clutter_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_clutter_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::clutter-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_clutter_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fade_out_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_fade_out_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fade-out\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fade_out_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_line_wrap_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::line-wrap\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_line_wrap_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_show_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_show_tooltip_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::show-tooltip\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_show_tooltip_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::use-markup\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_use_markup_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_x_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_y_align_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Label,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Label>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Label::from_glib_borrow(this).unsafe_cast_ref())
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
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label")
    }
}
