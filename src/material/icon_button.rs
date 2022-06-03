use crate::{
    elements::{Element, IconButtonElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry},
    rendering::BoxConstraints,
    services::MouseCursor,
    ui::VoidCallback,
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::{AlignmentGeometry, NoneAlignmentGeometry, VisualDensity};

pub struct IconButton {
    pub key: Key,
    pub icon_size: f32,
    pub visual_density: VisualDensity,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub alignment: Box<dyn AlignmentGeometry>,
    pub splash_radius: f32,
    pub color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub highlight_color: Color,
    pub splash_color: Color,
    pub disabled_color: Color,
    pub on_pressed: Option<VoidCallback>,
    pub mouse_cursor: MouseCursor,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub tooltip: String,
    pub enable_feedback: bool,
    pub constraints: BoxConstraints,
    pub icon: Box<dyn Widget>,
}

impl Default for IconButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            icon_size: Default::default(),
            visual_density: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            alignment: box NoneAlignmentGeometry,
            splash_radius: Default::default(),
            color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            highlight_color: Default::default(),
            splash_color: Default::default(),
            disabled_color: Default::default(),
            on_pressed: Default::default(),
            mouse_cursor: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            tooltip: Default::default(),
            enable_feedback: Default::default(),
            constraints: Default::default(),
            icon: box NoneWidget,
        }
    }
}

impl Widget for IconButton {
    fn create_element(&self) -> Box<dyn Element> {
        box IconButtonElement::new(self)
    }
}

impl WidgetProperties for IconButton {
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
