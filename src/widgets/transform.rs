use cgmath::prelude::*;

use cgmath::Matrix4;

use crate::{
    elements::{Element, TransformElement},
    foundation::{Id, Key, WidgetProperties},
    material::{AlignmentGeometry, NoneAlignmentGeometry},
    ui::{FilterQuality, Offset},
    widgets::Widget,
};

use super::NoneWidget;

pub struct Transform {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,

    // The alignment of the origin, relative to the size of the box.
    pub alignment: Box<dyn AlignmentGeometry>,

    // The filter quality with which to apply the transform as a bitmap operation.
    pub filter_quality: Option<FilterQuality>,

    // The origin of the coordinate system (relative to the upper left corner of this render object) in which to apply the matrix.
    pub origin: Option<Offset>,

    // The matrix to transform the child by during painting.
    pub transform: Matrix4<f32>,

    // Whether to apply the transformation when performing hit tests.
    pub transform_hit_tests: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            key: Default::default(),
            child: box NoneWidget,
            alignment: box NoneAlignmentGeometry,
            filter_quality: Default::default(),
            origin: Default::default(),
            transform: Matrix4::identity(),
            transform_hit_tests: Default::default(),
            
        }
    }
}

impl Widget for Transform {
    fn create_element(&self) -> Box<dyn Element> {
        box TransformElement::new(self)
    }
}

impl WidgetProperties for Transform {
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
