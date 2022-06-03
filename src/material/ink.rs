#![allow(unused_imports)]
use std::time::Duration;

use crate::{
    elements::{Element, InkElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder, BorderRadius, BoxShape, ImageProvider, Decoration, NoneDecoration},
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget, NoneImage},
};

use super::{ButtonTextTheme, MaterialTapTargetSize, VisualDensity, MaterialStateProperty, InteractiveInkFeatureFactory};

pub struct Ink {  
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    
    // The decoration to paint on the nearest ancestor Material widget. [...]
    pub decoration: Box<dyn Decoration>,
    
    // A height to apply to the decoration and the child. The height includes any padding.
    pub height: f32,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // Empty space to inscribe inside the decoration. The child, if any, is placed inside this padding. [...]
    pub padding: Box<dyn EdgeInsetsGeometry>,
    
    // A width to apply to the decoration and the child. The width includes any padding.
    pub width: f32,
}

impl Default for Ink {
    fn default() -> Self {
        Self {
            child: box NoneWidget,
            decoration: box NoneDecoration,
            height: Default::default(),
            key: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            width: Default::default(),
            
        }
    }
}

impl Widget for Ink {
    fn create_element(&self) -> Box<dyn Element> {
        box InkElement::new(self)
    }
}

impl WidgetProperties for Ink {
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
