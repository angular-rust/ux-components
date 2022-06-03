use crate::foundation::TargetPlatform;

use super::TextTheme;

pub struct Typography {
    pub platform: TargetPlatform,
    pub black: TextTheme,
    pub white: TextTheme,
    pub english_like: TextTheme,
    pub dense: TextTheme,
    pub tall: TextTheme,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            platform: Default::default(),
            black: Default::default(),
            white: Default::default(),
            english_like: Default::default(),
            dense: Default::default(),
            tall: Default::default(),
        }
    }
}
