use super::{BorderSide, BoxBorder, EdgeInsetsGeometry, ShapeBorder, NoneEdgeInsetsGeometry};

pub struct Border {
    // The bottom side of this border.
    pub bottom: BorderSide,

    // The widths of the sides of this border represented as an EdgeInsets.
    pub dimensions: Box<dyn EdgeInsetsGeometry>,

    // Whether all four sides of the border are identical. Uniform borders are typically more efficient to paint.
    pub is_uniform: bool,

    // The left side of this border.
    pub left: BorderSide,

    // The right side of this border.
    pub right: BorderSide,

    // The top side of this border.
    pub top: BorderSide,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            bottom: Default::default(),
            dimensions: box NoneEdgeInsetsGeometry,
            is_uniform: Default::default(),
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
        }
    }
}

impl BoxBorder for Border {}

impl ShapeBorder for Border {}
