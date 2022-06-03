use crate::{
    elements::{Element, ElevatedButtonElement},
    foundation::{Id, Key, WidgetProperties, ValueChanged},
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::ButtonStyle;

// defaultStyleOf(BuildContext context): ButtonStyle
// Defines the button's default appearance.
// override
//
// themeStyleOf(BuildContext context): ButtonStyle?
// Returns the ElevatedButtonThemeData.style of the closest ElevatedButtonTheme ancestor.
// override

pub struct ElevatedButton {
    // True if this widget will be selected as the initial focus when no other node in its scope is currently focused.
    pub autofocus: bool,
    
    // Typically the button's label.
    pub child: Box<dyn Widget>,
    
    // The content will be clipped (or not) according to this option.
    pub clip_behavior: Clip,
    
    // Whether the button is enabled or disabled.
    pub enabled: bool,
    
    // An optional focus node to use as the focus node for this widget.
    pub focus_node: FocusNode,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // Handler called when the focus changes.
    pub on_focus_change: Option<ValueChanged<bool>>,
    
    // Called when a pointer enters or exits the button response area.
    pub on_hover: Option<ValueChanged<bool>>,
    
    // Called when the button is long-pressed.
    pub on_long_press: Option<VoidCallback>,
    
    // Called when the button is tapped or otherwise activated.
    pub on_pressed: Option<VoidCallback>,
    
    // Customizes this button's appearance.
    pub style: ButtonStyle,
}

impl Default for ElevatedButton {
    fn default() -> Self {
        Self {
            autofocus: Default::default(),
            child: box NoneWidget,
            clip_behavior: Default::default(),
            enabled: Default::default(),
            focus_node: Default::default(),
            key: Default::default(),
            on_focus_change: Default::default(),
            on_hover: Default::default(),
            on_long_press: Default::default(),
            on_pressed: Default::default(),
            style: Default::default(),            
        }
    }
}

impl Widget for ElevatedButton {
    fn create_element(&self) -> Box<dyn Element> {
        box ElevatedButtonElement::new(self)
    }
}

impl WidgetProperties for ElevatedButton {
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
