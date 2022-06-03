pub trait EdgeInsetsGeometry {
    // collapsedSize → Size
    // The size that this EdgeInsets would occupy with an empty interior.
    // read-only
    // flipped → EdgeInsetsGeometry
    // An EdgeInsetsGeometry with top and bottom, left and right, and start and end flipped.
    // read-only
    // hashCode → int
    // The hash code for this object.
    // read-only, override
    // horizontal → double
    // The total offset in the horizontal direction.
    // read-only
    // isNonNegative → bool
    // Whether every dimension is non-negative.
    // read-only
    // runtimeType → Type
    // A representation of the runtime type of the object.
    // read-only, inherited
    // vertical → double
    // The total offset in the vertical direction.
    // read-only


    // Returns the sum of two EdgeInsetsGeometry objects.
    // add(EdgeInsetsGeometry other) -> EdgeInsetsGeometry
    
    // The total offset in the given direction.
    // along(Axis axis) -> double
    
    // Returns a new EdgeInsetsGeometry object with all values greater than or equal to min, and less than or equal to max.
    // clamp(EdgeInsetsGeometry min, EdgeInsetsGeometry max) -> EdgeInsetsGeometry
    
    // Returns a new size that is smaller than the given size by the amount of inset in the horizontal and vertical directions.
    // deflateSize(Size size) -> Size
    
    // Returns a new size that is bigger than the given size by the amount of inset in the horizontal and vertical directions.
    // inflateSize(Size size) -> Size
    
    // Convert this instance into an EdgeInsets, which uses literal coordinates (i.e. the left coordinate being explicitly a
    // distance from the left, and the right coordinate being explicitly a distance from the right).
    // resolve(TextDirection? direction) -> EdgeInsets
    
    // Returns the difference between two EdgeInsetsGeometry objects.
    // subtract(EdgeInsetsGeometry other) -> EdgeInsetsGeometry
}

pub struct NoneEdgeInsetsGeometry;

impl EdgeInsetsGeometry for NoneEdgeInsetsGeometry {
    
}

