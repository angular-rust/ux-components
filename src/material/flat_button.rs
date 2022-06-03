use crate::{
    elements::{Element, FlatButtonElement},
    foundation::{Id, Key, WidgetProperties},
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::ButtonStyle;

// defaultStyleOf(BuildContext context) → ButtonStyle
// Defines the button's default appearance.
// override
//
// themeStyleOf(BuildContext context) → ButtonStyle?
// Returns the ButtonStyle that belongs to the button's component theme.
// override

#[deprecated(note = "This class is deprecated, please use TextButton instead.")]
pub struct FlatButton {
    pub key: Key,
    pub on_pressed: Option<VoidCallback>,
    pub on_long_press: Option<VoidCallback>,
    pub style: ButtonStyle,
    pub focus_node: FocusNode,
    pub autofocus: bool,     // = false,
    pub clip_behavior: Clip, // = Clip.none,
    pub child: Box<dyn Widget>,
}

#[allow(deprecated)]
impl Default for FlatButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            on_pressed: Default::default(),
            on_long_press: Default::default(),
            style: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            clip_behavior: Default::default(),
            child: box NoneWidget,
        }
    }
}

#[allow(deprecated)]
impl Widget for FlatButton {
    fn create_element(&self) -> Box<dyn Element> {
        box FlatButtonElement::new(self)
    }
}

#[allow(deprecated)]
impl WidgetProperties for FlatButton {
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
