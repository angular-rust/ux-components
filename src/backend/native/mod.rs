use crate::Color;

#[macro_use]
mod rt;

mod actor_manager;
pub use self::actor_manager::{ActorManager, ActorManagerExt};

mod adjustment;
pub use self::adjustment::{Adjustment, AdjustmentExt};

// mod box_layout;
// pub use self::box_layout::{BoxLayout, BoxLayoutExt};

mod box_layout_child;
pub use self::box_layout_child::{BoxLayoutChild, BoxLayoutChildExt};

mod button;
pub use self::button::{Button, ButtonExt};

mod button_group;
pub use self::button_group::{ButtonGroup, ButtonGroupExt};

mod clipboard;
pub use self::clipboard::{Clipboard, ClipboardExt};

mod combo_box;
pub use self::combo_box::{ComboBox, ComboBoxExt};

mod dialog;
pub use self::dialog::{Dialog, DialogExt};

mod entry;
pub use self::entry::{Entry, EntryExt};

mod expander;
pub use self::expander::{Expander, ExpanderExt};

mod fade_effect;
pub use self::fade_effect::{FadeEffect, FadeEffectExt};

mod floating_widget;
pub use self::floating_widget::FloatingWidget;

mod focus_manager;
pub use self::focus_manager::{FocusManager, FocusManagerExt};

mod focusable;
pub use self::focusable::Focusable;

mod frame;
pub use self::frame::Frame;

mod grid;
pub use self::grid::{Grid, GridExt};

mod icon;
pub use self::icon::{Icon, IconExt};

mod icon_theme;
pub use self::icon_theme::{IconTheme, IconThemeExt};

mod image;
pub use self::image::{Image, ImageExt};

mod item_view;
pub use self::item_view::{ItemView, ItemViewExt};

mod kinetic_scroll_view;
pub use self::kinetic_scroll_view::{KineticScrollView, KineticScrollViewExt};

mod label;
pub use self::label::{Label, LabelExt};

mod list_view;
pub use self::list_view::{ListView, ListViewExt};

mod menu;
pub use self::menu::{Menu, MenuExt};

mod notebook;
pub use self::notebook::{Notebook, NotebookExt};

mod pager;
pub use self::pager::{Pager, PagerExt};

mod path_bar;
pub use self::path_bar::{PathBar, PathBarExt};

mod progress_bar;
pub use self::progress_bar::{ProgressBar, ProgressBarExt};

mod push_action;
pub use self::push_action::{PushAction, PushActionExt};

mod scroll_bar;
pub use self::scroll_bar::{ScrollBar, ScrollBarExt};

mod scroll_view;
pub use self::scroll_view::{ScrollView, ScrollViewExt};

mod settings;
pub use self::settings::{Settings, SettingsExt};

mod slider;
pub use self::slider::{Slider, SliderExt};

mod spinner;
pub use self::spinner::{Spinner, SpinnerExt};

mod stack;
pub use self::stack::{Stack, StackExt};

mod stack_child;
pub use self::stack_child::{StackChild, StackChildExt};

mod style;
pub use self::style::{Style, StyleExt};

mod surface;
pub use self::surface::{Surface, SurfaceExt};

mod table;
pub use self::table::{Table, TableExt};

mod table_child;
pub use self::table_child::{TableChild, TableChildExt};

mod texture_cache;
pub use self::texture_cache::{TextureCache, TextureCacheExt};

mod toggle;
pub use self::toggle::{Toggle, ToggleExt};

mod toolbar;
pub use self::toolbar::{Toolbar, ToolbarExt};

mod tooltip;
pub use self::tooltip::{Tooltip, TooltipExt};

mod viewport;
pub use self::viewport::{Viewport, ViewportExt};

mod widget;
pub use self::widget::{Widget, WidgetExt};

mod window;
pub use self::window::{Window, WindowExt};

