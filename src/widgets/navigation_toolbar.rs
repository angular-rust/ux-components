use crate::{
    elements::{Element, NavigationToolbarElement},
    foundation::{Id, Key, WidgetProperties},
};

use super::{NullWidget, Widget};

pub struct NavigationToolbar {
    pub key: Key,
    pub leading: Box<dyn Widget>,
    pub middle: Box<dyn Widget>,
    pub trailing: Box<dyn Widget>,
    pub center_middle: bool, // = true,
    pub middle_spacing: f32, // = kMiddleSpacing
}

impl Default for NavigationToolbar {
    fn default() -> Self {
        Self {
            key: Default::default(),
            leading: box NullWidget,
            middle: box NullWidget,
            trailing: box NullWidget,
            center_middle: Default::default(),
            middle_spacing: Default::default(),
        }
    }
}

impl Widget for NavigationToolbar {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create NavigationToolbarElement");
        box NavigationToolbarElement::new(self)
    }
}

impl WidgetProperties for NavigationToolbar {
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
