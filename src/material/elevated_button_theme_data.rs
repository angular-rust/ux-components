use super::ButtonStyle;

pub struct ElevatedButtonThemeData {
    pub style: ButtonStyle,
}

impl Default for ElevatedButtonThemeData {
    fn default() -> Self {
        Self {
            style: Default::default(),
        }
    }
}
