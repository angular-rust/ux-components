use crate::{foundation::colorspace::Color, services::MouseCursor};

use super::{MaterialStateProperty, MaterialTapTargetSize};
pub struct SwitchThemeData {
    pub thumb_color: MaterialStateProperty<Color>,
    pub track_color: MaterialStateProperty<Color>,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub mouse_cursor: MaterialStateProperty<MouseCursor>,
    pub overlay_color: MaterialStateProperty<Color>,
    pub splash_radius: f32,
}

impl Default for SwitchThemeData {
    fn default() -> Self {
        Self {
            thumb_color: Default::default(),
            track_color: Default::default(),
            material_tap_target_size: Default::default(),
            mouse_cursor: Default::default(),
            overlay_color: Default::default(),
            splash_radius: Default::default(),
        }
    }
}
