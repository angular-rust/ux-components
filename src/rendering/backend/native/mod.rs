use crate::Color;

#[macro_use]
mod rt;

mod advanced_widgets;
pub use advanced_widgets::*;

mod app_bar;
pub use self::app_bar::{AppBar, AppBarExt};

mod audio_widgets;
pub use audio_widgets::*;

mod backdrop;
pub use self::backdrop::{Backdrop, BackdropExt};

mod banner;
pub use self::banner::{Banner, BannerExt};

mod button;
pub use self::button::{Button, ButtonExt};

mod card;
pub use self::card::{Card, CardExt};

mod chart_widgets;
pub use chart_widgets::*;

mod checkbox;
pub use self::checkbox::{Checkbox, CheckboxExt};

mod chip;
pub use self::chip::{Chip, ChipExt};

mod circular_progress;
pub use self::circular_progress::{CircularProgress, CircularProgressExt};

mod data_table;
pub use self::data_table::{DataTable, DataTableExt};

mod date_picker;
pub use self::date_picker::{DatePicker, DatePickerExt};

mod dialog;
pub use self::dialog::{Dialog, DialogExt};

mod divider;
pub use self::divider::{Divider, DividerExt};

mod drawer;
pub use self::drawer::{Drawer, DrawerExt};

mod effects;
pub use effects::*;

mod fab;
pub use self::fab::{Fab, FabExt};

mod formfield;
pub use self::formfield::{Formfield, FormfieldExt};

mod icon_button;
pub use self::icon_button::{IconButton, IconButtonExt};

mod icon;
pub use self::icon::{Icon, IconExt};

mod image_list;
pub use self::image_list::{ImageList, ImageListExt};

mod layout;
pub use layout::*;

mod linear_progress;
pub use self::linear_progress::{LinearProgress, LinearProgressExt};

mod list;
pub use self::list::{List, ListExt};

mod menu;
pub use self::menu::{Menu, MenuExt};

mod radio;
pub use self::radio::{Radio, RadioExt};

mod sheet;
pub use self::sheet::{Sheet, SheetExt};

mod slider;
pub use self::slider::{Slider, SliderExt};

mod snackbar;
pub use self::snackbar::{Snackbar, SnackbarExt};

mod surface;
pub use self::surface::{Surface, SurfaceExt};

mod switch;
pub use self::switch::{Switch, SwitchExt};

mod tab;
pub use self::tab::{Tab, TabExt};

mod textarea;
pub use self::textarea::{Textarea, TextareaExt};

mod textfield;
pub use self::textfield::{Textfield, TextfieldExt};

mod time_picker;
pub use self::time_picker::{TimePicker, TimePickerExt};

mod tooltip;
pub use self::tooltip::{Tooltip, TooltipExt};

mod widget;
pub use self::widget::{Widget, WidgetExt};

mod window;
pub use self::window::{Window, WindowExt};

mod utils;
pub use utils::*;

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

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
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

impl Default for Position {
    fn default() -> Self {
        Position::Top
    }
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
