use crate::{
    foundation::colorspace::Color,
    painting::{ShapeBorder, TextStyle},
};

use super::SnackBarBehavior;

pub struct SnackBarThemeData {
    pub background_color: Color,
    pub action_text_color: Color,
    pub disabled_action_text_color: Color,
    pub content_text_style: TextStyle,
    pub elevation: f32,
    pub shape: ShapeBorder,
    pub behavior: SnackBarBehavior,
}

impl Default for SnackBarThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            action_text_color: Default::default(),
            disabled_action_text_color: Default::default(),
            content_text_style: Default::default(),
            elevation: Default::default(),
            shape: Default::default(),
            behavior: Default::default(),
        }
    }
}
