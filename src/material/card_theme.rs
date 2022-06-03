use crate::{
    foundation::colorspace::Color,
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder},
    ui::Clip,
};

pub struct CardTheme {
    pub clip_behavior: Clip,
    pub color: Color,
    pub shadow_color: Color,
    pub elevation: f32,
    pub margin: Box<dyn EdgeInsetsGeometry>,
    pub shape: Box<dyn ShapeBorder>,
}

impl Default for CardTheme {
    fn default() -> Self {
        Self {
            clip_behavior: Default::default(),
            color: Default::default(),
            shadow_color: Default::default(),
            elevation: Default::default(),
            margin: box NoneEdgeInsetsGeometry,
            shape: box NoneShapeBorder,
        }
    }
}
