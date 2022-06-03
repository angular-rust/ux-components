#![allow(unused_imports)]
use std::time::Duration;

use crate::{
    animation::AnimationController,
    elements::{Element, ModalBottomSheetElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{
        BorderRadius, BoxShape, EdgeInsetsGeometry, ImageProvider, NoneEdgeInsetsGeometry,
        NoneShapeBorder, ShapeBorder,
    },
    rendering::BoxConstraints,
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{
        BuildContext, FocusNode, NoneImage, NoneWidget, RouteSettings, Widget, WidgetBuilder,
    },
};

use super::{
    ButtonTextTheme, InteractiveInkFeatureFactory, MaterialStateProperty, MaterialTapTargetSize,
    VisualDensity,
};

// TODO should implement show method
pub struct ModalBottomSheet {
    pub key: Key,
    pub context: Option<BuildContext>,
    pub builder: WidgetBuilder,
    // The color with which to fill the circle. Changing the background color will cause the avatar to animate to the new color.
    pub background_color: Color,
    pub elevation: f32,
    pub shape: Box<dyn ShapeBorder>,
    pub clip_behavior: Clip,
    pub constraints: BoxConstraints,
    pub barrier_color: Color,
    pub is_scroll_controlled: bool, // = false,
    pub use_root_navigator: bool,   // = false,
    pub is_dismissible: bool,       // = true,
    pub enable_drag: bool,          // = true,
    pub route_settings: Option<RouteSettings<String>>,
    pub transition_animation_controller: Option<AnimationController>,
}

impl ModalBottomSheet {
    pub fn show() {
        todo!()
    }
}

impl Default for ModalBottomSheet {
    fn default() -> Self {
        Self {
            key: Default::default(),
            context: Default::default(),
            builder: box |_| box NoneWidget,
            background_color: Default::default(),
            elevation: Default::default(),
            shape: box NoneShapeBorder,
            clip_behavior: Default::default(),
            constraints: Default::default(),
            barrier_color: Default::default(),
            is_scroll_controlled: Default::default(),
            use_root_navigator: Default::default(),
            is_dismissible: Default::default(),
            enable_drag: Default::default(),
            route_settings: Default::default(),
            transition_animation_controller: Default::default(),
        }
    }
}

impl Widget for ModalBottomSheet {
    fn create_element(&self) -> Box<dyn Element> {
        box ModalBottomSheetElement::new(self)
    }
}

impl WidgetProperties for ModalBottomSheet {
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
