use crate::{
    elements::{Element, ListViewElement},
    foundation::{Id, Key, WidgetProperties},
    gestures::DragStartBehavior,
    painting::EdgeInsetsGeometry,
    ui::Clip,
};

use super::{NullWidget, Widget};

pub struct ListView {
    pub key: Key,
    // pub scroll_direction: Axis, // = Axis.vertical,
    pub reverse: bool, // = false
    // pub controller: ScrollController,
    pub primary: bool,
    // pub physics: ScrollPhysics,
    pub shrink_wrap: bool, // = false
    pub padding: EdgeInsetsGeometry,
    pub item_extent: f32,
    pub prototype_item: Box<dyn Widget>,
    pub add_automatic_keep_alives: bool, // = true
    pub add_repaint_boundaries: bool,    // = true
    pub add_semantic_indexes: bool,      // = true
    pub cache_extent: f32,
    pub children: Vec<Box<dyn Widget>>,
    pub semantic_child_count: i32,
    pub drag_start_behavior: DragStartBehavior, // = DragStartBehavior.start,
    // pub keyboard_dismiss_behavior: ScrollViewKeyboardDismissBehavior, // = ScrollViewKeyboardDismissBehavior.manual,
    pub restoration_id: String,
    pub clip_behavior: Clip, // = Clip.hardEdge
}

impl Default for ListView {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // scroll_direction: Default::default(),
            reverse: Default::default(),
            // controller: Default::default(),
            primary: Default::default(),
            // physics: Default::default(),
            shrink_wrap: Default::default(),
            padding: Default::default(),
            item_extent: Default::default(),
            prototype_item: box NullWidget,
            add_automatic_keep_alives: Default::default(),
            add_repaint_boundaries: Default::default(),
            add_semantic_indexes: Default::default(),
            cache_extent: Default::default(),
            children: Default::default(),
            semantic_child_count: Default::default(),
            drag_start_behavior: Default::default(),
            // keyboard_dismiss_behavior: Default::default(),
            restoration_id: Default::default(),
            clip_behavior: Default::default(),
        }
    }
}

impl Widget for ListView {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ListViewElement");
        box ListViewElement::new(self)
    }
}

impl WidgetProperties for ListView {
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
