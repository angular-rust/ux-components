use crate::foundation::colorspace::Color;

pub struct DividerThemeData {
    pub color: Color,
    pub space: f32,
    pub thickness: f32,
    pub indent: f32,
    pub end_indent: f32,
}

impl Default for DividerThemeData {
    fn default() -> Self {
        Self {
            color: Default::default(),
            space: Default::default(),
            thickness: Default::default(),
            indent: Default::default(),
            end_indent: Default::default(),
        }
    }
}
