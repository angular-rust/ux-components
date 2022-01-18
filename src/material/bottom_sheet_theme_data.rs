use crate::{
    foundation::colorspace::Color, painting::ShapeBorder, rendering::BoxConstraints, ui::Clip,
};
pub struct BottomSheetThemeData {
    pub background_color: Color,
    pub elevation: f32,
    pub modal_background_color: Color,
    pub modal_elevation: f32,
    pub shape: ShapeBorder,
    pub clip_behavior: Clip,
    pub constraints: BoxConstraints,
}

impl Default for BottomSheetThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            elevation: Default::default(),
            modal_background_color: Default::default(),
            modal_elevation: Default::default(),
            shape: Default::default(),
            clip_behavior: Default::default(),
            constraints: Default::default(),
        }
    }
}
