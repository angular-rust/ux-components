#[macro_use]
mod rt;
// #[macro_use]
// mod event;

#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod auto;

// pub mod prelude;
// pub use self::auto::functions::*;
pub use auto::*;

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
// #[cfg(any(feature = "v3_22", feature = "dox"))]
// pub use event_pad_axis::EventPadAxis;
// #[cfg(any(feature = "v3_22", feature = "dox"))]
// pub use event_pad_button::EventPadButton;
// #[cfg(any(feature = "v3_22", feature = "dox"))]
// pub use event_pad_group_mode::EventPadGroupMode;
// pub use event_property::EventProperty;
// pub use event_proximity::EventProximity;
// pub use event_scroll::EventScroll;
// pub use event_selection::EventSelection;
// pub use event_setting::EventSetting;
// pub use event_touch::EventTouch;
// #[cfg(any(feature = "v3_18", feature = "dox"))]
// pub use event_touchpad_pinch::EventTouchpadPinch;
// #[cfg(any(feature = "v3_18", feature = "dox"))]
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
