use crate::{
    elements::{BottomNavigationBarElement, Element},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::TextStyle,
    services::MouseCursor,
    widgets::{IconThemeData, Widget},
};

use super::{BottomNavigationBarLandscapeLayout, BottomNavigationBarType};

pub struct BottomNavigationBar {
    pub key: Key,
    // pub items: Vec<BottomNavigationBarItem>,
    pub on_tap: Option<ValueChanged<i32>>,
    pub current_index: i32,
    pub elevation: f32,
    pub bar_type: BottomNavigationBarType,
    pub fixed_color: Color,
    pub background_color: Color,
    pub icon_size: f32,
    pub selected_item_color: Color,
    pub unselected_item_color: Color,
    pub selected_icon_theme: IconThemeData,
    pub unselected_icon_theme: IconThemeData,
    pub selected_font_size: f32,
    pub unselected_font_size: f32,
    pub selected_label_style: TextStyle,
    pub unselected_label_style: TextStyle,
    pub show_selected_labels: bool,
    pub show_unselected_labels: bool,
    pub mouse_cursor: MouseCursor,
    pub enable_feedback: bool,
    pub landscape_layout: BottomNavigationBarLandscapeLayout,
}

impl Default for BottomNavigationBar {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // items: Default::default(),
            on_tap: Default::default(),
            current_index: Default::default(),
            elevation: Default::default(),
            bar_type: Default::default(),
            fixed_color: Default::default(),
            background_color: Default::default(),
            icon_size: Default::default(),
            selected_item_color: Default::default(),
            unselected_item_color: Default::default(),
            selected_icon_theme: Default::default(),
            unselected_icon_theme: Default::default(),
            selected_font_size: Default::default(),
            unselected_font_size: Default::default(),
            selected_label_style: Default::default(),
            unselected_label_style: Default::default(),
            show_selected_labels: Default::default(),
            show_unselected_labels: Default::default(),
            mouse_cursor: Default::default(),
            enable_feedback: Default::default(),
            landscape_layout: Default::default(),
        }
    }
}

impl Widget for BottomNavigationBar {
    fn create_element(&self) -> Box<dyn Element> {
        box BottomNavigationBarElement::new(self)
    }
}

impl WidgetProperties for BottomNavigationBar {
    fn key(&self) -> &Key {
        &self.key
    }

    fn x(&self) -> f32 {
        // self.x
        0.0
    }

    fn y(&self) -> f32 {
        // self.y
        0.0
    }

    fn w(&self) -> f32 {
        // self.w
        0.0
    }

    fn h(&self) -> f32 {
        // self.h
        0.0
    }

    fn w_min(&self) -> f32 {
        // self.w_min
        0.0
    }

    fn h_min(&self) -> f32 {
        // self.h_min
        0.0
    }

    fn w_max(&self) -> f32 {
        // self.w_max
        0.0
    }

    fn h_max(&self) -> f32 {
        // self.h_max
        0.0
    }

    fn parent(&self) -> Option<Id> {
        // self.parent
        None
    }

    fn depth(&self) -> f32 {
        // self.depth
        0.0
    }

    fn visible(&self) -> bool {
        // self.visible
        true
    }

    fn mouse_input(&self) -> bool {
        // self.mouse_input
        true
    }

    fn key_input(&self) -> bool {
        // self.key_input
        true
    }

    fn renderable(&self) -> bool {
        // self.renderable
        true
    }

    fn internal_visible(&self) -> bool {
        // self.internal_visible
        true
    }
}
