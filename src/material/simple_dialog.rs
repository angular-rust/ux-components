use crate::{
    elements::{Element, SimpleDialogElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, ShapeBorder, TextStyle},
    ui::Clip,
    widgets::{NullWidget, Widget},
};

pub struct SimpleDialog {
    pub key: Key,
    pub title: Box<dyn Widget>,
    pub title_padding: EdgeInsetsGeometry,
    pub title_text_style: TextStyle,
    pub children: Vec<Box<dyn Widget>>,
    pub content_padding: EdgeInsetsGeometry,
    pub background_color: Color,
    pub elevation: f32,
    pub semantic_label: String,
    // pub inset_padding: EdgeInsets,
    pub clip_behavior: Clip,
    pub shape: ShapeBorder,
}

impl Default for SimpleDialog {
    fn default() -> Self {
        Self {
            key: Default::default(),
            title: box NullWidget,
            title_padding: Default::default(),
            title_text_style: Default::default(),
            children: Default::default(),
            content_padding: Default::default(),
            background_color: Default::default(),
            elevation: Default::default(),
            semantic_label: Default::default(),
            // inset_padding: Default::default(),
            clip_behavior: Default::default(),
            shape: Default::default(),
        }
    }
}

impl Widget for SimpleDialog {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create SimpleDialogElement");
        box SimpleDialogElement::new(self)
    }
}

impl WidgetProperties for SimpleDialog {
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
