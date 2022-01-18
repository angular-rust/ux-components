use crate::foundation::colorspace::Color;

pub struct TextSelectionThemeData {
    pub cursor_color: Color,
    pub selection_color: Color,
    pub selection_handle_color: Color,
}

impl Default for TextSelectionThemeData {
    fn default() -> Self {
        Self {
            cursor_color: Default::default(),
            selection_color: Default::default(),
            selection_handle_color: Default::default(),
        }
    }
}
