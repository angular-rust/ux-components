#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FramePhase {
    VsyncStart = 0,
    BuildStart = 1,
    BildFinish = 2,
    RasterStart = 3,
    RasterFinish = 4,
    RasterFinishWallTime = 5,
}

impl Default for FramePhase {
    fn default() -> Self {
        Self::VsyncStart
    }
}
