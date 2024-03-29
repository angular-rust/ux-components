use crate::{
    elements::{Element, ExpansionPanelElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    widgets::{NoneWidget, Widget},
};

pub struct ExpansionPanel {
    pub key: Key,
    // pub header_builder: ExpansionPanelHeaderBuilder,
    pub body: Box<dyn Widget>,
    pub is_expanded: bool,
    pub can_tap_on_header: bool,
    pub background_color: Color,
}

impl Default for ExpansionPanel {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // header_builder: Default::default(),
            body: box NoneWidget,
            is_expanded: Default::default(),
            can_tap_on_header: Default::default(),
            background_color: Default::default(),
        }
    }
}

impl Widget for ExpansionPanel {
    fn create_element(&self) -> Box<dyn Element> {
        box ExpansionPanelElement::new(self)
    }
}

impl WidgetProperties for ExpansionPanel {
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
