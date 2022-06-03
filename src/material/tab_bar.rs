use crate::{
    elements::{Element, TabBarElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    gestures::DragStartBehavior,
    painting::{Decoration, EdgeInsetsGeometry, NoneDecoration, NoneEdgeInsetsGeometry, TextStyle},
    services::MouseCursor,
    widgets::Widget,
};

use super::{MaterialStateProperty, TabBarIndicatorSize};

pub struct TabBar {
    pub key: Key,
    pub tabs: Vec<Box<dyn Widget>>,
    // pub controller: TabController,
    pub is_scrollable: bool,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub indicator_color: Color,
    pub automatic_indicator_color_adjustment: bool,
    pub indicator_weight: f32,
    pub indicator_padding: Box<dyn EdgeInsetsGeometry>,
    pub indicator: Box<dyn Decoration>,
    pub indicator_size: TabBarIndicatorSize,
    pub label_color: Color,
    pub label_style: TextStyle,
    pub label_padding: Box<dyn EdgeInsetsGeometry>,
    pub unselected_label_color: Color,
    pub unselected_label_style: TextStyle,
    pub drag_start_behavior: DragStartBehavior,
    pub overlay_color: MaterialStateProperty<Color>,
    pub mouse_cursor: MouseCursor,
    pub enable_feedback: bool,
    pub on_tap: Option<ValueChanged<i32>>,
    // pub physics: ScrollPhysics,
}

impl Default for TabBar {
    fn default() -> Self {
        Self {
            key: Default::default(),
            tabs: Default::default(),
            // controller: Default::default(),
            is_scrollable: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            indicator_color: Default::default(),
            automatic_indicator_color_adjustment: Default::default(),
            indicator_weight: Default::default(),
            indicator_padding: box NoneEdgeInsetsGeometry,
            indicator: box NoneDecoration,
            indicator_size: Default::default(),
            label_color: Default::default(),
            label_style: Default::default(),
            label_padding: box NoneEdgeInsetsGeometry,
            unselected_label_color: Default::default(),
            unselected_label_style: Default::default(),
            drag_start_behavior: Default::default(),
            overlay_color: Default::default(),
            mouse_cursor: Default::default(),
            enable_feedback: Default::default(),
            on_tap: Default::default(),
            // physics: Default::default(),
        }
    }
}

impl Widget for TabBar {
    fn create_element(&self) -> Box<dyn Element> {
        box TabBarElement::new(self)
    }
}

impl WidgetProperties for TabBar {
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
