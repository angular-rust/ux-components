use crate::{
    elements::{Element, SliverGridElement},
    foundation::{Id, Key, WidgetProperties},
    rendering::{SliverGridDelegate, NoneSliverGridDelegate},
    widgets::Widget,
};

use super::{SliverChildDelegate, NoneSliverChildDelegate};

pub struct SliverGrid {
    // The delegate that provides the children for this widget.
    pub delegate: Box<dyn SliverChildDelegate>,

    // The delegate that controls the size and position of the children.
    pub grid_delegate: Box<dyn SliverGridDelegate>,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
}

impl Default for SliverGrid {
    fn default() -> Self {
        Self {
            delegate: box NoneSliverChildDelegate,
            grid_delegate: box NoneSliverGridDelegate,
            key: Default::default(),
        }
    }
}

impl Widget for SliverGrid {
    fn create_element(&self) -> Box<dyn Element> {
        box SliverGridElement::new(self)
    }
}

impl WidgetProperties for SliverGrid {
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
