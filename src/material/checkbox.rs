use crate::{
    elements::{CheckboxElement, Element},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{BorderSide, OutlinedBorder},
    services::MouseCursor,
    widgets::{FocusNode, Widget},
};

use super::{MaterialStateProperty, MaterialTapTargetSize, VisualDensity};

pub struct Checkbox {
    pub key: Key,
    pub value: bool,
    pub tristate: bool,
    pub on_changed: Option<Box<dyn ValueChanged<bool>>>,
    pub mouse_cursor: MouseCursor,
    pub active_color: Color,
    pub fill_color: MaterialStateProperty<Color>,
    pub check_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub overlay_color: MaterialStateProperty<Color>,
    pub splash_radius: f32,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub visual_density: VisualDensity,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub shape: OutlinedBorder,
    pub side: BorderSide,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            tristate: Default::default(),
            on_changed: Default::default(),
            mouse_cursor: Default::default(),
            active_color: Default::default(),
            fill_color: Default::default(),
            check_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            overlay_color: Default::default(),
            splash_radius: Default::default(),
            material_tap_target_size: Default::default(),
            visual_density: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            shape: Default::default(),
            side: Default::default(),
        }
    }
}

impl Widget for Checkbox {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create CheckboxElement");
        box CheckboxElement::new(self)
    }
}

impl WidgetProperties for Checkbox {
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
