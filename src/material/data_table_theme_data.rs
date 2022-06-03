use crate::{
    foundation::colorspace::Color,
    painting::{Decoration, NoneDecoration, TextStyle},
};

use super::MaterialStateProperty;

pub struct DataTableThemeData {
    pub decoration: Box<dyn Decoration>,
    pub data_row_color: MaterialStateProperty<Color>,
    pub data_row_height: f32,
    pub data_text_style: TextStyle,
    pub heading_row_color: MaterialStateProperty<Color>,
    pub heading_row_height: f32,
    pub heading_text_style: TextStyle,
    pub horizontal_margin: f32,
    pub column_spacing: f32,
    pub divider_thickness: f32,
    pub checkbox_horizontal_margin: f32,
}

impl Default for DataTableThemeData {
    fn default() -> Self {
        Self {
            decoration: box NoneDecoration,
            data_row_color: Default::default(),
            data_row_height: Default::default(),
            data_text_style: Default::default(),
            heading_row_color: Default::default(),
            heading_row_height: Default::default(),
            heading_text_style: Default::default(),
            horizontal_margin: Default::default(),
            column_spacing: Default::default(),
            divider_thickness: Default::default(),
            checkbox_horizontal_margin: Default::default(),
        }
    }
}
