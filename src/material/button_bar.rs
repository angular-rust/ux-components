use crate::{
    elements::{ButtonBarElement, Element},
    foundation::{Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, VerticalDirection},
    rendering::{MainAxisAlignment, MainAxisSize},
    widgets::Widget,
};

use super::{ButtonBarLayoutBehavior, ButtonTextTheme};

pub struct ButtonBar {
    // pub key: Key,
    // // pub header_builder: ButtonBarHeaderBuilder,
    // pub body: Box<dyn Widget>,
    // pub is_expanded: bool,
    // pub can_tap_on_header: bool,
    // pub background_color: Color,

    // How the children should be placed along the horizontal axis.
    pub alignment: MainAxisAlignment,

    // Overrides the surrounding ButtonThemeData.alignedDropdown to define whether a DropdownButton menu's width will match the button's width.
    pub button_aligned_dropdown: bool,

    // Overrides the surrounding ButtonThemeData.height to define a button's minimum height.
    pub button_height: f32,

    // Overrides the surrounding ButtonThemeData.minWidth to define a button's minimum width.
    pub button_min_width: f32,

    // Overrides the surrounding ButtonThemeData.padding to define the padding for a button's child (typically the button's label).
    pub button_padding: Box<dyn EdgeInsetsGeometry>,

    // Overrides the surrounding ButtonBarThemeData.buttonTextTheme to define a button's base colors, size, internal padding and shape.
    pub button_text_theme: ButtonTextTheme,

    // The buttons to arrange horizontally.
    pub children: Vec<Box<dyn Widget>>,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // Defines whether a ButtonBar should size itself with a minimum size constraint or with padding.
    pub layout_behavior: ButtonBarLayoutBehavior,

    // How much horizontal space is available. See Row.mainAxisSize.
    pub main_axis_size: MainAxisSize,

    // The spacing between buttons when the button bar overflows.
    pub overflow_button_spacing: f32,

    // Defines the vertical direction of a ButtonBar's children if it overflows.
    pub overflow_direction: VerticalDirection,
}

impl Default for ButtonBar {
    fn default() -> Self {
        Self {
            alignment: Default::default(),
            button_aligned_dropdown: Default::default(),
            button_height: Default::default(),
            button_min_width: Default::default(),
            button_padding: box NoneEdgeInsetsGeometry,
            button_text_theme: Default::default(),
            children: Default::default(),
            key: Default::default(),
            layout_behavior: Default::default(),
            main_axis_size: Default::default(),
            overflow_button_spacing: Default::default(),
            overflow_direction: Default::default(),
        }
    }
}

impl Widget for ButtonBar {
    fn create_element(&self) -> Box<dyn Element> {
        box ButtonBarElement::new(self)
    }
}

impl WidgetProperties for ButtonBar {
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
