use crate::{
    elements::{AlertDialogElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{
        EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder, TextStyle,
        VerticalDirection,
    },
    rendering::MainAxisAlignment,
    ui::Clip,
    widgets::{NoneWidget, Widget},
};

pub struct AlertDialog {
    pub key: Key,
    pub title: Box<dyn Widget>,
    pub title_padding: Box<dyn EdgeInsetsGeometry>,
    pub title_text_style: TextStyle,
    pub content: Box<dyn Widget>,
    pub content_padding: Box<dyn EdgeInsetsGeometry>,
    pub content_text_style: TextStyle,
    pub actions: Vec<Box<dyn Widget>>,
    pub actions_padding: Box<dyn EdgeInsetsGeometry>,
    pub actions_alignment: MainAxisAlignment,
    pub actions_overflow_direction: VerticalDirection,
    pub actions_overflow_button_spacing: f32,
    pub button_padding: Box<dyn EdgeInsetsGeometry>,
    pub background_color: Color,
    pub elevation: f32,
    pub semantic_label: String,
    // pub inset_padding: EdgeInsets,
    pub clip_behavior: Clip,
    pub shape: Box<dyn ShapeBorder>,
    pub scrollable: bool,
}

impl Default for AlertDialog {
    fn default() -> Self {
        Self {
            key: Default::default(),
            title: box NoneWidget,
            title_padding: box NoneEdgeInsetsGeometry,
            title_text_style: Default::default(),
            content: box NoneWidget,
            content_padding: box NoneEdgeInsetsGeometry,
            content_text_style: Default::default(),
            actions: Default::default(),
            actions_padding: box NoneEdgeInsetsGeometry,
            actions_alignment: Default::default(),
            actions_overflow_direction: Default::default(),
            actions_overflow_button_spacing: Default::default(),
            button_padding: box NoneEdgeInsetsGeometry,
            background_color: Default::default(),
            elevation: Default::default(),
            semantic_label: Default::default(),
            // inset_padding: Default::default(),
            clip_behavior: Default::default(),
            shape: box NoneShapeBorder,
            scrollable: Default::default(),
        }
    }
}

impl Widget for AlertDialog {
    fn create_element(&self) -> Box<dyn Element> {
        box AlertDialogElement::new(self)
    }
}

impl WidgetProperties for AlertDialog {
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
