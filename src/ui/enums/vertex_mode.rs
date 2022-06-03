#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexMode {
    Triangles = 0,
    TriangleStrip = 1,
    TriangleFan = 2,
}

impl Default for VertexMode {
    fn default() -> Self {
        Self::Triangles
    }
}
