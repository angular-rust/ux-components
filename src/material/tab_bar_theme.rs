use crate::{
    foundation::colorspace::Color,
    painting::{Decoration, EdgeInsetsGeometry, NoneDecoration, NoneEdgeInsetsGeometry, TextStyle},
};

use super::TabBarIndicatorSize;

pub struct TabBarTheme {
    pub indicator: Box<dyn Decoration>,
    pub indicator_size: TabBarIndicatorSize,
    pub label_color: Color,
    pub label_padding: Box<dyn EdgeInsetsGeometry>,
    pub label_style: TextStyle,
    pub unselected_label_color: Color,
    pub unselected_label_style: TextStyle,
}

impl Default for TabBarTheme {
    fn default() -> Self {
        Self {
            indicator: box NoneDecoration,
            indicator_size: Default::default(),
            label_color: Default::default(),
            label_padding: box NoneEdgeInsetsGeometry,
            label_style: Default::default(),
            unselected_label_color: Default::default(),
            unselected_label_style: Default::default(),
        }
    }
}
