#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustLogoStyle {
    // Show only Flutter's logo, not the "Flutter" label.
    // This is the default behavior for FlutterLogoDecoration objects.
    MarkOnly = 0,
    // Show Flutter's logo on the left, and the "Flutter" label to its right.
    Horizontal = 1,
    // Show Flutter's logo above the "Flutter" label.
    Stacked = 2,
}

impl Default for RustLogoStyle {
    fn default() -> Self {
        Self::MarkOnly
    }
}
