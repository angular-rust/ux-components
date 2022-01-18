use cgmath::prelude::*;
use cgmath::Matrix4;

use crate::{
    elements::{ContainerElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    material::AlignmentGeometry,
    painting::{Decoration, EdgeInsetsGeometry},
    rendering::BoxConstraints,
    ui::Clip,
    widgets::Widget,
};

use super::NullWidget;

pub struct Container {
    pub key: Key,
    pub alignment: AlignmentGeometry,
    pub padding: EdgeInsetsGeometry,
    pub color: Color,
    pub decoration: Decoration,
    pub foreground_decoration: Decoration,
    pub width: f32,
    pub height: f32,
    pub constraints: BoxConstraints,
    pub margin: EdgeInsetsGeometry,
    pub transform: Matrix4<f32>,
    pub transform_alignment: AlignmentGeometry,
    pub child: Box<dyn Widget>,
    pub clip_behavior: Clip,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            key: Default::default(),
            alignment: Default::default(),
            padding: Default::default(),
            color: Default::default(),
            decoration: Default::default(),
            foreground_decoration: Default::default(),
            width: Default::default(),
            height: Default::default(),
            constraints: Default::default(),
            margin: Default::default(),
            transform: Matrix4::identity(),
            transform_alignment: Default::default(),
            child: box NullWidget,
            clip_behavior: Default::default(),
        }
    }
}

impl Widget for Container {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ContainerElement");
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
