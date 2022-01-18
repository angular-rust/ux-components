#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageByteFormat {
    Png = 2,
    RawRgba = 0,
    RawUnmodified = 1,
}
