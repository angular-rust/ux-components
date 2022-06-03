use crate::{
    elements::{Element, LinearProgressIndicatorElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    widgets::Widget,
};

pub struct LinearProgressIndicator {
    pub key: Key,
    pub value: f32,
    pub background_color: Color,
    pub color: Color,
    // pub value_color: Animation<Color>,
    pub min_height: f32,
    pub semantics_label: String,
    pub semantics_value: String,
}

impl Default for LinearProgressIndicator {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            background_color: Default::default(),
            color: Default::default(),
            // value_color: Default::default(),
            min_height: Default::default(),
            semantics_label: Default::default(),
            semantics_value: Default::default(),
        }
    }
}

impl Widget for LinearProgressIndicator {
    fn create_element(&self) -> Box<dyn Element> {
        box LinearProgressIndicatorElement::new(self)
    }
}

impl WidgetProperties for LinearProgressIndicator {
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
