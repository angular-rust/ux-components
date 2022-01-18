use crate::{
    foundation::colorspace::Color,
    painting::{ShapeBorder, TextStyle},
};

pub struct DialogTheme {
    pub background_color: Color,
    pub elevation: f32,
    pub shape: ShapeBorder,
    pub title_text_style: TextStyle,
    pub content_text_style: TextStyle,
}

impl Default for DialogTheme {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            elevation: Default::default(),
            shape: Default::default(),
            title_text_style: Default::default(),
            content_text_style: Default::default(),
        }
    }
}
