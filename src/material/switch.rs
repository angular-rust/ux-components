use crate::{
    elements::{Element, SwitchElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    gestures::DragStartBehavior,
    painting::{ImageErrorListener, ImageProvider},
    services::MouseCursor,
    widgets::{FocusNode, Widget},
};

use super::{MaterialStateProperty, MaterialTapTargetSize};

pub struct Switch {
    pub key: Key,
    pub value: bool,
    pub on_changed: Option<Box<dyn ValueChanged<bool>>>,
    pub active_color: Color,
    pub active_track_color: Color,
    pub inactive_thumb_color: Color,
    pub inactive_track_color: Color,
    pub active_thumb_image: ImageProvider<String>,
    pub on_active_thumb_image_error: Option<Box<dyn ImageErrorListener<String>>>,
    pub inactive_thumb_image: ImageProvider<String>,
    pub on_inactive_thumb_image_error: Option<Box<dyn ImageErrorListener<String>>>,
    pub thumb_color: MaterialStateProperty<Color>,
    pub track_color: MaterialStateProperty<Color>,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub drag_start_behavior: DragStartBehavior,
    pub mouse_cursor: MouseCursor,
    pub focus_color: Color,
    pub hover_color: Color,
    pub overlay_color: MaterialStateProperty<Color>,
    pub splash_radius: f32,
    pub focus_node: FocusNode,
    pub autofocus: bool,
}

impl Default for Switch {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            on_changed: Default::default(),
            active_color: Default::default(),
            active_track_color: Default::default(),
            inactive_thumb_color: Default::default(),
            inactive_track_color: Default::default(),
            active_thumb_image: Default::default(),
            on_active_thumb_image_error: Default::default(),
            inactive_thumb_image: Default::default(),
            on_inactive_thumb_image_error: Default::default(),
            thumb_color: Default::default(),
            track_color: Default::default(),
            material_tap_target_size: Default::default(),
            drag_start_behavior: Default::default(),
            mouse_cursor: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            overlay_color: Default::default(),
            splash_radius: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
        }
    }
}

impl Widget for Switch {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create SwitchElement");
        box SwitchElement::new(self)
    }
}

impl WidgetProperties for Switch {
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
