#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppLifecycleState {
    Resumed = 0,
    Inactive = 1,
    Paused = 2,
    Detached = 3,
}

impl Default for AppLifecycleState {
    fn default() -> Self {
        Self::Resumed
    }
}