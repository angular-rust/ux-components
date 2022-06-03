use crate::{
    elements::{FlexibleElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget, rendering::FlexFit,
};

use super::NoneWidget;

pub struct Flexible {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    // How a flexible child is inscribed into the available space.
    pub fit: FlexFit,
    // The flex factor to use for this child.
    pub flex: i32,
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
}

impl Default for Flexible {
    fn default() -> Self {
        Self {
            key: Default::default(),
            fit: Default::default(),
            flex: Default::default(),
            child: box NoneWidget,
        }
    }
}

impl Widget for Flexible {
    fn create_element(&self) -> Box<dyn Element> {
        box FlexibleElement::new(self)
    }
}

impl WidgetProperties for Flexible {
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
