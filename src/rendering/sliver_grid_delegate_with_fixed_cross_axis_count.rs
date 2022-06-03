use super::SliverGridDelegate;

/*
getLayout(SliverConstraints constraints) -> SliverGridLayout
Returns information about the size and position of the tiles in the grid.
override
noSuchMethod(Invocation invocation) -> dynamic
Invoked when a non-existent method or property is accessed.
inherited
shouldRelayout(covariant SliverGridDelegateWithFixedCrossAxisCount oldDelegate) -> bool
Override this method to return true when the children need to be laid out.
override
*/
pub struct SliverGridDelegateWithFixedCrossAxisCount {
    // The ratio of the cross-axis to the main-axis extent of each child.
    pub child_aspect_ratio: f32,

    // The number of children in the cross axis.
    pub cross_axis_count: usize,

    // The number of logical pixels between each child along the cross axis.
    pub cross_axis_spacing: f32,

    // The extent of each tile in the main axis. If provided it would define the logical pixels taken by each tile in the main-axis.
    pub main_axis_extent: f32,

    // The number of logical pixels between each child along the main axis.
    pub main_axis_spacing: f32,
}

impl Default for SliverGridDelegateWithFixedCrossAxisCount {
    fn default() -> Self {
        Self {
            child_aspect_ratio: Default::default(),
            cross_axis_count: Default::default(),
            cross_axis_spacing: Default::default(),
            main_axis_extent: Default::default(),
            main_axis_spacing: Default::default(),
        }
    }
}

impl SliverGridDelegate for SliverGridDelegateWithFixedCrossAxisCount {
    
}