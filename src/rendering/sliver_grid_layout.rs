pub trait SliverGridLayout {
    // computeMaxScrollOffset(int childCount): f32
    // The scroll extent needed to fully display all the tiles if there are childCount children in total.
    // getGeometryForChildIndex(int index): SliverGridGeometry
    // The size and position of the child with the given index.
    // getMaxChildIndexForScrollOffset(f32 scrollOffset): int
    // The maximum child index that intersects with (or is before) this scroll offset.
    // getMinChildIndexForScrollOffset(f32 scrollOffset): int
    // The minimum child index that intersects with (or is after) this scroll offset.
}