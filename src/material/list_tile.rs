use crate::{
    elements::{Element, ListTileElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder},
    services::MouseCursor,
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::VisualDensity;

pub struct ListTile {
    pub key: Key,
    pub leading: Box<dyn Widget>,
    pub title: Box<dyn Widget>,
    pub subtitle: Box<dyn Widget>,
    pub trailing: Box<dyn Widget>,
    pub is_three_line: bool,
    pub dense: bool,
    pub visual_density: VisualDensity,
    pub shape: Box<dyn ShapeBorder>,
    pub content_padding: Box<dyn EdgeInsetsGeometry>,
    pub enabled: bool,
    // pub on_tap: GestureTapCallback,
    // pub on_long_press: GestureLongPressCallback,
    pub mouse_cursor: MouseCursor,
    pub selected: bool,
    pub focus_color: Color,
    pub hover_color: Color,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub tile_color: Color,
    pub selected_tile_color: Color,
    pub enable_feedback: bool,
    pub horizontal_title_gap: f32,
    pub min_vertical_padding: f32,
    pub min_leading_width: f32,
}

impl Default for ListTile {
    fn default() -> Self {
        Self {
            key: Default::default(),
            leading: box NoneWidget,
            title: box NoneWidget,
            subtitle: box NoneWidget,
            trailing: box NoneWidget,
            is_three_line: Default::default(),
            dense: Default::default(),
            visual_density: Default::default(),
            shape: box NoneShapeBorder,
            content_padding: box NoneEdgeInsetsGeometry,
            enabled: Default::default(),
            // on_tap: Default::default(),
            // on_long_press: Default::default(),
            mouse_cursor: Default::default(),
            selected: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            tile_color: Default::default(),
            selected_tile_color: Default::default(),
            enable_feedback: Default::default(),
            horizontal_title_gap: Default::default(),
            min_vertical_padding: Default::default(),
            min_leading_width: Default::default(),
        }
    }
}

impl Widget for ListTile {
    fn create_element(&self) -> Box<dyn Element> {
        box ListTileElement::new(self)
    }
}

impl WidgetProperties for ListTile {
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
