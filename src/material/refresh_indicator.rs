use crate::{
    elements::{Element, RefreshIndicatorElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    widgets::{NullWidget, Widget},
};

pub struct RefreshIndicator {
    pub key: Key,
    pub child: Box<dyn Widget>,
    pub displacement: f32,
    pub edge_offset: f32,
    // pub on_refresh: RefreshCallback,
    pub color: Color,
    pub background_color: Color,
    // pub notification_predicate: ScrollNotificationPredicate,
    pub semantics_label: String,
    pub semantics_value: String,
    pub stroke_width: f32,
    // pub trigger_mode: RefreshIndicatorTriggerMode,
}

impl Default for RefreshIndicator {
    fn default() -> Self {
        Self {
            key: Default::default(),
            child: box NullWidget,
            displacement: Default::default(),
            edge_offset: Default::default(),
            // on_refresh: Default::default(),
            color: Default::default(),
            background_color: Default::default(),
            // notification_predicate: Default::default(),
            semantics_label: Default::default(),
            semantics_value: Default::default(),
            stroke_width: Default::default(),
            // trigger_mode: Default::default(),
        }
    }
}

impl Widget for RefreshIndicator {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create RefreshIndicatorElement");
        box RefreshIndicatorElement::new(self)
    }
}

impl WidgetProperties for RefreshIndicator {
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
