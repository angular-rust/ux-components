pub struct  WindowPadding {
    // The distance from the bottom edge to the first unpadded pixel, in physical pixels.
    pub bottom: f32,


    // The distance from the left edge to the first unpadded pixel, in physical pixels.
    pub left: f32,

    // The distance from the right edge to the first unpadded pixel, in physical pixels.
    pub right: f32,

    // The distance from the top edge to the first unpadded pixel, in physical pixels.
    pub top: f32,
}

impl WindowPadding {
    pub const ZERO: WindowPadding = WindowPadding {
        left: 0.0,
        top: 0.0, 
        right: 0.0, 
        bottom: 0.0
    };
}