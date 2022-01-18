use crate::{
    elements::{Element, TabBarViewElement},
    foundation::{Id, Key, WidgetProperties},
    gestures::DragStartBehavior,
    widgets::Widget,
};

pub struct TabBarView {
    pub key: Key,
    pub children: Vec<Box<dyn Widget>>,
    // pub controller: TabController,
    // pub physics: ScrollPhysics,
    pub drag_start_behavior: DragStartBehavior,
}

impl Default for TabBarView {
    fn default() -> Self {
        Self {
            key: Default::default(),
            children: Default::default(),
            // controller: Default::default(),
            // physics: Default::default(),
            drag_start_behavior: Default::default(),
        }
    }
}

impl Widget for TabBarView {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create TabBarViewElement");
        box TabBarViewElement::new(self)
    }
}

impl WidgetProperties for TabBarView {
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
