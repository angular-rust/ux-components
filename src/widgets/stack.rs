use crate::{
    elements::{Element, StackElement},
    foundation::{Id, Key, WidgetProperties},
    material::AlignmentGeometry,
    ui::{Clip, TextDirection},
};

use super::Widget;

pub struct Stack {
    pub key: Key,
    pub alignment: AlignmentGeometry,
    pub text_direction: TextDirection,
    // pub fit: StackFit,
    // @Deprecated("Use clipBehavior instead. See the migration guide in flutter.dev/go/clip-behavior.")
    // pub overflow: Overflow,
    pub clip_behavior: Clip,
    pub children: Vec<Box<dyn Widget>>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            key: Default::default(),
            alignment: Default::default(),
            text_direction: Default::default(),
            // fit: Default::default(),
            // overflow: Default::default(),
            clip_behavior: Default::default(),
            children: Default::default(),
        }
    }
}

impl Widget for Stack {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create StackElement");
        box StackElement::new(self)
    }
}

impl WidgetProperties for Stack {
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
