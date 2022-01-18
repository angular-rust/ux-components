use crate::{foundation::colorspace::Color, ui::Radius};

use super::MaterialStateProperty;

pub struct ScrollbarThemeData {
    pub thickness: MaterialStateProperty<f32>,
    pub show_track_on_hover: bool,
    pub is_always_shown: bool,
    pub radius: Radius,
    pub thumb_color: MaterialStateProperty<Color>,
    pub track_color: MaterialStateProperty<Color>,
    pub track_border_color: MaterialStateProperty<Color>,
    pub cross_axis_margin: f32,
    pub main_axis_margin: f32,
    pub min_thumb_length: f32,
    pub interactive: bool,
}

impl Default for ScrollbarThemeData {
    fn default() -> Self {
        Self {
            thickness: Default::default(),
            show_track_on_hover: Default::default(),
            is_always_shown: Default::default(),
            radius: Default::default(),
            thumb_color: Default::default(),
            track_color: Default::default(),
            track_border_color: Default::default(),
            cross_axis_margin: Default::default(),
            main_axis_margin: Default::default(),
            min_thumb_length: Default::default(),
            interactive: Default::default(),
        }
    }
}
