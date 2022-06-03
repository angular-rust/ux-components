#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    Rgba8888 = 0,
    Bgra8888 = 1,
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::Rgba8888
    }
}