use crate::{
    foundation::colorspace::Color,
    painting::{ShapeBorder, TextStyle, NoneShapeBorder},
    services::SystemUiOverlayStyle,
    ui::Brightness,
    widgets::IconThemeData,
};

use super::TextTheme;

pub struct AppBarTheme {
    // @Deprecated("This property is no longer used, please use systemOverlayStyle instead. ")
    pub brightness: Brightness,
    pub color: Color,
    pub background_color: Color,
    pub foreground_color: Color,
    pub elevation: f32,
    pub shadow_color: Color,
    pub shape: Box<dyn ShapeBorder>,
    pub icon_theme: IconThemeData,
    pub actions_icon_theme: IconThemeData,
    // @Deprecated("This property is no longer used, please use toolbarTextStyle and titleTextStyle instead. ")
    pub text_theme: TextTheme,
    pub center_title: bool,
    pub title_spacing: f32,
    pub toolbar_height: f32,
    pub toolbar_text_style: TextStyle,
    pub title_text_style: TextStyle,
    pub system_overlay_style: SystemUiOverlayStyle,
    // @Deprecated("This property is obsolete and is false by default. ")
    pub backwards_compatibility: bool,
}

impl Default for AppBarTheme {
    fn default() -> Self {
        Self {
            brightness: Default::default(),
            color: Default::default(),
            background_color: Default::default(),
            foreground_color: Default::default(),
            elevation: Default::default(),
            shadow_color: Default::default(),
            shape: box NoneShapeBorder,
            icon_theme: Default::default(),
            actions_icon_theme: Default::default(),
            text_theme: Default::default(),
            center_title: Default::default(),
            title_spacing: Default::default(),
            toolbar_height: Default::default(),
            toolbar_text_style: Default::default(),
            title_text_style: Default::default(),
            system_overlay_style: Default::default(),
            backwards_compatibility: Default::default(),
        }
    }
}
