use crate::{
    material::AlignmentGeometry,
    ui::{Offset, Rect, Size, TextDirection},
};

use super::Alignment;

pub struct FractionalOffset {
    // The distance fraction in the horizontal direction.
    pub dx: f32,

    // The distance fraction in the vertical direction.
    pub dy: f32,
    // The distance fraction in the horizontal direction.
    // pub x: f32, // Alignment

    // The distance fraction in the vertical direction.
    // pub y: f32, // Alignment
}

impl FractionalOffset {
    // The center point along the bottom edge.
    pub const BOTTOM_CENTER: FractionalOffset = FractionalOffset { dx: 0.5, dy: 1.0 };

    // The bottom left corner.
    pub const BOTTOM_LEFT: FractionalOffset = FractionalOffset { dx: 0.0, dy: 1.0 };
    
    // The bottom right corner.
    pub const BOTTOM_RIGHT: FractionalOffset = FractionalOffset { dx: 1.0, dy: 1.0 };

    // The center point, both horizontally and vertically.
    pub const CENTER: FractionalOffset = FractionalOffset { dx: 0.5, dy: 0.5 };

    // The center point along the left edge.
    pub const CENTER_LEFT: FractionalOffset = FractionalOffset { dx: 0.0, dy: 0.5 };

    // The center point along the right edge.
    pub const CENTER_RIGHT: FractionalOffset = FractionalOffset { dx: 1.0, dy: 0.5 };

    // The center point along the top edge.
    pub const TOP_CENTER: FractionalOffset = FractionalOffset { dx: 0.5, dy: 0.0 };

    // The top left corner.
    pub const TOP_LEFT: FractionalOffset = FractionalOffset { dx: 0.0, dy: 0.0 };

    // The top right corner.
    pub const TOP_RIGHT: FractionalOffset = FractionalOffset { dx: 1.0, dy: 0.0 };

    // Returns the sum of two AlignmentGeometry objects.
    pub fn add(other: Box<dyn AlignmentGeometry>) -> Box<dyn AlignmentGeometry> {
        todo!()
    }

    // Returns the offset that is this fraction in the direction of the given offset.
    pub fn along_offset(other: Offset) -> Offset {
        todo!()
    }

    // Returns the offset that is this fraction within the given size.
    pub fn along_size(other: Size) -> Offset {
        todo!()
    }

    // Returns a rect of the given size, aligned within given rect as specified by this alignment.
    pub fn inscribe(size: Size, rect: Rect) -> Rect {
        todo!()
    }

    // Convert this instance into an Alignment, which uses literal coordinates (the x coordinate being explicitly a distance from the left).
    pub fn resolve(direction: Option<TextDirection>) -> Alignment {
        todo!()
    }

    // Returns the point that is this fraction within the given rect.
    pub fn within_rect(rect: Rect) -> Offset {
        todo!()
    }
}
