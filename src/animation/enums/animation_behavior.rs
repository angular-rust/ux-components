#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationBehavior {
    // The AnimationController will reduce its duration when AccessibilityFeatures.disableAnimations is true.
    Normal = 0,
    // The AnimationController will preserve its behavior.
    // This is the default for repeating animations in order to prevent them from flashing rapidly on the screen 
    // if the widget does not take the AccessibilityFeatures.disableAnimations flag into account.
    Preserve = 1,
}

impl Default for AnimationBehavior {
    fn default() -> Self {
        Self::Normal
    }
}
