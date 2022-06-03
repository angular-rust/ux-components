use crate::{elements::Element, properties::WidgetProperties, widgets::Widget, WidgetId};

pub struct TabController {
    pub initial_index: i32, // = 0
    pub length: i32,
    // pub vsync: TickerProvider,
}

impl Default for TabController {
    fn default() -> Self {
        Self {
            initial_index: Default::default(),
            length: Default::default(),
            // vsync: Default::default(),
        }
    }
}

impl Widget for TabController {
    fn create_element(&self) -> Box<dyn Element> {
        box TabControllerElement::new(self)
    }
}

impl WidgetProperties for TabController {
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

    fn parent(&self) -> Option<WidgetId> {
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