#[derive(Debug, Clone)]
pub struct ItemFactory;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KineticScrollViewState {
    Idle,
    Panning,
    Scrolling,
    Clamping,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutomaticScroll {
    None,
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusDirection {
    Out,
    Up,
    Down,
    Left,
    Right,
    Next,
    Previous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusHint {
    First,
    Last,
    Prior,
    FromAbove,
    FromBelow,
    FromLeft,
    FromRight,
}

/// BorderImage:
/// uri: uri of a supported image file
/// top: top border slice width
/// right: right border slice width
/// bottom: bottom border slice width
/// left: bottom border slice width
#[derive(Default, Debug, Clone)]
pub struct BorderImage {
    pub uri: String,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

// gboolean border_image_equal (BorderImage *b1, BorderImage *b2);

// void border_image_set_from_string (GValue *value,
//                                       const gchar *str,
//                                       const gchar *filename);

// void font_weight_set_from_string (GValue *value, const gchar *str);

/// Padding:
/// top: padding from the top
/// right: padding from the right
/// bottom: padding from the bottom
/// left: padding from the left
///
/// The padding from the internal border of the parent container.
#[derive(Default, Debug, Clone, Copy)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

/// TextShadow:
/// h_offset: horizontal shadow offset
/// v_offset: vertical shadow offset
/// blur: blur distance
/// color: shadow color
///
/// Properties of a text shadow
#[derive(Clone, Debug)]
pub struct TextShadow {
    h_offset: f64,
    v_offset: f64,
    blur: f64,
    color: Color,
}

/// Align:
/// Start: Align at the beginning of the axis
/// Middle: Align in the middle of the axis
/// End: Align at the end of the axis
///
/// Set the alignment of the item
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Start,
    Middle,
    End,
}

/// FontWeight:
/// Normal: Normal font weight
/// Bold: Bold font weight
/// Bolder: Bolder font weight
/// Lighter: Lighter font weight
///
/// Support values of font weight
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
}

/// ScrollPolicy:
/// None: Never scroll
/// Horizontal: Only allow horizontal scrolling
/// Vertical: Only allow vertical scrolling
/// Both: Allow scrolling both horizontally and vertically
/// Automatic: Automatically align scroll to horizontal
/// or vertical direction or both depending on the drag angle.
///
/// Defines the scrolling policy of scrollable widgets.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollPolicy {
    None,
    Horizontal,
    Vertical,
    Both,
    Automatic,
}

// /// Orientation:
// /// Horizontal: horizontal orientation
// /// Vertical: vertical orientation
// ///
// /// Defines the orientation of various layout widgets.
// ///
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Orientation {
//     Horizontal,
//     Vertical,
// }

/// WindowRotation:
/// Rotation0: Zero degrees of rotation
/// Rotation90: 90 degrees of rotation
/// Rotation180: 180 degrees of rotation
/// Rotation270: 270 degrees of rotation
///
/// Defines the clock-wise rotation angle of a window.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowRotation {
    Rotation0,
    Rotation90,
    Rotation180,
    Rotation270,
}

/// Position:
/// Top: The top position
/// Right: The right position
/// Bottom: The bottom position
/// Left: The left position
///
/// Defines the position of an interface element.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

/// ImageScaleMode:
/// None: Do not apply any scaling and center the image within
/// the allocation
/// Fit: Scale the image, but maintain the aspect ratio so that
/// it fits exactly within the allocation
/// Crop: Scale and crop the image so that it covers the entire
/// allocation while retaining the correct aspect ratio
///
/// Defines the scaling mode of an image.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageScaleMode {
    None,
    Fit,
    Crop,
}

/// VisibilityStyle:
/// Visible: The actor is visible
/// Hidden: The actor is invisible (but is still allocated space)
///
/// Values for the "visibility" style property.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisibilityStyle {
    Visible,
    Hidden,
}

/// DisplayStyle:
/// None: The actor is not displayed at all
/// Inline: The actor is displayed as normal
///
/// Values for the "display" style property.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayStyle {
    None,
    Inline,
}

/// TextAlign:
/// Left: align text to the left
/// Right: align text to the right
/// Center: center the text
/// Justify: justify the text
///
/// The horizontal alignment and layout of multiple lines of text.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

// #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
// #[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
// mod auto;

// pub mod prelude;
// pub use self::auto::functions::*;
// pub use auto::*;

