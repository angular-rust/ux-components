#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialType {
    // Rectangle using default theme canvas color.
    Canvas = 0,
    // Rounded edges, card theme color.
    Card = 1,
    // A circle, no color by default (used for floating action buttons).
    Circle = 2,
    // Rounded edges, no color by default (used for MaterialButton buttons).
    Button = 3,
    // A transparent piece of material that draws ink splashes and highlights.
    // While the material metaphor describes child widgets as printed on the material 
    // itself and do not hide ink effects, in practice the Material widget draws 
    // child widgets on top of the ink effects. A Material with type transparency 
    // can be placed on top of opaque widgets to show ink effects on top of them.
    // Prefer using the Ink widget for showing ink effects on top of opaque widgets.
    Transparency = 4, 
}

impl Default for MaterialType {
    fn default() -> Self {
        Self::Canvas
    }
}