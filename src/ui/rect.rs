use super::Offset;

pub struct Rect {
    // The offset of the bottom edge of this rectangle from the y axis.
    // bottom: f32
    
    // The offset to the center of the bottom edge of this rectangle.
    // bottomCenter: Offset
    
    // The offset to the intersection of the bottom and left edges of this rectangle.
    // bottomLeft: Offset
    
    // The offset to the intersection of the bottom and right edges of this rectangle.
    // bottomRight: Offset
    
    // The offset to the point halfway between the left and right and the top and bottom edges of this rectangle.
    // center: Offset
    
    // The offset to the center of the left edge of this rectangle.
    // centerLeft: Offset
    
    // The offset to the center of the right edge of this rectangle.
    // centerRight: Offset
    
    // Whether any of the dimensions are NaN.
    // hasNaN: bool
    
    // The distance between the top and bottom edges of this rectangle.
    // height: f32
    
    // Whether this rectangle encloses a non-zero area. Negative areas are considered empty.
    // isEmpty: bool
    
    // Whether all coordinates of this rectangle are finite.
    // isFinite: bool
    
    // Whether any of the coordinates of this rectangle are equal to positive infinity.
    // isInfinite: bool
    
    // The offset of the left edge of this rectangle from the x axis.
    // left: f32
    
    // The greater of the magnitudes of the width and the height of this rectangle.
    // longestSide: f32
    
    // The offset of the right edge of this rectangle from the x axis.
    // right: f32
    
    // The lesser of the magnitudes of the width and the height of this rectangle.
    // shortestSide: f32
    
    // The distance between the upper-left corner and the lower-right corner of this rectangle.
    // size: Size
    
    // The offset of the top edge of this rectangle from the y axis.
    // top: f32
    
    // The offset to the center of the top edge of this rectangle.
    // topCenter: Offset
    
    // The offset to the intersection of the top and left edges of this rectangle.
    // topLeft: Offset
    
    // The offset to the intersection of the top and right edges of this rectangle.
    // topRight: Offset
    
    // The distance between the left and right edges of this rectangle.
    // width: f32
}

impl Rect {
    // constructors {
    // Constructs a rectangle from its center point, width, and height. 
    pub fn from_center(center: Offset, width: f32, height: f32) -> Self {
        todo!()
    }
    
    // Construct a rectangle that bounds the given circle. 
    pub fn from_circle(center: Offset, radius: f32) -> Self {
        todo!()
    }
    
    // Construct a rectangle from its left, top, right, and bottom edges.
    pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        todo!()
    }
    
    // Construct a rectangle from its left and top edges, its width, and its height.
    pub fn from_ltwh(left: f32, top: f32, width: f32, height: f32) -> Self {
        todo!()
    }
    
    // Construct the smallest rectangle that encloses the given offsets, treating them as vectors from the origin. 
    pub fn from_points(a: Offset, b: Offset) -> Self {
        todo!()
    }
    
    // }

    // Whether the point specified by the given offset (which is assumed to be relative to the origin) 
    // lies between the left and right and the top and bottom edges of this rectangle. 
    // contains(Offset offset) -> bool
    
    // Returns a new rectangle with edges moved inwards by the given delta. 
    // deflate(double delta) -> Rect
    
    // Returns a new rectangle which is the bounding box containing this rectangle and the given rectangle. 
    // expandToInclude(Rect other) -> Rect
    
    // Returns a new rectangle with edges moved outwards by the given delta. 
    // inflate(double delta) -> Rect
    
    // Returns a new rectangle that is the intersection of the given rectangle and this rectangle. 
    // The two rectangles must overlap for this to be meaningful. 
    // If the two rectangles do not overlap, then the resulting Rect will have a negative width or height. 
    // intersect(Rect other) -> Rect
    
    // Whether other has a nonzero area of overlap with this rectangle. 
    // overlaps(Rect other) -> bool
    
    // Returns a new rectangle translated by the given offset. 
    // shift(Offset offset) -> Rect
    
    // Returns a new rectangle with translateX added to the x components and translateY added to the y components. 
    // translate(double translateX, double translateY) -> Rect    
}