use crate::{
    elements::{Element, NavigationRailDestinationElement},
    foundation::{Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry},
    widgets::{NoneWidget, Widget},
};

pub struct NavigationRailDestination {
    pub key: Key,
    pub icon: Box<dyn Widget>,
    pub selected_icon: Box<dyn Widget>,
    pub label: Box<dyn Widget>,
    pub padding: Box<dyn EdgeInsetsGeometry>,
}

impl Default for NavigationRailDestination {
    fn default() -> Self {
        Self {
            key: Default::default(),
            icon: box NoneWidget,
            selected_icon: box NoneWidget,
            label: box NoneWidget,
            padding: box NoneEdgeInsetsGeometry,
        }
    }
}

impl Widget for NavigationRailDestination {
    fn create_element(&self) -> Box<dyn Element> {
        box NavigationRailDestinationElement::new(self)
    }
}

impl WidgetProperties for NavigationRailDestination {
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
