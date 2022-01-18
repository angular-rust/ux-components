use crate::{
    elements::{Element, TextElement},
    foundation::{Id, Key, WidgetProperties},
    painting::{TextOverflow, TextStyle},
    ui::{Locale, TextAlign, TextDirection},
    widgets::Widget,
};

pub struct Text {
    pub data: String,
    pub key: Key,
    pub style: TextStyle,
    // pub strut_style: StrutStyle,// TODO:
    pub text_align: TextAlign,
    pub text_direction: TextDirection,
    pub locale: Locale,
    pub soft_wrap: bool,
    pub overflow: TextOverflow,
    pub text_scale_factor: f32,
    pub max_lines: i32,
    pub semantics_label: String,
    // pub text_width_basis: TextWidthBasis, // TODO:
    // pub text_height_behavior: TextHeightBehavior,// TODO:
}

impl Default for Text {
    fn default() -> Self {
        Self {
            data: Default::default(),
            key: Default::default(),
            style: Default::default(),
            // strut_style: Default::default(),
            text_align: Default::default(),
            text_direction: Default::default(),
            locale: Default::default(),
            soft_wrap: Default::default(),
            overflow: Default::default(),
            text_scale_factor: Default::default(),
            max_lines: Default::default(),
            semantics_label: Default::default(),
            // text_width_basis: Default::default(),
            // text_height_behavior: Default::default(),
        }
    }
}

impl Widget for Text {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create TextElement");
        box TextElement::new(self)
    }
}

impl WidgetProperties for Text {
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
