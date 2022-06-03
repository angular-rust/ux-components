#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapAlignment {
    // Place the objects as close to the start of the axis as possible.
    // If this value is used in a horizontal direction, a TextDirection must be available to determine if the start is the left or the right.   
    // If this value is used in a vertical direction, a VerticalDirection must be available to determine if the start is the top or the bottom.
    Start  = 0,

    // Place the objects as close to the end of the axis as possible.
    // If this value is used in a horizontal direction, a TextDirection must be available to determine if the end is the left or the right.    
    // If this value is used in a vertical direction, a VerticalDirection must be available to determine if the end is the top or the bottom.    
    End = 1,

    // Place the objects as close to the middle of the axis as possible.
    Center = 2,

    // Place the free space evenly between the objects.
    SpaceBetween = 3,

    // Place the free space evenly between the objects as well as half of that space before and after the first and last objects.
    SpaceAround = 4,

    // Place the free space evenly between the objects as well as before and after the first and last objects.
    SpaceEvenly = 5,
}

impl Default for WrapAlignment {
    fn default() -> Self {
        Self::Start
    }
}
