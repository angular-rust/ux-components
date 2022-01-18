#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexMode {
    Triangles = 0,
    TriangleStrip = 1,
    TriangleFan = 2,
}
