use cgmath::prelude::*;

use cgmath::Matrix4;

use crate::{
    elements::{ContainerElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    material::{AlignmentGeometry, NoneAlignmentGeometry},
    painting::{Decoration, EdgeInsetsGeometry, NoneDecoration, NoneEdgeInsetsGeometry},
    rendering::BoxConstraints,
    ui::Clip,
    widgets::Widget,
};

use super::NoneWidget;

pub struct Container {
    pub key: Key,
    pub alignment: Box<dyn AlignmentGeometry>,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub color: Color,
    pub decoration: Box<dyn Decoration>,
    pub foreground_decoration: Box<dyn Decoration>,
    pub width: f32,
    pub height: f32,
    pub constraints: BoxConstraints,
    pub margin: Box<dyn EdgeInsetsGeometry>,
    pub transform: Matrix4<f32>,
    pub transform_alignment: Box<dyn AlignmentGeometry>,
    pub child: Box<dyn Widget>,
    pub clip_behavior: Clip,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            key: Default::default(),
            alignment: box NoneAlignmentGeometry,
            padding: box NoneEdgeInsetsGeometry,
            color: Default::default(),
            decoration: box NoneDecoration,
            foreground_decoration: box NoneDecoration,
            width: Default::default(),
            height: Default::default(),
            constraints: Default::default(),
            margin: box NoneEdgeInsetsGeometry,
            transform: Matrix4::identity(),
            transform_alignment: box NoneAlignmentGeometry,
            child: box NoneWidget,
            clip_behavior: Default::default(),
        }
    }
}

impl Widget for Container {
    fn create_element(&self) -> Box<dyn Element> {
        box ContainerElement::new(self)
    }
}

impl WidgetProperties for Container {
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
