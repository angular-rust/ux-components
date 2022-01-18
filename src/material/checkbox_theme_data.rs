use crate::{
    foundation::colorspace::Color,
    painting::{BorderSide, OutlinedBorder},
    services::MouseCursor,
};

use super::{MaterialStateProperty, MaterialTapTargetSize, VisualDensity};
pub struct CheckboxThemeData {
    pub mouse_cursor: MaterialStateProperty<MouseCursor>,
    pub fill_color: MaterialStateProperty<Color>,
    pub check_color: MaterialStateProperty<Color>,
    pub overlay_color: MaterialStateProperty<Color>,
    pub splash_radius: f32,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub visual_density: VisualDensity,
    pub shape: OutlinedBorder,
    pub side: BorderSide,
}

impl Default for CheckboxThemeData {
    fn default() -> Self {
        Self {
            mouse_cursor: Default::default(),
            fill_color: Default::default(),
            check_color: Default::default(),
            overlay_color: Default::default(),
            splash_radius: Default::default(),
            material_tap_target_size: Default::default(),
            visual_density: Default::default(),
            shape: Default::default(),
            side: Default::default(),
        }
    }
}
