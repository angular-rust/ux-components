use crate::{
    foundation::colorspace::Color,
    painting::{EdgeInsetsGeometry, TextStyle},
};
pub struct MaterialBannerThemeData {
    pub background_color: Color,
    pub content_text_style: TextStyle,
    pub padding: EdgeInsetsGeometry,
    pub leading_padding: EdgeInsetsGeometry,
}

impl Default for MaterialBannerThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            content_text_style: Default::default(),
            padding: Default::default(),
            leading_padding: Default::default(),
        }
    }
}
