use crate::{
    elements::{Element, IconElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    ui::TextDirection,
};

use super::{IconData, Widget};

pub struct Icon {
    pub icon: IconData,
    pub key: Key,
    pub size: f32,
    pub color: Color,
    pub semantic_label: String,
    pub text_direction: TextDirection,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            icon: Default::default(),
            key: Default::default(),
            size: Default::default(),
            color: Default::default(),
            semantic_label: Default::default(),
            text_direction: Default::default(),
        }
    }
}

impl Widget for Icon {
    fn create_element(&self) -> Box<dyn Element> {
        box IconElement::new(self)
    }
}

impl WidgetProperties for Icon {
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
