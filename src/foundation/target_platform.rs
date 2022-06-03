pub enum TargetPlatform {
    Android = 0,
    Fuchsia = 1,
    Ios = 2,
    Linux = 3,
    MacOs = 4,
    Windows = 5,
}

impl Default for TargetPlatform {
    fn default() -> Self {
        Self::Android
    }
}
