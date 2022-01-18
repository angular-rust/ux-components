use std::time::Duration;

use crate::{
    elements::{Element, MaterialButtonElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{EdgeInsetsGeometry, ShapeBorder},
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{FocusNode, NullWidget, Widget},
};

use super::{ButtonTextTheme, MaterialTapTargetSize, VisualDensity};

pub struct MaterialButton {
    pub key: Key,
    pub on_pressed: Option<Box<dyn VoidCallback>>,
    pub on_long_press: Option<Box<dyn VoidCallback>>,
    pub on_highlight_changed: Option<Box<dyn ValueChanged<bool>>>,
    pub mouse_cursor: MouseCursor,
    pub text_theme: ButtonTextTheme,
    pub text_color: Color,
    pub disabled_text_color: Color,
    pub color: Color,
    pub disabled_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub highlight_color: Color,
    pub splash_color: Color,
    pub color_brightness: Brightness,
    pub elevation: f32,
    pub focus_elevation: f32,
    pub hover_elevation: f32,
    pub highlight_elevation: f32,
    pub disabled_elevation: f32,
    pub padding: EdgeInsetsGeometry,
    pub visual_density: VisualDensity,
    pub shape: ShapeBorder,
    pub clip_behavior: Clip,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub animation_duration: Duration,
    pub min_width: f32,
    pub height: f32,
    pub enable_feedback: bool,
    pub child: Box<dyn Widget>,
}

impl Default for MaterialButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            on_pressed: Default::default(),
            on_long_press: Default::default(),
            on_highlight_changed: Default::default(),
            mouse_cursor: Default::default(),
            text_theme: Default::default(),
            text_color: Default::default(),
            disabled_text_color: Default::default(),
            color: Default::default(),
            disabled_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            highlight_color: Default::default(),
            splash_color: Default::default(),
            color_brightness: Default::default(),
            elevation: Default::default(),
            focus_elevation: Default::default(),
            hover_elevation: Default::default(),
            highlight_elevation: Default::default(),
            disabled_elevation: Default::default(),
            padding: Default::default(),
            visual_density: Default::default(),
            shape: Default::default(),
            clip_behavior: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            material_tap_target_size: Default::default(),
            animation_duration: Default::default(),
            min_width: Default::default(),
            height: Default::default(),
            enable_feedback: Default::default(),
            child: box NullWidget,
        }
    }
}

impl Widget for MaterialButton {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create MaterialButtonElement");
        box MaterialButtonElement::new(self)
    }
}

impl WidgetProperties for MaterialButton {
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
