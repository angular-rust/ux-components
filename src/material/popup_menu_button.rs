use crate::{
    elements::{Element, PopupMenuButtonElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{EdgeInsetsGeometry, ShapeBorder},
    ui::Offset,
    widgets::{NullWidget, Widget},
};

pub struct PopupMenuButton {
    pub key: Key,
    // pub item_builder: PopupMenuItemBuilder<T>,
    // pub initial_value: T,
    // pub on_selected: PopupMenuItemSelected<T>,
    // pub on_canceled: PopupMenuCanceled,
    pub tooltip: String,
    pub elevation: f32,
    pub padding: EdgeInsetsGeometry,
    pub child: Box<dyn Widget>,
    pub icon: Box<dyn Widget>,
    pub icon_size: f32,
    pub offset: Offset,
    pub enabled: bool,
    pub shape: ShapeBorder,
    pub color: Color,
    pub enable_feedback: bool,
}

impl Default for PopupMenuButton {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // item_builder: Default::default(),
            // initial_value: Default::default(),
            // on_selected: Default::default(),
            // on_canceled: Default::default(),
            tooltip: Default::default(),
            elevation: Default::default(),
            padding: Default::default(),
            child: box NullWidget,
            icon: box NullWidget,
            icon_size: Default::default(),
            offset: Default::default(),
            enabled: Default::default(),
            shape: Default::default(),
            color: Default::default(),
            enable_feedback: Default::default(),
        }
    }
}

impl Widget for PopupMenuButton {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create PopupMenuButtonElement");
        box PopupMenuButtonElement::new(self)
    }
}

impl WidgetProperties for PopupMenuButton {
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