// mod application;

// mod box_layout;
// mod button;
// mod combo_box;
// mod dialog;
// mod entry;
// mod expander;
// mod fade_effect;
// mod frame;
// mod grid;
// mod icon;
// mod image;
// mod item_view;
// mod kinetic_scroll_view;
// mod label;
// mod list_view;
// mod menu;
// mod notebook;
// mod path_bar;
// mod progress_bar;
// mod scroll_bar;
// mod scroll_view;
// mod slider;
// mod spinner;
// mod stack;
// mod table;
// mod toggle;
// mod toolbar;
// mod viewport;

// pub use gdk_sys::GdkColor as Color;

// pub use self::rt::{init, set_initialized};

// pub use atom::Atom;
// pub use atom::NONE as ATOM_NONE;
// pub use atom::SELECTION_CLIPBOARD;
// pub use atom::SELECTION_PRIMARY;
// pub use atom::SELECTION_SECONDARY;
// pub use atom::SELECTION_TYPE_ATOM;
// pub use atom::SELECTION_TYPE_BITMAP;
// pub use atom::SELECTION_TYPE_COLORMAP;
// pub use atom::SELECTION_TYPE_DRAWABLE;
// pub use atom::SELECTION_TYPE_INTEGER;
// pub use atom::SELECTION_TYPE_PIXMAP;
// pub use atom::SELECTION_TYPE_STRING;
// pub use atom::SELECTION_TYPE_WINDOW;
// pub use atom::TARGET_BITMAP;
// pub use atom::TARGET_COLORMAP;
// pub use atom::TARGET_DRAWABLE;
// pub use atom::TARGET_PIXMAP;
// pub use atom::TARGET_STRING;
// pub use change_data::ChangeData;
// pub use event::Event;
// pub use event_button::EventButton;
// pub use event_configure::EventConfigure;
// pub use event_crossing::EventCrossing;
// pub use event_dnd::EventDND;
// pub use event_expose::EventExpose;
// pub use event_focus::EventFocus;
// pub use event_grab_broken::EventGrabBroken;
// pub use event_key::EventKey;
// pub use event_motion::EventMotion;
// pub use event_owner_change::EventOwnerChange;
// pub use event_pad_axis::EventPadAxis;
// pub use event_pad_button::EventPadButton;
// pub use event_pad_group_mode::EventPadGroupMode;
// pub use event_property::EventProperty;
// pub use event_proximity::EventProximity;
// pub use event_scroll::EventScroll;
// pub use event_selection::EventSelection;
// pub use event_setting::EventSetting;
// pub use event_touch::EventTouch;
// pub use event_touchpad_pinch::EventTouchpadPinch;
// pub use event_touchpad_swipe::EventTouchpadSwipe;
// pub use event_visibility::EventVisibility;
// pub use event_window_state::EventWindowState;
// pub use functions::*;
// pub use geometry::Geometry;
// pub use keymap_key::KeymapKey;
// pub use rectangle::Rectangle;
// pub use rgba::{RgbaParseError, RGBA};
// pub use time_coord::TimeCoord;
// pub use window::WindowAttr;

// #[allow(non_camel_case_types)]
// pub type key = i32;

// /// The primary button. This is typically the left mouse button, or the right button in a left-handed setup.
// pub const BUTTON_PRIMARY: u32 = gdk_sys::GDK_BUTTON_PRIMARY as u32;

// /// The middle button.
// pub const BUTTON_MIDDLE: u32 = gdk_sys::GDK_BUTTON_MIDDLE as u32;

// /// The secondary button. This is typically the right mouse button, or the left button in a left-handed setup.
// pub const BUTTON_SECONDARY: u32 = gdk_sys::GDK_BUTTON_SECONDARY as u32;

// // Used as the return value for stopping the propagation of an event handler.
// pub const EVENT_STOP: u32 = gdk_sys::GDK_EVENT_STOP as u32;

// // Used as the return value for continuing the propagation of an event handler.
// pub const EVENT_PROPAGATE: u32 = gdk_sys::GDK_EVENT_PROPAGATE as u32;
