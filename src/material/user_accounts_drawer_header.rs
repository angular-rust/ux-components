use std::time::Duration;

use crate::painting::{NoneShapeBorder, NoneBorderRadiusGeometry};
use crate::prelude::Color;

use crate::{
    elements::{Element, UserAccountsDrawerHeaderElement},
    foundation::{Id, Key, WidgetProperties},
    painting::{BorderRadiusGeometry, ShapeBorder, TextStyle},
    ui::Clip,
    widgets::{NoneWidget, Widget},
};

use super::MaterialType;

pub struct UserAccountsDrawerHeader {
    // Defines the duration of animated changes for shape, elevation, shadowColor and the elevation overlay if it is applied.
    pub animation_duration: Duration,

    // Whether to paint the shape border in front of the child.
    pub border_on_foreground: bool,

    // If non-null, the corners of this box are rounded by this BorderRadiusGeometry value.
    pub border_radius: Box<dyn BorderRadiusGeometry>,

    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,

    // The content will be clipped (or not) according to this option.
    pub clip_behavior: Clip,

    // The color to paint the material.
    pub color: Color,

    // The z-coordinate at which to place this material relative to its parent.
    pub elevation: f32,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // The color to paint the shadow below the material.
    pub shadow_color: Color,

    // Defines the material's shape as well its shadow.
    pub shape: Box<dyn ShapeBorder>,

    // The typographical style to use for text within this material.
    pub text_style: TextStyle,

    // The kind of material to show (e.g., card or canvas). This affects the shape of the widget,
    // the roundness of its corners if the shape is rectangular, and the default color.
    pub material_type: MaterialType,
}

impl Default for UserAccountsDrawerHeader {
    fn default() -> Self {
        Self {
            animation_duration: Default::default(),
            border_on_foreground: Default::default(),
            border_radius: box NoneBorderRadiusGeometry,
            child: box NoneWidget,
            clip_behavior: Default::default(),
            color: Default::default(),
            elevation: Default::default(),
            key: Default::default(),
            shadow_color: Default::default(),
            shape: box NoneShapeBorder,
            text_style: Default::default(),
            material_type: Default::default(),
        }
    }
}

impl Widget for UserAccountsDrawerHeader {
    fn create_element(&self) -> Box<dyn Element> {
        box UserAccountsDrawerHeaderElement::new(self)
    }
}

impl WidgetProperties for UserAccountsDrawerHeader {
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
