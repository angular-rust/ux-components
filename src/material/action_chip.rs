use crate::{
    elements::{ActionChipElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{
        BorderSide, EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneOutlinedBorder, OutlinedBorder,
        TextStyle,
    },
    ui::{Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::{MaterialTapTargetSize, VisualDensity};
pub struct ActionChip {
    pub key: Key,
    pub avatar: Box<dyn Widget>,
    pub label: Box<dyn Widget>,
    pub label_style: TextStyle,
    pub label_padding: Box<dyn EdgeInsetsGeometry>,
    pub delete_icon: Box<dyn Widget>,
    pub on_deleted: Option<VoidCallback>,
    pub delete_icon_color: Color,
    pub use_delete_button_tooltip: bool,
    pub delete_button_tooltip_message: String,
    pub side: BorderSide,
    pub shape: Box<dyn OutlinedBorder>,
    pub clip_behavior: Clip,
    pub focus_node: FocusNode,
    pub autofocus: bool,
    pub background_color: Color,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub visual_density: VisualDensity,
    pub material_tap_target_size: MaterialTapTargetSize,
    pub elevation: f32,
    pub shadow_color: Color,
}

impl Default for ActionChip {
    fn default() -> Self {
        Self {
            key: Default::default(),
            avatar: box NoneWidget,
            label: box NoneWidget,
            label_style: Default::default(),
            label_padding: box NoneEdgeInsetsGeometry,
            delete_icon: box NoneWidget,
            on_deleted: Default::default(),
            delete_icon_color: Default::default(),
            use_delete_button_tooltip: Default::default(),
            delete_button_tooltip_message: Default::default(),
            side: Default::default(),
            shape: box NoneOutlinedBorder,
            clip_behavior: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
            background_color: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            visual_density: Default::default(),
            material_tap_target_size: Default::default(),
            elevation: Default::default(),
            shadow_color: Default::default(),
        }
    }
}

impl Widget for ActionChip {
    fn create_element(&self) -> Box<dyn Element> {
        box ActionChipElement::new(self)
    }
}

impl WidgetProperties for ActionChip {
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
