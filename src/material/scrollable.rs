use crate::{
    elements::{Element, ScrollableElement},
    foundation::{Id, Key, WidgetProperties},
    gestures::DragStartBehavior,
    widgets::{ScrollBehavior, Widget},
};

pub struct Scrollable {
    pub key: Key,
    // pub axis_direction: AxisDirection,
    // pub controller: ScrollController,
    // pub physics: ScrollPhysics,
    // pub viewport_builder: ViewportBuilder,
    // pub increment_calculator: ScrollIncrementCalculator,
    pub exclude_from_semantics: bool,
    pub semantic_child_count: i32,
    pub drag_start_behavior: DragStartBehavior,
    pub restoration_id: String,
    pub scroll_behavior: ScrollBehavior,
}

impl Default for Scrollable {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // axis_direction: Default::default(),
            // controller: Default::default(),
            // physics: Default::default(),
            // viewport_builder: Default::default(),
            // increment_calculator: Default::default(),
            exclude_from_semantics: Default::default(),
            semantic_child_count: Default::default(),
            drag_start_behavior: Default::default(),
            restoration_id: Default::default(),
            scroll_behavior: Default::default(),
        }
    }
}

impl Widget for Scrollable {
    fn create_element(&self) -> Box<dyn Element> {
        box ScrollableElement::new(self)
    }
}

impl WidgetProperties for Scrollable {
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
