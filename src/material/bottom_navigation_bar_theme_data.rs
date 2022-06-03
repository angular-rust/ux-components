use crate::{foundation::colorspace::Color, painting::TextStyle, widgets::IconThemeData};

use super::{BottomNavigationBarLandscapeLayout, BottomNavigationBarType};

pub struct BottomNavigationBarThemeData {
    pub background_color: Color,
    pub elevation: f32,
    pub selected_icon_theme: IconThemeData,
    pub unselected_icon_theme: IconThemeData,
    pub selected_item_color: Color,
    pub unselected_item_color: Color,
    pub selected_label_style: TextStyle,
    pub unselected_label_style: TextStyle,
    pub show_selected_labels: bool,
    pub show_unselected_labels: bool,
    pub bar_type: BottomNavigationBarType,
    pub enable_feedback: bool,
    pub landscape_layout: BottomNavigationBarLandscapeLayout,
}

impl Default for BottomNavigationBarThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            elevation: Default::default(),
            selected_icon_theme: Default::default(),
            unselected_icon_theme: Default::default(),
            selected_item_color: Default::default(),
            unselected_item_color: Default::default(),
            selected_label_style: Default::default(),
            unselected_label_style: Default::default(),
            show_selected_labels: Default::default(),
            show_unselected_labels: Default::default(),
            bar_type: Default::default(),
            enable_feedback: Default::default(),
            landscape_layout: Default::default(),
        }
    }
}
