#![allow(unused_imports)]
use std::time::Duration;

use crate::{
    elements::{Element, CircleAvatarElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder, BorderRadius, BoxShape, ImageProvider},
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget, NoneImage},
};

use super::{ButtonTextTheme, MaterialTapTargetSize, VisualDensity, MaterialStateProperty, InteractiveInkFeatureFactory};

pub struct CircleAvatar {
    // The color with which to fill the circle. Changing the background color will cause the avatar to animate to the new color.
    pub background_color: Color,
    
    // The background image of the circle. Changing the background image will cause the avatar to animate to the new image.
    pub background_image: Box<dyn ImageProvider>,
    
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    
    // The default text color for text in the circle.
    pub foreground_color: Color,
    
    // The foreground image of the circle.
    pub foreground_image: Box<dyn ImageProvider>,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // The maximum size of the avatar, expressed as the radius (half the diameter).
    pub max_radius: f32,
    
    // The minimum size of the avatar, expressed as the radius (half the diameter).
    pub min_radius: f32,
    
    // An optional error callback for errors emitted when loading backgroundImage.
    // pub on_background_image_error: Option<ImageErrorListener>,
    
    // An optional error callback for errors emitted when loading foregroundImage.
    // pub on_foreground_image_error: Option<ImageErrorListener>,
    
    // The size of the avatar, expressed as the radius (half the diameter).
    pub radius: f32,
}

impl Default for CircleAvatar {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            background_image: box NoneImage,
            child: box NoneWidget,
            foreground_color: Default::default(),
            foreground_image: box NoneImage,
            key: Default::default(),
            max_radius: Default::default(),
            min_radius: Default::default(),
            // on_background_image_error: Default::default(),
            // on_foreground_image_error: Default::default(),
            radius: Default::default(),            
        }
    }
}

impl Widget for CircleAvatar {
    fn create_element(&self) -> Box<dyn Element> {
        box CircleAvatarElement::new(self)
    }
}

impl WidgetProperties for CircleAvatar {
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
