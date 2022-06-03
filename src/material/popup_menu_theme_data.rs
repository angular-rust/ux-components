use crate::{
    foundation::colorspace::Color,
    painting::{ShapeBorder, TextStyle, NoneShapeBorder},
};

pub struct PopupMenuThemeData {
    pub color: Color,
    pub shape: Box<dyn ShapeBorder>,
    pub elevation: f32,
    pub text_style: TextStyle,
    pub enable_feedback: bool,
}

impl Default for PopupMenuThemeData {
    fn default() -> Self {
        Self {
            color: Default::default(),
            shape: box NoneShapeBorder,
            elevation: Default::default(),
            text_style: Default::default(),
            enable_feedback: Default::default(),
        }
    }
}
