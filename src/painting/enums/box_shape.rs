#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxShape {
    // An axis-aligned, 2D rectangle. May have rounded corners (described by a BorderRadius). 
    // The edges of the rectangle will match the edges of the box into which the Border or BoxDecoration is painte
    Rectangle = 0,
    // A circle centered in the middle of the box into which the Border or BoxDecoration is painted. 
    // The diameter of the circle is the shortest dimension of the box, either the width or the height, 
    // such that the circle touches the edges of the box.
    Circle = 1,
}

impl Default for BoxShape {
    fn default() -> Self {
        Self::Rectangle
    }
}
