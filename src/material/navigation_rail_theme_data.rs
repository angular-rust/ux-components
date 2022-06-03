use crate::{foundation::colorspace::Color, painting::TextStyle, widgets::IconThemeData};

use super::NavigationRailLabelType;

pub struct NavigationRailThemeData {
    pub background_color: Color,
    pub elevation: f32,
    pub unselected_label_text_style: TextStyle,
    pub selected_label_text_style: TextStyle,
    pub unselected_icon_theme: IconThemeData,
    pub selected_icon_theme: IconThemeData,
    pub group_alignment: f32,
    pub label_type: NavigationRailLabelType,
}

impl Default for NavigationRailThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            elevation: Default::default(),
            unselected_label_text_style: Default::default(),
            selected_label_text_style: Default::default(),
            unselected_icon_theme: Default::default(),
            selected_icon_theme: Default::default(),
            group_alignment: Default::default(),
            label_type: Default::default(),
        }
    }
}
