use crate::{
    elements::Element, properties::WidgetProperties, services::MouseCursor, ui::Locale,
    widgets::Widget, WidgetId,
};

use super::TextStyle;

pub struct TextSpan {
    pub text: String,
    // pub children: Vec<InlineSpan>,
    pub style: TextStyle,
    // pub recognizer: GestureRecognizer,
    pub mouse_cursor: MouseCursor,
    // pub on_enter: PointerEnterEventListener,
    // pub on_exit: PointerExitEventListener,
    pub semantics_label: String,
    pub locale: Locale,
    pub spell_out: bool,
}

impl Default for TextSpan {
    fn default() -> Self {
        Self {
            text: Default::default(),
            // children: Default::default(),
            style: Default::default(),
            // recognizer: Default::default(),
            mouse_cursor: Default::default(),
            // on_enter: Default::default(),
            // on_exit: Default::default(),
            semantics_label: Default::default(),
            locale: Default::default(),
            spell_out: Default::default(),
        }
    }
}

impl Widget for TextSpan {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create TextSpanElement");
        box TextSpanElement::new(self)
    }
}

impl WidgetProperties for TextSpan {
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
