use crate::{
    elements::{CustomScrollViewElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget, ui::Clip, gestures::DragStartBehavior, painting::Axis,
};

use super::ScrollBehavior;

pub struct CustomScrollView {
    // The relative position of the zero scroll offset.
    pub anchor: f32,
    
    // The viewport has an area before and after the visible area to cache items that are about to become visible when the user scrolls.
    pub cache_extent: f32,
    
    // The first child in the GrowthDirection.forward growth direction.
    pub center: Option<Key>,
    
    // The content will be clipped (or not) according to this option.
    pub clip_behavior: Clip,
    
    // An object that can be used to control the position to which this scroll view is scrolled.
    // pub controller: ScrollController,
    
    // Determines the way that drag start behavior is handled.
    pub drag_start_behavior: DragStartBehavior,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // ScrollViewKeyboardDismissBehavior the defines how this ScrollView will dismiss the keyboard automatically.
    // pub keyboard_dismiss_behavior: ScrollViewKeyboardDismissBehavior,
    
    // How the scroll view should respond to user input.
    // pub physics: ScrollPhysics,
    
    // Whether this is the primary scroll view associated with the parent PrimaryScrollController.
    pub primary: bool,
    
    // Restoration ID to save and restore the scroll offset of the scrollable.
    pub restoration_id: Option<String>,
    
    // Whether the scroll view scrolls in the reading direction.
    pub reverse: bool,
    
    // A ScrollBehavior that will be applied to this widget individually.
    pub scroll_behavior: Option<ScrollBehavior>,
    
    // The axis along which the scroll view scrolls.
    pub scroll_direction: Axis,
    
    // The number of children that will contribute semantic information.
    pub semantic_child_count: Option<usize>,
    
    // Whether the extent of the scroll view in the scrollDirection should be determined by the contents being viewed.
    pub shrink_wrap: bool,
    
    // The slivers to place inside the viewport.
    pub slivers: Vec<Box<dyn Widget>>,
}

impl Default for CustomScrollView {
    fn default() -> Self {
        Self {
            anchor: Default::default(),
            cache_extent: Default::default(),
            center: Default::default(),
            clip_behavior: Default::default(),
            // controller: Default::default(),
            drag_start_behavior: Default::default(),
            key: Default::default(),
            // keyboard_dismiss_behavior: Default::default(),
            // physics: Default::default(),
            primary: Default::default(),
            restoration_id: Default::default(),
            reverse: Default::default(),
            scroll_behavior: Default::default(),
            scroll_direction: Default::default(),
            semantic_child_count: Default::default(),
            shrink_wrap: Default::default(),
            slivers: Default::default(),
            // key: Default::default(),
            // width_factor: Default::default(),
            // height_factor: Default::default(),
            // child: box NoneWidget,
            
        }
    }
}

impl Widget for CustomScrollView {
    fn create_element(&self) -> Box<dyn Element> {
        box CustomScrollViewElement::new(self)
    }
}

impl WidgetProperties for CustomScrollView {
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
