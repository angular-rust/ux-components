use crate::{
    elements::{Element, SizedBoxElement},
    foundation::{Id, Key, WidgetProperties},
    prelude::With,
    ui::Size,
    widgets::Widget,
};

use super::NoneWidget;

pub struct SizedBox {
    pub key: Key,
    pub width: f32,
    pub height: f32,
    pub child: Box<dyn Widget>,
}

impl With<Size> for SizedBox {
    fn with(mut self, param: Size) -> Self {
        self.width = param.0;
        self.height = param.1;

        self
    }
}

impl Default for SizedBox {
    fn default() -> Self {
        Self {
            key: Default::default(),
            width: Default::default(),
            height: Default::default(),
            child: box NoneWidget,
        }
    }
}

impl Widget for SizedBox {
    fn create_element(&self) -> Box<dyn Element> {
        box SizedBoxElement::new(self)
    }
}

impl WidgetProperties for SizedBox {
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
