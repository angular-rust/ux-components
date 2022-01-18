use crate::{
    elements::{CardElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, ShapeBorder},
    ui::Clip,
    widgets::{NullWidget, Widget},
};

pub struct Card {
    pub key: Key,
    pub color: Color,
    pub shadow_color: Color,
    pub elevation: f32,
    pub shape: ShapeBorder,
    pub border_on_foreground: bool,
    pub margin: EdgeInsetsGeometry,
    pub clip_behavior: Clip,
    pub child: Box<dyn Widget>,
    pub semantic_container: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            key: Default::default(),
            color: Default::default(),
            shadow_color: Default::default(),
            elevation: Default::default(),
            shape: Default::default(),
            border_on_foreground: Default::default(),
            margin: Default::default(),
            clip_behavior: Default::default(),
            child: box NullWidget,
            semantic_container: Default::default(),
        }
    }
}

impl Widget for Card {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create CardElement");
        box CardElement::new(self)
    }
}

impl WidgetProperties for Card {
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
