use crate::{
    foundation::colorspace::Color,
    painting::{EdgeInsetsGeometry, TextStyle, NoneEdgeInsetsGeometry},
};
pub struct MaterialBannerThemeData {
    pub background_color: Color,
    pub content_text_style: TextStyle,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub leading_padding: Box<dyn EdgeInsetsGeometry>,
}

impl Default for MaterialBannerThemeData {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            content_text_style: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            leading_padding: box NoneEdgeInsetsGeometry,
        }
    }
}
