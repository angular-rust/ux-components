use super::ButtonStyle;

pub struct TextButtonThemeData {
    pub style: ButtonStyle,
}

impl Default for TextButtonThemeData {
    fn default() -> Self {
        Self {
            style: Default::default(),
        }
    }
}
