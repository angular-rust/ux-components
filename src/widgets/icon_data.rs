/// A description of an icon fulfilled by a font glyph.
///
/// See [Icons] for a number of predefined icons available for material design applications.
///
/// Originally it contain additional fields like a fontFamily, fontPackage and matchTextDirection.
/// We do not use them becouse we use different rendering model
///
/// Inner is the Unicode code point at which this icon is stored in the icon font.

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IconData(pub i32);

impl IconData {
    pub fn code(&self) -> i32 {
        self.0
    }
}
