use crate::{
    elements::{Element, FlexibleSpaceBarElement},
    foundation::{Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry},
    widgets::{NoneWidget, Widget},
};

use super::{CollapseMode, StretchMode};

pub struct FlexibleSpaceBar {
    // Shown behind the title when expanded.
    pub background: Box<dyn Widget>,

    // Whether the title should be centered.
    pub center_title: bool,

    // Collapse effect while scrolling.
    pub collapse_mode: CollapseMode,

    // Defines how much the title is scaled when the FlexibleSpaceBar is expanded due to the user scrolling downwards.
    // The title is scaled uniformly on the x and y axes while maintaining its bottom-left position (bottom-center if centerTitle is true).
    pub expanded_title_scale: f32,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // Stretch effect while over-scrolling.
    pub stretch_modes: Vec<StretchMode>,

    // The primary contents of the flexible space bar when expanded.
    pub title: Box<dyn Widget>,

    // Defines how far the title is inset from either the widget's bottom-left or its center.
    pub title_padding: Box<dyn EdgeInsetsGeometry>,
}

impl Default for FlexibleSpaceBar {
    fn default() -> Self {
        Self {
            background: box NoneWidget,
            center_title: Default::default(),
            collapse_mode: Default::default(),
            expanded_title_scale: Default::default(),
            key: Default::default(),
            stretch_modes: Default::default(),
            title: box NoneWidget,
            title_padding: box NoneEdgeInsetsGeometry,
        }
    }
}

impl Widget for FlexibleSpaceBar {
    fn create_element(&self) -> Box<dyn Element> {
        box FlexibleSpaceBarElement::new(self)
    }
}

impl WidgetProperties for FlexibleSpaceBar {
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
