use crate::{
    foundation::colorspace::Color,
    painting::{EdgeInsetsGeometry, TextStyle, NoneEdgeInsetsGeometry},
    rendering::BoxConstraints,
};

use super::{FloatingLabelBehavior, InputBorder};

pub struct InputDecorationTheme {
    pub label_style: TextStyle,
    pub floating_label_style: TextStyle,
    pub helper_style: TextStyle,
    pub helper_max_lines: i32,
    pub hint_style: TextStyle,
    pub error_style: TextStyle,
    pub error_max_lines: i32,
    pub floating_label_behavior: FloatingLabelBehavior,
    pub is_dense: bool,
    pub content_padding: Box<dyn EdgeInsetsGeometry>,
    pub is_collapsed: bool,
    pub prefix_style: TextStyle,
    pub suffix_style: TextStyle,
    pub counter_style: TextStyle,
    pub filled: bool,
    pub fill_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub error_border: InputBorder,
    pub focused_border: InputBorder,
    pub focused_error_border: InputBorder,
    pub disabled_border: InputBorder,
    pub enabled_border: InputBorder,
    pub border: InputBorder,
    pub align_label_with_hint: bool,
    pub constraints: BoxConstraints,
}

impl Default for InputDecorationTheme {
    fn default() -> Self {
        Self {
            label_style: Default::default(),
            floating_label_style: Default::default(),
            helper_style: Default::default(),
            helper_max_lines: Default::default(),
            hint_style: Default::default(),
            error_style: Default::default(),
            error_max_lines: Default::default(),
            floating_label_behavior: Default::default(),
            is_dense: Default::default(),
            content_padding: box NoneEdgeInsetsGeometry,
            is_collapsed: Default::default(),
            prefix_style: Default::default(),
            suffix_style: Default::default(),
            counter_style: Default::default(),
            filled: Default::default(),
            fill_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            error_border: Default::default(),
            focused_border: Default::default(),
            focused_error_border: Default::default(),
            disabled_border: Default::default(),
            enabled_border: Default::default(),
            border: Default::default(),
            align_label_with_hint: Default::default(),
            constraints: Default::default(),
        }
    }
}
