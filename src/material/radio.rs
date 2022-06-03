use crate::{
    elements::{Element, RadioElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    services::MouseCursor,
    widgets::{FocusNode, Widget},
};

use super::{MaterialStateProperty, MaterialTapTargetSize, VisualDensity};

pub struct Radio<T: Default> {
    pub key: Key,
    pub value: T,
    pub group_value: T,
    pub on_changed: Option<ValueChanged<T>>,
    pub mouse_cursor: MouseCursor,
    pub toggleable: bool,
    pub active_color: Color,
    pub fill_color: MaterialStateProperty<Color>,
    pub focus_color: Color,
    pub hover_color: Color,
    pub overlay_color: MaterialStateProperty<Color>,
    pub splash_radius: f32,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub visual_density: VisualDensity,
    pub focus_node: FocusNode,
    pub autofocus: bool,
}

impl<T: Default> Default for Radio<T> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            group_value: Default::default(),
            on_changed: Default::default(),
            mouse_cursor: Default::default(),
            toggleable: Default::default(),
            active_color: Default::default(),
            fill_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            overlay_color: Default::default(),
            splash_radius: Default::default(),
            material_tap_target_size: Default::default(),
            visual_density: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
        }
    }
}

impl<T: Default> Widget for Radio<T> {
    fn create_element(&self) -> Box<dyn Element> {
        box RadioElement::new(self)
    }
}

impl<T: Default> WidgetProperties for Radio<T> {
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
