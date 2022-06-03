#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Clear = 0,
    Color = 27,
    ColorBurn = 19,
    ColorDodge = 18,
    Darken = 16,
    Difference = 22,
    Dst = 2,
    DstATop = 10,
    DstIn = 6,
    DstOut = 8,
    DstOver = 4,
    Exclusion = 23,
    HardLight = 20,
    Hue = 25,
    Lighten = 17,
    Luminosity = 28,
    Modulate = 13,
    Multiply = 24,
    Overlay = 15,
    Plus = 12,
    Saturation = 26,
    Screen = 14,
    SoftLight = 21,
    Src = 1,
    SrcATop = 9,
    SrcIn = 5,
    SrcOut = 7,
    SrcOver = 3,
    Xor = 11,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Clear
    }
}
