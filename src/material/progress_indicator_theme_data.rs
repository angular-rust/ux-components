use crate::foundation::colorspace::Color;
pub struct ProgressIndicatorThemeData {
    pub color: Color,
    pub linear_track_color: Color,
    pub linear_min_height: f32,
    pub circular_track_color: Color,
    pub refresh_background_color: Color,
}

impl Default for ProgressIndicatorThemeData {
    fn default() -> Self {
        Self {
            color: Default::default(),
            linear_track_color: Default::default(),
            linear_min_height: Default::default(),
            circular_track_color: Default::default(),
            refresh_background_color: Default::default(),
        }
    }
}
