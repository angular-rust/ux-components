use crate::ui::{Offset, Size, WindowPadding};

use super::EdgeInsetsGeometry;

// add(EdgeInsetsGeometry other) → EdgeInsetsGeometry
// Returns the sum of two EdgeInsetsGeometry objects.
// override
// along(Axis axis) → double
// The total offset in the given direction.
// inherited
// clamp(EdgeInsetsGeometry min, EdgeInsetsGeometry max) → EdgeInsetsGeometry
// Returns a new EdgeInsetsGeometry object with all values greater than or equal to min, and less than or equal to max.
// override
// copyWith({double? left, double? top, double? right, double? bottom}) → EdgeInsets
// Creates a copy of this EdgeInsets but with the given fields replaced with the new values.
// deflateRect(Rect rect) → Rect
// Returns a new rect that is smaller than the given rect in each direction by the amount of inset in each direction. Specifically, the left edge of the rect is moved right by left, the top edge of the rect is moved down by top, the right edge of the rect is moved left by right, and the bottom edge of the rect is moved up by bottom.
// deflateSize(Size size) → Size
// Returns a new size that is smaller than the given size by the amount of inset in the horizontal and vertical directions.
// inherited
// inflateRect(Rect rect) → Rect
// Returns a new rect that is bigger than the given rect in each direction by the amount of inset in each direction. Specifically, the left edge of the rect is moved left by left, the top edge of the rect is moved up by top, the right edge of the rect is moved right by right, and the bottom edge of the rect is moved down by bottom.
// inflateSize(Size size) → Size
// Returns a new size that is bigger than the given size by the amount of inset in the horizontal and vertical directions.
// inherited
// noSuchMethod(Invocation invocation) → dynamic
// Invoked when a non-existent method or property is accessed.
// inherited
// resolve(TextDirection? direction) → EdgeInsets
// Convert this instance into an EdgeInsets, which uses literal coordinates (i.e. the left coordinate being explicitly a distance from the left, and the right coordinate being explicitly a distance from the right).
// override
// subtract(EdgeInsetsGeometry other) → EdgeInsetsGeometry
// Returns the difference between two EdgeInsetsGeometry objects.
// override
// toString() → String
// A string representation of this object.
// inherited

// // Creates insets where all the offsets are value.
// pub struct EdgeInsetsAll(pub f32);

// // Creates insets from offsets from the left, top, right, and bottom.
// pub struct EdgeInsetsLTRB {
//     pub left: f32, 
//     pub top: f32, 
//     pub right: f32, 
//     pub bottom: f32,
// }


// Creates insets that match the given window padding.
// EdgeInsets.fromWindowPadding(WindowPadding padding, double devicePixelRatio)

// Creates insets with symmetrical vertical and horizontal offsets.
pub struct EdgeInsetsSymmetric {
    pub vertical: f32,
    pub horizontal: f32,
}

pub struct EdgeInsets {
    // The offset from the bottom.
    pub bottom: f32,
    
    // An Offset describing the vector from the bottom left of a rectangle to the bottom left of that rectangle inset by this object.
    pub bottom_left: Offset,
    
    // An Offset describing the vector from the bottom right of a rectangle to the bottom right of that rectangle inset by this object.
    pub bottom_right: Offset,
    
    // The size that this EdgeInsets would occupy with an empty interior. (from EdgeInsetsGeometry)
    pub collapsed_size: Size,
    
    // An EdgeInsets with top and bottom as well as left and right flipped. (from EdgeInsetsGeometry)
    // flipped: Option<EdgeInsets>,
    
    // The total offset in the horizontal direction. (from EdgeInsetsGeometry)
    pub horizontal: f32,
    
    // Whether every dimension is non-negative. (from EdgeInsetsGeometry)
    pub is_non_negative: bool,
    
    // The offset from the left.
    pub left: f32,
    
    // The offset from the right.
    pub right: f32,
    
    // The offset from the top.
    pub top: f32,
    
    // An Offset describing the vector from the top left of a rectangle to the top left of that rectangle inset by this object.
    pub top_left: Offset,
    
    // An Offset describing the vector from the top right of a rectangle to the top right of that rectangle inset by this object.
    pub top_right: Offset,
    
    // The total offset in the vertical direction. (from EdgeInsetsGeometry)
    pub vertical: f32,
}

impl EdgeInsets {
    // Creates insets where all the offsets are value.
    pub fn all(value: f32) -> Self {
        todo!()
    }
    
    // Creates insets from offsets from the left, top, right, and bottom.
    pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        todo!()
    }
    
    
    // Creates insets that match the given window padding.
    pub fn from_window_padding(padding: WindowPadding, device_pixel_ratio: f32) -> Self {
        todo!()
    }
   
    // Creates insets with symmetrical vertical and horizontal offsets.
    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        todo!()
    }    
}

impl EdgeInsetsGeometry for EdgeInsets {

}