use std::time::Duration;

use crate::{
    elements::{Element, RawMaterialButtonElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{EdgeInsetsGeometry, ShapeBorder, TextStyle},
    rendering::BoxConstraints,
    services::MouseCursor,
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NullWidget, Widget},
};

use super::{MaterialTapTargetSize, VisualDensity};

pub struct RawMaterialButton {
    pub key: Key,
    pub on_pressed: Option<Box<dyn VoidCallback>>,
    pub on_long_press: Option<Box<dyn VoidCallback>>,
    pub on_highlight_changed: Option<Box<dyn ValueChanged<bool>>>,
    pub mouse_cursor: MouseCursor,
    pub text_style: TextStyle,
    pub fill_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub highlight_color: Color,
    pub splash_color: Color,
    pub elevation: f32,
    pub focus_elevation: f32,
    pub hover_elevation: f32,
    pub highlight_elevation: f32,
    pub disabled_elevation: f32,
    pub padding: EdgeInsetsGeometry,
    pub visual_density: VisualDensity,
    pub constraints: BoxConstraints,
    pub shape: ShapeBorder,
    pub animation_duration: Duration,
    pub clip_behavior: Clip,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub child: Box<dyn Widget>,
    pub enable_feedback: bool,
}

impl Default for RawMaterialButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            on_pressed: Default::default(),
            on_long_press: Default::default(),
            on_highlight_changed: Default::default(),
            mouse_cursor: Default::default(),
            text_style: Default::default(),
            fill_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            highlight_color: Default::default(),
            splash_color: Default::default(),
            elevation: Default::default(),
            focus_elevation: Default::default(),
            hover_elevation: Default::default(),
            highlight_elevation: Default::default(),
            disabled_elevation: Default::default(),
            padding: Default::default(),
            visual_density: Default::default(),
            constraints: Default::default(),
            shape: Default::default(),
            animation_duration: Default::default(),
            clip_behavior: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            material_tap_target_size: Default::default(),
            child: box NullWidget,
            enable_feedback: Default::default(),
        }
    }
}

impl Widget for RawMaterialButton {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create RawMaterialButtonElement");
        box RawMaterialButtonElement::new(self)
    }
}

impl WidgetProperties for RawMaterialButton {
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
