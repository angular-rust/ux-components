use crate::{
    foundation::colorspace::Color,
    painting::{Decoration, EdgeInsetsGeometry, TextStyle},
};

use super::TabBarIndicatorSize;

pub struct TabBarTheme {
    pub indicator: Decoration,
    pub indicator_size: TabBarIndicatorSize,
    pub label_color: Color,
    pub label_padding: EdgeInsetsGeometry,
    pub label_style: TextStyle,
    pub unselected_label_color: Color,
    pub unselected_label_style: TextStyle,
}

impl Default for TabBarTheme {
    fn default() -> Self {
        Self {
            indicator: Default::default(),
            indicator_size: Default::default(),
            label_color: Default::default(),
            label_padding: Default::default(),
            label_style: Default::default(),
            unselected_label_color: Default::default(),
            unselected_label_style: Default::default(),
        }
    }
}
