use crate::{
    foundation::colorspace::Color,
    painting::{
        EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder, TextStyle,
    },
    rendering::BoxConstraints,
};

pub struct FloatingActionButtonThemeData {
    pub foreground_color: Color,
    pub background_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub splash_color: Color,
    pub elevation: f32,
    pub focus_elevation: f32,
    pub hover_elevation: f32,
    pub disabled_elevation: f32,
    pub highlight_elevation: f32,
    pub shape: Box<dyn ShapeBorder>,
    pub enable_feedback: bool,
    pub size_constraints: BoxConstraints,
    pub small_size_constraints: BoxConstraints,
    pub large_size_constraints: BoxConstraints,
    pub extended_size_constraints: BoxConstraints,
    pub extended_icon_label_spacing: f32,
    pub extended_padding: Box<dyn EdgeInsetsGeometry>,
    pub extended_text_style: TextStyle,
}

impl Default for FloatingActionButtonThemeData {
    fn default() -> Self {
        Self {
            foreground_color: Default::default(),
            background_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            splash_color: Default::default(),
            elevation: Default::default(),
            focus_elevation: Default::default(),
            hover_elevation: Default::default(),
            disabled_elevation: Default::default(),
            highlight_elevation: Default::default(),
            shape: box NoneShapeBorder,
            enable_feedback: Default::default(),
            size_constraints: Default::default(),
            small_size_constraints: Default::default(),
            large_size_constraints: Default::default(),
            extended_size_constraints: Default::default(),
            extended_icon_label_spacing: Default::default(),
            extended_padding: box NoneEdgeInsetsGeometry,
            extended_text_style: Default::default(),
        }
    }
}
