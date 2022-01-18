use crate::{foundation::Key, ui::{TextAlign, TextDirection, Locale}, painting::TextOverflow, elements::Element, properties::WidgetProperties, WidgetId};

use super::Widget;

pub struct RichText {
    pub key: Key,
    // pub text: InlineSpan,
    pub text_align: TextAlign,
    pub text_direction: TextDirection,
    pub soft_wrap: bool,
    pub overflow: TextOverflow,
    pub text_scale_factor: f32,
    pub max_lines: i32,
    pub locale: Locale,
    // pub strut_style: StrutStyle,
    // pub text_width_basis: TextWidthBasis,
    // pub text_height_behavior: TextHeightBehavior,
}

impl Default for RichText {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // text: Default::default(),
            text_align: Default::default(),
            text_direction: Default::default(),
            soft_wrap: Default::default(),
            overflow: Default::default(),
            text_scale_factor: Default::default(),
            max_lines: Default::default(),
            locale: Default::default(),
            // strut_style: Default::default(),
            // text_width_basis: Default::default(),
            // text_height_behavior: Default::default(),
        }
    }
}

impl Widget for RichText {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create RichTextElement");
        box RichTextElement::new(self)
    }
}

impl WidgetProperties for RichText {
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

    fn parent(&self) -> Option<WidgetId> {
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
