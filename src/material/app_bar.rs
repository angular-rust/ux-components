use crate::{
    elements::{AppBarElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{NoneShapeBorder, ShapeBorder, TextStyle},
    services::SystemUiOverlayStyle,
    ui::{Brightness, Size},
    widgets::{IconThemeData, NoneWidget, PreferredSizeWidget, Widget},
};

use super::TextTheme;

pub struct AppBar {
    pub key: Key,
    pub leading: Box<dyn Widget>,
    pub automatically_imply_leading: bool,
    pub title: Box<dyn Widget>,
    pub actions: Vec<Box<dyn Widget>>,
    pub flexible_space: Box<dyn Widget>,
    pub bottom: Box<dyn PreferredSizeWidget>,
    pub elevation: f32,
    pub shadow_color: Color,
    pub shape: Box<dyn ShapeBorder>,
    pub background_color: Color,
    pub foreground_color: Color,
    // @Deprecated("This property is no longer used, please use system_overlay_style instead. ")
    pub brightness: Brightness,
    pub icon_theme: IconThemeData,
    pub actions_icon_theme: IconThemeData,
    // @Deprecated("This property is no longer used, please use toolbar_text_style and title_text_style instead. ")
    pub text_theme: TextTheme,
    pub primary: bool,
    pub center_title: bool,
    pub exclude_header_semantics: bool,
    pub title_spacing: f32,
    pub toolbar_opacity: f32,
    pub bottom_opacity: f32,
    pub toolbar_height: f32,
    pub leading_width: f32,
    // @Deprecated("This property is obsolete and is false by default. ")
    pub backwards_compatibility: bool,
    pub toolbar_text_style: TextStyle,
    pub title_text_style: TextStyle,
    pub system_overlay_style: SystemUiOverlayStyle,
}

impl Default for AppBar {
    fn default() -> Self {
        Self {
            key: Default::default(),
            leading: box NoneWidget,
            automatically_imply_leading: Default::default(),
            title: box NoneWidget,
            actions: Default::default(),
            flexible_space: box NoneWidget,
            bottom: box NoneWidget,
            elevation: Default::default(),
            shadow_color: Default::default(),
            shape: box NoneShapeBorder,
            background_color: Default::default(),
            foreground_color: Default::default(),
            brightness: Default::default(),
            icon_theme: Default::default(),
            actions_icon_theme: Default::default(),
            text_theme: Default::default(),
            primary: Default::default(),
            center_title: Default::default(),
            exclude_header_semantics: Default::default(),
            title_spacing: Default::default(),
            toolbar_opacity: Default::default(),
            bottom_opacity: Default::default(),
            toolbar_height: 64.0,
            leading_width: Default::default(),
            backwards_compatibility: Default::default(),
            toolbar_text_style: Default::default(),
            title_text_style: Default::default(),
            system_overlay_style: Default::default(),
        }
    }
}

impl PreferredSizeWidget for AppBar {
    fn preferred_size(&self) -> Size {
        Size(0.0, self.toolbar_height) // +  bottom widget's preferred height
    }
}

impl Widget for AppBar {
    fn create_element(&self) -> Box<dyn Element> {
        box AppBarElement::new(self)
    }
}

impl WidgetProperties for AppBar {
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
