// width, height
pub struct Size(pub f32, pub f32);

impl Default for Size {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
