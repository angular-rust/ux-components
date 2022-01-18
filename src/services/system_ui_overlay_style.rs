use crate::{foundation::colorspace::Color, ui::Brightness};

pub struct SystemUiOverlayStyle {
    pub system_navigation_bar_color: Color,
    pub system_navigation_bar_divider_color: Color,
    pub system_navigation_bar_icon_brightness: Brightness,
    pub system_navigation_bar_contrast_enforced: bool,
    pub status_bar_color: Color,
    pub status_bar_brightness: Brightness,
    pub status_bar_icon_brightness: Brightness,
    pub system_status_bar_contrast_enforced: bool,
}

impl Default for SystemUiOverlayStyle {
    fn default() -> Self {
        Self {
            system_navigation_bar_color: Default::default(),
            system_navigation_bar_divider_color: Default::default(),
            system_navigation_bar_icon_brightness: Default::default(),
            system_navigation_bar_contrast_enforced: Default::default(),
            status_bar_color: Default::default(),
            status_bar_brightness: Default::default(),
            status_bar_icon_brightness: Default::default(),
            system_status_bar_contrast_enforced: Default::default(),
        }
    }
}
