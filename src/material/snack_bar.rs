use std::time::Duration;

use crate::{
    elements::{Element, SnackBarElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder},
    ui::VoidCallback,
    widgets::{NoneWidget, Widget},
};

use super::SnackBarBehavior;

pub struct SnackBar {
    pub key: Key,
    pub content: Box<dyn Widget>,
    pub background_color: Color,
    pub elevation: f32,
    pub margin: Box<dyn EdgeInsetsGeometry>,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub width: f32,
    pub shape: Box<dyn ShapeBorder>,
    pub behavior: SnackBarBehavior,
    // pub action: SnackBarAction,
    pub duration: Duration,
    // pub animation: Animation<f32>,
    pub on_visible: Option<VoidCallback>,
    // pub dismiss_direction: DismissDirection,
}

impl Default for SnackBar {
    fn default() -> Self {
        Self {
            key: Default::default(),
            content: box NoneWidget,
            background_color: Default::default(),
            elevation: Default::default(),
            margin: box NoneEdgeInsetsGeometry,
            padding: box NoneEdgeInsetsGeometry,
            width: Default::default(),
            shape: box NoneShapeBorder,
            behavior: Default::default(),
            // action: Default::default(),
            duration: Default::default(),
            // animation: Default::default(),
            on_visible: Default::default(),
            // dismiss_direction: Default::default(),
        }
    }
}

impl Widget for SnackBar {
    fn create_element(&self) -> Box<dyn Element> {
        box SnackBarElement::new(self)
    }
}

impl WidgetProperties for SnackBar {
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
