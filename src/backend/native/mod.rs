#[macro_use]
mod rt;

// #[macro_use]
// mod event;

mod action;
pub use self::action::{Action, ActionExt, NONE_ACTION};

mod actor_manager;
pub use self::actor_manager::{ActorManager, ActorManagerExt, NONE_ACTOR_MANAGER};

mod adjustment;
pub use self::adjustment::{Adjustment, AdjustmentExt, NONE_ADJUSTMENT};

mod box_layout;
pub use self::box_layout::{BoxLayout, BoxLayoutExt, NONE_BOX_LAYOUT};

mod box_layout_child;
pub use self::box_layout_child::{BoxLayoutChild, BoxLayoutChildExt, NONE_BOX_LAYOUT_CHILD};

mod button;
pub use self::button::{Button, ButtonExt, NONE_BUTTON};

mod button_group;
pub use self::button_group::{ButtonGroup, ButtonGroupExt, NONE_BUTTON_GROUP};

mod clipboard;
pub use self::clipboard::{Clipboard, ClipboardExt, NONE_CLIPBOARD};

mod combo_box;
pub use self::combo_box::{ComboBox, ComboBoxExt, NONE_COMBO_BOX};

mod dialog;
pub use self::dialog::{Dialog, DialogExt, NONE_DIALOG};

mod entry;
pub use self::entry::{Entry, EntryExt, NONE_ENTRY};

mod expander;
pub use self::expander::{Expander, ExpanderExt, NONE_EXPANDER};

mod fade_effect;
pub use self::fade_effect::{FadeEffect, FadeEffectExt, NONE_FADE_EFFECT};

mod floating_widget;
pub use self::floating_widget::{FloatingWidget, NONE_FLOATING_WIDGET};

mod focus_manager;
pub use self::focus_manager::{FocusManager, FocusManagerExt, NONE_FOCUS_MANAGER};

mod frame;
pub use self::frame::{Frame, NONE_FRAME};

mod grid;
pub use self::grid::{Grid, GridExt, NONE_GRID};

mod icon;
pub use self::icon::{Icon, IconExt, NONE_ICON};

mod icon_theme;
pub use self::icon_theme::{IconTheme, IconThemeExt, NONE_ICON_THEME};

mod image;
pub use self::image::{Image, ImageExt, NONE_IMAGE};

mod item_view;
pub use self::item_view::{ItemView, ItemViewExt, NONE_ITEM_VIEW};

mod kinetic_scroll_view;
pub use self::kinetic_scroll_view::{
    KineticScrollView, KineticScrollViewExt, NONE_KINETIC_SCROLL_VIEW,
};

mod label;
pub use self::label::{Label, LabelExt, NONE_LABEL};

mod list_view;
pub use self::list_view::{ListView, ListViewExt, NONE_LIST_VIEW};

mod menu;
pub use self::menu::{Menu, MenuExt, NONE_MENU};

mod notebook;
pub use self::notebook::{Notebook, NotebookExt, NONE_NOTEBOOK};

mod pager;
pub use self::pager::{Pager, PagerExt, NONE_PAGER};

mod path_bar;
pub use self::path_bar::{PathBar, PathBarExt, NONE_PATH_BAR};

mod progress_bar;
pub use self::progress_bar::{ProgressBar, ProgressBarExt, NONE_PROGRESS_BAR};

mod scroll_bar;
pub use self::scroll_bar::{ScrollBar, ScrollBarExt, NONE_SCROLL_BAR};

mod scroll_view;
pub use self::scroll_view::{ScrollView, ScrollViewExt, NONE_SCROLL_VIEW};

mod settings;
pub use self::settings::{Settings, SettingsExt, NONE_SETTINGS};

mod slider;
pub use self::slider::{Slider, SliderExt, NONE_SLIDER};

mod spinner;
pub use self::spinner::{Spinner, SpinnerExt, NONE_SPINNER};

mod stack;
pub use self::stack::{Stack, StackExt, NONE_STACK};

mod stack_child;
pub use self::stack_child::{StackChild, StackChildExt, NONE_STACK_CHILD};

mod style;
pub use self::style::{Style, StyleExt, NONE_STYLE};

mod table;
pub use self::table::{Table, TableExt, NONE_TABLE};

mod table_child;
pub use self::table_child::{TableChild, TableChildExt, NONE_TABLE_CHILD};

mod texture_cache;
pub use self::texture_cache::{TextureCache, TextureCacheExt, NONE_TEXTURE_CACHE};

mod toggle;
pub use self::toggle::{Toggle, ToggleExt, NONE_TOGGLE};

mod toolbar;
pub use self::toolbar::{Toolbar, ToolbarExt, NONE_TOOLBAR};

mod tooltip;
pub use self::tooltip::{Tooltip, TooltipExt, NONE_TOOLTIP};

mod viewport;
pub use self::viewport::{Viewport, ViewportExt, NONE_VIEWPORT};

mod actor;
pub use self::actor::{Actor, WidgetExt, NONE_WIDGET};

mod stage;
pub use self::stage::{Stage, WindowExt, NONE_WINDOW};


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
