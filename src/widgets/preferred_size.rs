use crate::{
    elements::{Element, PreferredSizeElement},
    foundation::{Id, Key, WidgetProperties},
    ui::Size,
    widgets::Widget,
};

use super::{NoneWidget, PreferredSizeWidget};

pub struct PreferredSize {
    pub key: Key,
    pub child: Box<dyn Widget>,
    pub preferred_size: Size,
}

impl Default for PreferredSize {
    fn default() -> Self {
        Self {
            key: Default::default(),
            child: box NoneWidget,
            preferred_size: Size::default(),
        }
    }
}

impl PreferredSizeWidget for PreferredSize {
    fn preferred_size(&self) -> Size {
        self.preferred_size.clone()
    }
}

impl Widget for PreferredSize {
    fn create_element(&self) -> Box<dyn Element> {
        box PreferredSizeElement::new(self)
    }
}

impl WidgetProperties for PreferredSize {
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
