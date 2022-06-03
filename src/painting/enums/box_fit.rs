#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxFit {
    // Fill the target box by distorting the source's aspect ratio.
    Fill = 0,
    // As large as possible while still containing the source entirely within the target box.
    Contain = 1,
    // As small as possible while still covering the entire target box.
    // To actually clip the content, use clipBehavior: Clip.hardEdge alongside this in a FittedBox.
    Cover = 2,
    // Make sure the full width of the source is shown, regardless of whether this means the source overflows the target box vertically.
    // To actually clip the content, use clipBehavior: Clip.hardEdge alongside this in a FittedBox.
    FitWidth = 3,
    // Make sure the full height of the source is shown, regardless of whether this means the source overflows the target box horizontally.
    // To actually clip the content, use clipBehavior: Clip.hardEdge alongside this in a FittedBox.
    FitHeight = 4,
    // Align the source within the target box (by default, centering) and discard any portions of the source that lie outside the box.
    // The source image is not resized.
    // To actually clip the content, use clipBehavior: Clip.hardEdge alongside this in a FittedBox.
    None = 5,
    // Align the source within the target box (by default, centering) and, if necessary, scale the source down to ensure that the source fits within the box.
    // This is the same as contain if that would shrink the image, otherwise it is the same as none.
    ScaleDown = 6,
}

impl Default for BoxFit {
    fn default() -> Self {
        Self::Fill
    }
}
