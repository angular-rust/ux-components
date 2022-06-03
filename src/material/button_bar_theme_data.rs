use crate::{
    painting::{EdgeInsetsGeometry, VerticalDirection, NoneEdgeInsetsGeometry},
    rendering::{MainAxisAlignment, MainAxisSize},
};

use super::{ButtonBarLayoutBehavior, ButtonTextTheme};

pub struct ButtonBarThemeData {
    pub alignment: MainAxisAlignment,
    pub main_axis_size: MainAxisSize,
    pub button_text_theme: ButtonTextTheme,
    pub button_min_width: f32,
    pub button_height: f32,
    pub button_padding: Box<dyn EdgeInsetsGeometry>,
    pub button_aligned_dropdown: bool,
    pub layout_behavior: ButtonBarLayoutBehavior,
    pub overflow_direction: VerticalDirection,
}

impl Default for ButtonBarThemeData {
    fn default() -> Self {
        Self {
            alignment: Default::default(),
            main_axis_size: Default::default(),
            button_text_theme: Default::default(),
            button_min_width: Default::default(),
            button_height: Default::default(),
            button_padding: box NoneEdgeInsetsGeometry,
            button_aligned_dropdown: Default::default(),
            layout_behavior: Default::default(),
            overflow_direction: Default::default(),
        }
    }
}
