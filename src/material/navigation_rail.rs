use crate::{
    elements::{Element, NavigationRailElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::TextStyle,
    widgets::{IconThemeData, NoneWidget, Widget},
};

use super::{NavigationRailDestination, NavigationRailLabelType};

pub struct NavigationRail {
    pub key: Key,
    pub background_color: Color,
    pub extended: bool, // = false,
    pub leading: Box<dyn Widget>,
    pub trailing: Box<dyn Widget>,
    pub destinations: Vec<NavigationRailDestination>,
    pub selected_index: i32,
    pub on_destination_selected: Option<ValueChanged<i32>>,
    pub elevation: f32,
    pub group_alignment: f32,
    pub label_type: NavigationRailLabelType,
    pub unselected_label_text_style: TextStyle,
    pub selected_label_text_style: TextStyle,
    pub unselected_icon_theme: IconThemeData,
    pub selected_icon_theme: IconThemeData,
    pub min_width: f32,
    pub min_extended_width: f32,
}

impl Default for NavigationRail {
    fn default() -> Self {
        Self {
            key: Default::default(),
            background_color: Default::default(),
            extended: Default::default(),
            leading: box NoneWidget,
            trailing: box NoneWidget,
            destinations: Default::default(),
            selected_index: Default::default(),
            on_destination_selected: Default::default(),
            elevation: Default::default(),
            group_alignment: Default::default(),
            label_type: Default::default(),
            unselected_label_text_style: Default::default(),
            selected_label_text_style: Default::default(),
            unselected_icon_theme: Default::default(),
            selected_icon_theme: Default::default(),
            min_width: Default::default(),
            min_extended_width: Default::default(),
        }
    }
}

impl Widget for NavigationRail {
    fn create_element(&self) -> Box<dyn Element> {
        box NavigationRailElement::new(self)
    }
}

impl WidgetProperties for NavigationRail {
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
