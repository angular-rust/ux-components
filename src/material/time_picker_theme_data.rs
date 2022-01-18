use crate::{
    foundation::colorspace::Color,
    painting::{BorderSide, OutlinedBorder, ShapeBorder, TextStyle},
};

use super::InputDecorationTheme;

pub struct TimePickerThemeData {
    pub background_color: Color,
    pub hour_minute_text_color: Color,
    pub hour_minute_color: Color,
    pub day_period_text_color: Color,
    pub day_period_color: Color,
    pub dial_hand_color: Color,
    pub dial_background_color: Color,
    pub dial_text_color: Color,
    pub entry_mode_icon_color: Color,
    pub hour_minute_text_style: TextStyle,
    pub day_period_text_style: TextStyle,
    pub help_text_style: TextStyle,
    pub shape: ShapeBorder,
    pub hour_minute_shape: ShapeBorder,
    pub day_period_shape: OutlinedBorder,
    pub day_period_border_side: BorderSide,
    pub input_decoration_theme: InputDecorationTheme,
}

impl Default for TimePickerThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            hour_minute_text_color: Default::default(),
            hour_minute_color: Default::default(),
            day_period_text_color: Default::default(),
            day_period_color: Default::default(),
            dial_hand_color: Default::default(),
            dial_background_color: Default::default(),
            dial_text_color: Default::default(),
            entry_mode_icon_color: Default::default(),
            hour_minute_text_style: Default::default(),
            day_period_text_style: Default::default(),
            help_text_style: Default::default(),
            shape: Default::default(),
            hour_minute_shape: Default::default(),
            day_period_shape: Default::default(),
            day_period_border_side: Default::default(),
            input_decoration_theme: Default::default(),
        }
    }
}
