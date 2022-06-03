#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TooltipTriggerMode {
    Manual = 0,
    LongPress = 1,
    Tap = 2,
}

impl Default for TooltipTriggerMode {
    fn default() -> Self {
        Self::LongPress
    }
}
