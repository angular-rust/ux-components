#![allow(unused_imports)]
use std::time::Duration;

use crate::{
    elements::{Element, ThemeElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{
        BorderRadius, BoxShape, EdgeInsetsGeometry, ImageProvider, NoneEdgeInsetsGeometry,
        NoneShapeBorder, ShapeBorder,
    },
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{FocusNode, NoneImage, NoneWidget, Widget},
};

use super::{
    ButtonTextTheme, InteractiveInkFeatureFactory, MaterialStateProperty, MaterialTapTargetSize,
    ThemeData, VisualDensity,
};

pub struct Theme {
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,

    // Specifies the color and typography values for descendant widgets.
    pub data: ThemeData,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            child: box NoneWidget,
            data: Default::default(),
            key: Default::default(),
        }
    }
}

impl Widget for Theme {
    fn create_element(&self) -> Box<dyn Element> {
        box ThemeElement::new(self)
    }
}

impl WidgetProperties for Theme {
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
