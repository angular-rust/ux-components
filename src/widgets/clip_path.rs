use crate::{
    elements::{ClipPathElement, Element},
    foundation::{Id, Key, WidgetProperties},
    rendering::{CustomClipper, NoneCustomPathClipper},
    ui::{Path, Clip},
    widgets::Widget,
};

use super::NoneWidget;

pub struct ClipPath {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    // Controls how to clip.
    pub clip_behavior: Clip,
    // If non-null, determines which clip to use.
    pub clipper: Box<dyn CustomClipper<Path>>,
}

impl Default for ClipPath {
    fn default() -> Self {
        Self {
            key: Default::default(),
            child: box NoneWidget,
            clip_behavior: Default::default(),
            clipper: box NoneCustomPathClipper,
        }
    }
}

impl Widget for ClipPath {
    fn create_element(&self) -> Box<dyn Element> {
        box ClipPathElement::new(self)
    }
}

impl WidgetProperties for ClipPath {
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
