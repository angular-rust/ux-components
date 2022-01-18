use crate::foundation::colorspace::Color;

// shade50 → Color
// The lightest shade.
// read-only
//
// shade100 → Color
// The second lightest shade.
// read-only
//
// shade200 → Color
// The third lightest shade.
// read-only
//
// shade300 → Color
// The fourth lightest shade.
// read-only
//
// shade400 → Color
// The fifth lightest shade.
// read-only
//
// shade500 → Color
// The default shade.
// read-only
//
// shade600 → Color
// The fourth darkest shade.
// read-only
//
// shade700 → Color
// The third darkest shade.
// read-only
//
// shade800 → Color
// The second darkest shade.
// read-only
//
// shade900 → Color
// The darkest shade.
// read-only

/// Defines a single color as well a color swatch with ten shades of the color.
// int primary, Map<int, Color> swatch
pub struct MaterialColor(pub Color);

impl Default for MaterialColor {
    fn default() -> Self {
        Self(Default::default())
    }
}
