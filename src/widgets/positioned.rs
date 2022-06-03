use crate::{
    elements::{PositionedElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget,
};

use super::NoneWidget;

pub struct Positioned {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // The distance that the child's bottom edge is inset from the bottom of the stack.
    pub bottom: f32,
       
    // The child's height.
    pub height: f32,
    
    // The distance that the child's left edge is inset from the left of the stack.
    pub left: f32,
        
    // The distance that the child's right edge is inset from the right of the stack.
    pub right: f32,

    // The distance that the child's top edge is inset from the top of the stack.
    pub top: f32,
    
    // The child's width.
    pub width: f32,
    
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
}

impl Default for Positioned {
    fn default() -> Self {
        Self {
            key: Default::default(),
            bottom: Default::default(),
            height: Default::default(),
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
            width: Default::default(),
            child: box NoneWidget,
            
        }
    }
}

impl Widget for Positioned {
    fn create_element(&self) -> Box<dyn Element> {
        box PositionedElement::new(self)
    }
}

impl WidgetProperties for Positioned {
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
