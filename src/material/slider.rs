use crate::{
    elements::{Element, SliderElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    services::MouseCursor,
    widgets::{FocusNode, Widget},
};

pub struct Slider {
    pub key: Key,
    pub value: f32,
    pub on_changed: Option<Box<dyn ValueChanged<f32>>>,
    pub on_change_start: Option<Box<dyn ValueChanged<f32>>>,
    pub on_change_end: Option<Box<dyn ValueChanged<f32>>>,
    pub min: f32,
    pub max: f32,
    pub divisions: i32,
    pub label: String,
    pub active_color: Color,
    pub inactive_color: Color,
    pub thumb_color: Color,
    pub mouse_cursor: MouseCursor,
    // pub semantic_formatter_callback: SemanticFormatterCallback,
    pub focus_node: FocusNode,
    pub autofocus: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            on_changed: Default::default(),
            on_change_start: Default::default(),
            on_change_end: Default::default(),
            min: Default::default(),
            max: Default::default(),
            divisions: Default::default(),
            label: Default::default(),
            active_color: Default::default(),
            inactive_color: Default::default(),
            thumb_color: Default::default(),
            mouse_cursor: Default::default(),
            // semantic_formatter_callback: Default::default(),
            focus_node: Default::default(),
            autofocus: Default::default(),
        }
    }
}

impl Widget for Slider {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create SliderElement");
        box SliderElement::new(self)
    }
}

impl WidgetProperties for Slider {
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
