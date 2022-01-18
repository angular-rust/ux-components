use crate::{
    elements::{Element, TabPageSelectorElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    widgets::Widget,
};

pub struct TabPageSelector {
    pub key: Key,
    // pub controller: TabController,
    pub indicator_size: f32,
    pub color: Color,
    pub selected_color: Color,
}

impl Default for TabPageSelector {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // controller: Default::default(),
            indicator_size: Default::default(),
            color: Default::default(),
            selected_color: Default::default(),
        }
    }
}

impl Widget for TabPageSelector {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create TabPageSelectorElement");
        box TabPageSelectorElement::new(self)
    }
}

impl WidgetProperties for TabPageSelector {
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
