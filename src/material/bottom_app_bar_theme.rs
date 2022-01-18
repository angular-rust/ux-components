use crate::{foundation::colorspace::Color, painting::NotchedShape};

pub struct BottomAppBarTheme {
    pub color: Color,
    pub elevation: f32,
    pub shape: NotchedShape,
}

impl Default for BottomAppBarTheme {
    fn default() -> Self {
        Self {
            color: Default::default(),
            elevation: Default::default(),
            shape: Default::default(),
        }
    }
}
