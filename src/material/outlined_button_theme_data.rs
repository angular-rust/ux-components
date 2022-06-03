use super::ButtonStyle;

pub struct OutlinedButtonThemeData {
    pub style: ButtonStyle,
}

impl Default for OutlinedButtonThemeData {
    fn default() -> Self {
        Self {
            style: Default::default(),
        }
    }
}
