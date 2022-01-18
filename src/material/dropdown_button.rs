use crate::{
    elements::{DropdownButtonElement, Element},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{BorderRadius, TextStyle},
    ui::VoidCallback,
    widgets::{FocusNode, NullWidget, Widget},
};

use super::AlignmentGeometry;
pub struct DropdownButton<T: Default> {
    pub key: Key,
    // pub items: Vec<DropdownMenuItem<T>>,
    // pub selected_item_builder: DropdownButtonBuilder,
    pub value: T,
    pub hint: Box<dyn Widget>,
    pub disabled_hint: Box<dyn Widget>,
    pub on_changed: Option<Box<dyn ValueChanged<T>>>,
    pub on_tap: Option<Box<dyn VoidCallback>>,
    pub elevation: i32,
    pub style: TextStyle,
    pub underline: Box<dyn Widget>,
    pub icon: Box<dyn Widget>,
    pub icon_disabled_color: Color,
    pub icon_enabled_color: Color,
    pub icon_size: f32,
    pub is_dense: bool,
    pub is_expanded: bool,
    pub item_height: f32,
    pub focus_color: Color,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub dropdown_color: Color,
    pub menu_max_height: f32,
    pub enable_feedback: bool,
    pub alignment: AlignmentGeometry,
    pub border_radius: BorderRadius,
}

impl<T: Default> Default for DropdownButton<T> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // items: Default::default(),
            // selected_item_builder: Default::default(),
            value: Default::default(),
            hint: box NullWidget,
            disabled_hint: box NullWidget,
            on_changed: Default::default(),
            on_tap: Default::default(),
            elevation: Default::default(),
            style: Default::default(),
            underline: box NullWidget,
            icon: box NullWidget,
            icon_disabled_color: Default::default(),
            icon_enabled_color: Default::default(),
            icon_size: Default::default(),
            is_dense: Default::default(),
            is_expanded: Default::default(),
            item_height: Default::default(),
            focus_color: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            dropdown_color: Default::default(),
            menu_max_height: Default::default(),
            enable_feedback: Default::default(),
            alignment: Default::default(),
            border_radius: Default::default(),
        }
    }
}

impl<T: Default> Widget for DropdownButton<T> {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create DropdownButtonElement");
        box DropdownButtonElement::new(self)
    }
}

impl<T: Default> WidgetProperties for DropdownButton<T> {
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
