use crate::{
    foundation::colorspace::Color,
    painting::{BorderSide, EdgeInsetsGeometry, OutlinedBorder, TextStyle},
    ui::Brightness,
};

pub struct ChipThemeData {
    pub background_color: Color,
    pub delete_icon_color: Color,
    pub disabled_color: Color,
    pub selected_color: Color,
    pub secondary_selected_color: Color,
    pub shadow_color: Color,
    pub selected_shadow_color: Color,
    pub show_checkmark: bool,
    pub checkmark_color: Color,
    pub label_padding: EdgeInsetsGeometry,
    pub padding: EdgeInsetsGeometry,
    pub side: BorderSide,
    pub shape: OutlinedBorder,
    pub label_style: TextStyle,
    pub secondary_label_style: TextStyle,
    pub brightness: Brightness,
    pub elevation: f32,
    pub press_elevation: f32,
}

impl Default for ChipThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            delete_icon_color: Default::default(),
            disabled_color: Default::default(),
            selected_color: Default::default(),
            secondary_selected_color: Default::default(),
            shadow_color: Default::default(),
            selected_shadow_color: Default::default(),
            show_checkmark: Default::default(),
            checkmark_color: Default::default(),
            label_padding: Default::default(),
            padding: Default::default(),
            side: Default::default(),
            shape: Default::default(),
            label_style: Default::default(),
            secondary_label_style: Default::default(),
            brightness: Default::default(),
            elevation: Default::default(),
            press_elevation: Default::default(),
        }
    }
}
