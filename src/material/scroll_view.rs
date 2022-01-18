use crate::{
    elements::{Element, ScrollViewElement},
    foundation::{Id, Key, WidgetProperties},
    gestures::DragStartBehavior,
    ui::Clip,
    widgets::{ScrollBehavior, Widget},
};

pub struct ScrollView {
    pub key: Key,
    // pub scroll_direction: Axis,
    pub reverse: bool,
    // pub controller: ScrollController,
    pub primary: bool,
    // pub physics: ScrollPhysics,
    pub scroll_behavior: ScrollBehavior,
    pub shrink_wrap: bool,
    pub center: Key,
    pub anchor: f32,
    pub cache_extent: f32,
    pub semantic_child_count: i32,
    pub drag_start_behavior: DragStartBehavior,
    // pub keyboard_dismiss_behavior: ScrollViewKeyboardDismissBehavior,
    pub restoration_id: String,
    pub clip_behavior: Clip,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // scroll_direction: Default::default(),
            reverse: Default::default(),
            // controller: Default::default(),
            primary: Default::default(),
            // physics: Default::default(),
            scroll_behavior: Default::default(),
            shrink_wrap: Default::default(),
            center: Default::default(),
            anchor: Default::default(),
            cache_extent: Default::default(),
            semantic_child_count: Default::default(),
            drag_start_behavior: Default::default(),
            // keyboard_dismiss_behavior: Default::default(),
            restoration_id: Default::default(),
            clip_behavior: Default::default(),
        }
    }
}

impl Widget for ScrollView {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ScrollViewElement");
        box ScrollViewElement::new(self)
    }
}

impl WidgetProperties for ScrollView {
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
