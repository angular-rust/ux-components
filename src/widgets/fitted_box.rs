use crate::{
    elements::{FittedBoxElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget, painting::BoxFit, ui::Clip, material::{AlignmentGeometry, NoneAlignmentGeometry},
};

use super::NoneWidget;

pub struct FittedBox {
    // How to align the child within its parent's bounds.
    pub alignment: Box<dyn AlignmentGeometry>,
    
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    
    // The content will be clipped (or not) according to this option.
    pub clip_behavior: Clip,
    
    // How to inscribe the child into the space allocated during layout.
    pub fit: BoxFit,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
}

impl Default for FittedBox {
    fn default() -> Self {
        Self {
            alignment: box NoneAlignmentGeometry,
            child: box NoneWidget, 
            clip_behavior: Default::default(),
            fit: Default::default(),
            key: Default::default(),
        }
    }
}

impl Widget for FittedBox {
    fn create_element(&self) -> Box<dyn Element> {
        box FittedBoxElement::new(self)
    }
}

impl WidgetProperties for FittedBox {
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
