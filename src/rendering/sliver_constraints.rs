use crate::painting::{Axis, AxisDirection};

use super::{GrowthDirection, ScrollDirection};

/*
asBoxConstraints({f32 minExtent = 0.0, f32 maxExtent = f32.infinity, f32? crossAxisExtent}) -> BoxConstraints
Returns BoxConstraints that reflects the sliver constraints.
copyWith({AxisDirection? axisDirection, GrowthDirection? growthDirection, ScrollDirection? userScrollDirection, f32? scrollOffset, f32? precedingScrollExtent, f32? overlap, f32? remainingPaintExtent, f32? crossAxisExtent, AxisDirection? crossAxisDirection, f32? viewportMainAxisExtent, f32? remainingCacheExtent, f32? cacheOrigin}) -> SliverConstraints
Creates a copy of this object but with the given fields replaced with the new values.
*/
pub struct SliverConstraints {
    // The axis along which the scrollOffset and remainingPaintExtent are measured.
    pub axis: Axis,
    
    // The direction in which the scrollOffset and remainingPaintExtent increase.
    pub axis_direction: AxisDirection,
    
    // Where the cache area starts relative to the scrollOffset.
    pub cache_origin: f32,
    
    // The direction in which children should be placed in the cross axis.
    pub cross_axis_direction: AxisDirection,
    
    // The number of pixels in the cross-axis.
    pub cross_axis_extent: f32,
    
    // The direction in which the contents of slivers are ordered, relative to the axisDirection.
    pub growth_direction: GrowthDirection,
    
    // Whether the constraint is expressed in a consistent manner.
    pub is_normalized: bool,
    
    // Whether there is exactly one size possible given these constraints.
    pub is_tight: bool,
    
    // Return what the growthDirection would be if the axisDirection was either AxisDirection.down or AxisDirection.right.
    pub normalized_growth_direction: GrowthDirection,
    
    // The number of pixels from where the pixels corresponding to the scrollOffset will be painted up to the first pixel 
    // that has not yet been painted on by an earlier sliver, in the axisDirection.
    pub overlap: f32,
    
    // The scroll distance that has been consumed by all RenderSlivers that came before this RenderSliver.
    pub preceding_scroll_extent: f32,
    
    // Describes how much content the sliver should provide starting from the cacheOrigin.
    pub remaining_cache_extent: f32,
    
    // The number of pixels of content that the sliver should consider providing. (Providing more pixels than this is inefficient.)
    pub remaining_paint_extent: f32,
    
    // The scroll offset, in this sliver's coordinate system, that corresponds to the earliest visible part of 
    // this sliver in the AxisDirection if growthDirection is GrowthDirection.forward or in 
    // the opposite AxisDirection direction if growthDirection is GrowthDirection.reverse.
    pub scroll_offset: f32,
    
    // The direction in which the user is attempting to scroll, relative to the axisDirection and growthDirection.
    pub user_scroll_direction: ScrollDirection,
    
    // The number of pixels the viewport can display in the main axis.
    pub viewport_main_axis_extent: f32,
}