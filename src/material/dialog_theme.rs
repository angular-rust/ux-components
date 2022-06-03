use crate::{
    foundation::colorspace::Color,
    painting::{NoneShapeBorder, ShapeBorder, TextStyle},
};

pub struct DialogTheme {
    pub background_color: Color,
    pub elevation: f32,
    pub shape: Box<dyn ShapeBorder>,
    pub title_text_style: TextStyle,
    pub content_text_style: TextStyle,
}

impl Default for DialogTheme {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            elevation: Default::default(),
            shape: box NoneShapeBorder,
            title_text_style: Default::default(),
            content_text_style: Default::default(),
        }
    }
}
