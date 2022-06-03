use crate::{
    elements::{Element, RowElement},
    foundation::{Id, Key, WidgetProperties},
    painting::VerticalDirection,
    rendering::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize},
    ui::{TextBaseline, TextDirection},
    widgets::Widget,
};

pub struct Row {
    pub key: Key,
    pub main_axis_alignment: MainAxisAlignment, // = MainAxisAlignment.start,
    pub main_axis_size: MainAxisSize,           // = MainAxisSize.max,
    pub cross_axis_alignment: CrossAxisAlignment, // = CrossAxisAlignment.center,
    pub text_direction: TextDirection,
    pub vertical_direction: VerticalDirection, // = VerticalDirection.down,
    pub text_baseline: TextBaseline,
    pub children: Vec<Box<dyn Widget>>,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            key: Default::default(),
            main_axis_alignment: Default::default(),
            main_axis_size: Default::default(),
            cross_axis_alignment: Default::default(),
            text_direction: Default::default(),
            vertical_direction: Default::default(),
            text_baseline: Default::default(),
            children: Default::default(),
        }
    }
}

impl Widget for Row {
    fn create_element(&self) -> Box<dyn Element> {
        box RowElement::new(self)
    }
}

impl WidgetProperties for Row {
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
