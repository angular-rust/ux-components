use std::time::Duration;

use crate::{
    animation::{Curve, NoneCurve},
    elements::{Element, FlutterLogoElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::RustLogoStyle,
};

use super::Widget;

// StatelessWidget
pub struct RustLogo {
    // The curve for the logo animation if the style or textColor change.
    pub curve: Box<dyn Curve>,

    // The length of time for the animation if the style or textColor properties are changed.
    pub duration: Duration,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // The size of the logo in logical pixels.
    pub size: f32,

    // Whether and where to draw the "Flutter" text. By default, only the logo itself is drawn.
    pub style: RustLogoStyle,

    // The color used to paint the "Flutter" text on the logo, if style is FlutterLogoStyle.horizontal or FlutterLogoStyle.stacked.
    pub text_color: Color,
}

impl Default for RustLogo {
    fn default() -> Self {
        Self {
            curve: box NoneCurve,
            duration: Default::default(),
            key: Default::default(),
            size: Default::default(),
            style: Default::default(),
            text_color: Default::default(),
        }
    }
}

impl Widget for RustLogo {
    fn create_element(&self) -> Box<dyn Element> {
        box FlutterLogoElement::new(self)
    }
}

impl WidgetProperties for RustLogo {
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
