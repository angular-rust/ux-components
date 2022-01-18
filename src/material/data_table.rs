use crate::{
    elements::{DataTableElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{Decoration, TextStyle},
    widgets::Widget,
};

use super::MaterialStateProperty;

pub struct DataTable {
    pub key: Key,
    // pub columns: Vec<DataColumn>,
    pub sort_column_index: i32,
    pub sort_ascending: bool,
    // pub on_select_all: ValueSetter<bool>,
    pub decoration: Decoration,
    pub data_row_color: MaterialStateProperty<Color>,
    pub data_row_height: f32,
    pub data_text_style: TextStyle,
    pub heading_row_color: MaterialStateProperty<Color>,
    pub heading_row_height: f32,
    pub heading_text_style: TextStyle,
    pub horizontal_margin: f32,
    pub column_spacing: f32,
    pub show_checkbox_column: bool,
    pub show_bottom_border: bool,
    pub divider_thickness: f32,
    // pub rows: Vec<DataRow>,
    pub checkbox_horizontal_margin: f32,
}

impl Default for DataTable {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // columns: Default::default(),
            sort_column_index: Default::default(),
            sort_ascending: Default::default(),
            // on_select_all: Default::default(),
            decoration: Default::default(),
            data_row_color: Default::default(),
            data_row_height: Default::default(),
            data_text_style: Default::default(),
            heading_row_color: Default::default(),
            heading_row_height: Default::default(),
            heading_text_style: Default::default(),
            horizontal_margin: Default::default(),
            column_spacing: Default::default(),
            show_checkbox_column: Default::default(),
            show_bottom_border: Default::default(),
            divider_thickness: Default::default(),
            // rows: Default::default(),
            checkbox_horizontal_margin: Default::default(),
        }
    }
}

impl Widget for DataTable {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create DataTableElement");
        box DataTableElement::new(self)
    }
}

impl WidgetProperties for DataTable {
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
