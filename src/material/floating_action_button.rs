use crate::{
    elements::{Element, FloatingActionButtonElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::ShapeBorder,
    services::MouseCursor,
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NullWidget, Widget},
};

use super::MaterialTapTargetSize;

pub struct FloatingActionButton {
    pub key: Key,
    pub child: Box<dyn Widget>,
    pub tooltip: String,
    pub foreground_color: Color,
    pub background_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub splash_color: Color,
    // pub hero_tag: Object,
    pub elevation: f32,
    pub focus_elevation: f32,
    pub hover_elevation: f32,
    pub highlight_elevation: f32,
    pub disabled_elevation: f32,
    pub on_pressed: Option<Box<dyn VoidCallback>>,
    pub mouse_cursor: MouseCursor,
    pub mini: bool,
    pub shape: ShapeBorder,
    pub clip_behavior: Clip,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub is_extended: bool,
    pub enable_feedback: bool,
}

impl Default for FloatingActionButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            child: box NullWidget,
            tooltip: Default::default(),
            foreground_color: Default::default(),
            background_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            splash_color: Default::default(),
            // hero_tag: Default::default(),
            elevation: Default::default(),
            focus_elevation: Default::default(),
            hover_elevation: Default::default(),
            highlight_elevation: Default::default(),
            disabled_elevation: Default::default(),
            on_pressed: Default::default(),
            mouse_cursor: Default::default(),
            mini: Default::default(),
            shape: Default::default(),
            clip_behavior: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            material_tap_target_size: Default::default(),
            is_extended: Default::default(),
            enable_feedback: Default::default(),
        }
    }
}

impl Widget for FloatingActionButton {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create FloatingActionButtonElement");
        box FloatingActionButtonElement::new(self)
    }
}

impl WidgetProperties for FloatingActionButton {
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
