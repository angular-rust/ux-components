#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationStatus {
    // The animation is stopped at the beginning.
    Dismissed = 0,
    // The animation is running from beginning to end.
    Forward = 1,
    // The animation is running backwards, from end to beginning.
    Reverse = 2,
    // The animation is stopped at the end.
    Completed = 3,
}

impl Default for AnimationStatus {
    fn default() -> Self {
        Self::Dismissed
    }
}
