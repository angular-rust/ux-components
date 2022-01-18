use crate::{
    elements::{ButtonStyleButtonElement, Element},
    foundation::{Id, Key, WidgetProperties},
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NullWidget, Widget},
};

use super::ButtonStyle;

pub struct ButtonStyleButton {
    pub key: Key,
    pub on_pressed: Option<Box<dyn VoidCallback>>,
    pub on_long_press: Option<Box<dyn VoidCallback>>,
    pub style: ButtonStyle,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub clip_behavior: Clip,
    pub child: Box<dyn Widget>,
}

impl Default for ButtonStyleButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            on_pressed: Default::default(),
            on_long_press: Default::default(),
            style: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            clip_behavior: Default::default(),
            child: box NullWidget,
        }
    }
}

impl Widget for ButtonStyleButton {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ButtonStyleButtonElement");
        box ButtonStyleButtonElement::new(self)
    }
}

impl WidgetProperties for ButtonStyleButton {
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
