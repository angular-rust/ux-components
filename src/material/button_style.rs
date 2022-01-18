use std::time::Duration;

use crate::{
    foundation::colorspace::Color,
    painting::{BorderSide, EdgeInsetsGeometry, OutlinedBorder, TextStyle},
    services::MouseCursor,
    ui::Size,
};

use super::{
    AlignmentGeometry, InteractiveInkFeatureFactory, MaterialStateProperty, MaterialTapTargetSize,
    VisualDensity,
};

pub struct ButtonStyle {
    pub text_style: MaterialStateProperty<TextStyle>,
    pub background_color: MaterialStateProperty<Color>,
    pub foreground_color: MaterialStateProperty<Color>,
    pub overlay_color: MaterialStateProperty<Color>,
    pub shadow_color: MaterialStateProperty<Color>,
    pub elevation: MaterialStateProperty<f32>,
    pub padding: MaterialStateProperty<EdgeInsetsGeometry>,
    pub minimum_size: MaterialStateProperty<Size>,
    pub fixed_size: MaterialStateProperty<Size>,
    pub maximum_size: MaterialStateProperty<Size>,
    pub side: MaterialStateProperty<BorderSide>,
    pub shape: MaterialStateProperty<OutlinedBorder>,
    pub mouse_cursor: MaterialStateProperty<MouseCursor>,
    pub visual_density: VisualDensity,
    pub tap_target_size: MaterialTapTargetSize,
    pub animation_duration: Duration,
    pub enable_feedback: bool,
    pub alignment: AlignmentGeometry,
    pub splash_factory: InteractiveInkFeatureFactory,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            text_style: Default::default(),
            background_color: Default::default(),
            foreground_color: Default::default(),
            overlay_color: Default::default(),
            shadow_color: Default::default(),
            elevation: Default::default(),
            padding: Default::default(),
            minimum_size: Default::default(),
            fixed_size: Default::default(),
            maximum_size: Default::default(),
            side: Default::default(),
            shape: Default::default(),
            mouse_cursor: Default::default(),
            visual_density: Default::default(),
            tap_target_size: Default::default(),
            animation_duration: Default::default(),
            enable_feedback: Default::default(),
            alignment: Default::default(),
            splash_factory: Default::default(),
        }
    }
}
