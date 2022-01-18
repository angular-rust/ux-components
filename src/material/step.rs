use crate::{
    elements::{Element, StepElement},
    foundation::{Id, Key, WidgetProperties},
    widgets::{NullWidget, Widget},
};

use super::StepState;

pub struct Step {
    pub key: Key,
    pub title: Box<dyn Widget>,
    pub subtitle: Box<dyn Widget>,
    pub content: Box<dyn Widget>,
    pub state: StepState, // = StepState.indexed,
    pub is_active: bool,  // = false
}

impl Default for Step {
    fn default() -> Self {
        Self {
            key: Default::default(),
            title: box NullWidget,
            subtitle: box NullWidget,
            content: box NullWidget,
            state: Default::default(),
            is_active: Default::default(),
        }
    }
}

impl Widget for Step {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create StepElement");
        box StepElement::new(self)
    }
}

impl WidgetProperties for Step {
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
