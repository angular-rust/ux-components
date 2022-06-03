#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageRepeat {
    // Repeat the image in both the x and y directions until the box is filled.
    Repeat = 0,
    // Repeat the image in the x direction until the box is filled horizontally.
    RepeatX = 1,
    // Repeat the image in the y direction until the box is filled vertically.
    RepeatY = 2,
    // Leave uncovered portions of the box transparent.
    NoRepeat = 3
}

impl Default for ImageRepeat {
    fn default() -> Self {
        Self::Repeat
    }
}