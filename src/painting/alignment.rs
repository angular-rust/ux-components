use crate::material::AlignmentGeometry;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Alignment {
    pub x: f32,
    pub y: f32,
}

impl Alignment {
    // The center point along the bottom edge.
    pub const BOTTOM_CENTER: Alignment = Alignment { x: 0.0, y: 1.0 };

    // The bottom left corner.
    pub const BOTTOM_LEFT: Alignment = Alignment { x: -1.0, y: 1.0 };

    // The bottom right corner.
    pub const BOTTOM_RIGHT: Alignment = Alignment { x: 1.0, y: 1.0 };

    // The center point, both horizontally and vertically.
    pub const CENTER: Alignment = Alignment { x: 0.0, y: 0.0 };

    // The center point along the left edge.
    pub const CENTER_LEFT: Alignment = Alignment { x: -1.0, y: 0.0 };

    // The center point along the right edge.
    pub const CENTER_RIGHT: Alignment = Alignment { x: 1.0, y: 0.0 };

    // The center point along the top edge.
    pub const TOP_CENTER: Alignment = Alignment { x: 0.0, y: -1.0 };

    // The top left corner.
    pub const TOP_LEFT: Alignment = Alignment { x: -1.0, y: -1.0 };

    // The top right corner.
    pub const TOP_RIGHT: Alignment = Alignment { x: 1.0, y: -1.0 };

    // add(AlignmentGeometry other) → AlignmentGeometry
    // Returns the sum of two AlignmentGeometry objects.
    // override
    // alongOffset(Offset other) → Offset
    // Returns the offset that is this fraction in the direction of the given offset.
    // alongSize(Size other) → Offset
    // Returns the offset that is this fraction within the given size.
    // inscribe(Size size, Rect rect) → Rect
    // Returns a rect of the given size, aligned within given rect as specified by this alignment.
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // resolve(TextDirection? direction) → Alignment
    // Convert this instance into an Alignment, which uses literal coordinates (the x coordinate being explicitly a distance from the left).
    // override
    // toString() → String
    // A string representation of this object.
    // override
    // withinRect(Rect rect) → Offset
    // Returns the point that is this fraction within the given rect.
}

impl AlignmentGeometry for Alignment {}
