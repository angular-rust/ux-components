use crate::{
    elements::{SingleChildScrollViewElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget,
};

use super::NoneWidget;

pub struct SingleChildScrollView {
    pub key: Key,
    pub width_factor: f32,
    pub height_factor: f32,
    pub child: Box<dyn Widget>,
}

impl Default for SingleChildScrollView {
    fn default() -> Self {
        Self {
            key: Default::default(),
            width_factor: Default::default(),
            height_factor: Default::default(),
            child: box NoneWidget,
        }
    }
}

impl Widget for SingleChildScrollView {
    fn create_element(&self) -> Box<dyn Element> {
        box SingleChildScrollViewElement::new(self)
    }
}

impl WidgetProperties for SingleChildScrollView {
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
