use crate::foundation::colorspace::Color;

// merge(IconThemeData? other) -> IconThemeData
// Returns a new icon theme that matches this icon theme but with some values replaced by the non-null parameters of the given icon theme. If the given icon theme is null, simply returns this icon theme.
//
// resolve(BuildContext context) -> IconThemeData
// Called by IconTheme.of to convert this instance to an IconThemeData that fits the given BuildContext.

pub struct IconThemeData {
    color: Color,
    opacity: f32,
    size: f32,
}

impl Default for IconThemeData {
    fn default() -> Self {
        Self {
            color: Default::default(),
            opacity: Default::default(),
            size: Default::default(),
        }
    }
}
