use super::ShapeBorder;

pub trait BoxBorder: ShapeBorder {
    // The bottom side of this border.
    // bottom: BorderSide
    
    // Whether all four sides of the border are identical. Uniform borders are typically more efficient to paint.
    // isUniform: bool
    
    // The top side of this border.
    // top: BorderSide
}