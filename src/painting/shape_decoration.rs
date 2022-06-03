use crate::prelude::Color;

use super::{BoxShadow, Decoration, DecorationImage, EdgeInsetsGeometry, Gradient, ShapeBorder};

pub struct ShapeDecoration {
    // The color to fill in the background of the shape.
    pub color: Color,

    // A gradient to use when filling the shape.
    pub gradient: Box<dyn Gradient>,

    // An image to paint inside the shape (clipped to its outline).
    pub image: Option<DecorationImage>,

    // Whether this decoration is complex enough to benefit from caching its painting.
    pub is_complex: bool,

    // The inset space occupied by the shape's border.
    pub padding: Box<dyn EdgeInsetsGeometry>,

    // A list of shadows cast by the shape.
    pub shadows: Vec<BoxShadow>,

    // The shape to fill the color, gradient, and image into and to cast as the shadows.
    pub shape: Box<dyn ShapeBorder>,
}

impl ShapeDecoration {
    // Creates a copy of this object but with the given fields replaced with the new values.
    // copyWith({Color? color, DecorationImage? image, BoxBorder? border, BorderRadiusGeometry? borderRadius, List<BoxShadow>? boxShadow, Gradient? gradient, BlendMode? backgroundBlendMode, BoxShape? shape}) -> ShapeDecoration

    // Returns a new box decoration that is scaled by the given factor.
    pub fn scale(&self, factor: f32) -> ShapeDecoration {
        todo!()
    }
}

impl Decoration for ShapeDecoration {
    // createBoxPainter([VoidCallback onChanged]) -> BoxPainter
    // Returns a BoxPainter that will paint this decoration.
    // @factory

    // getClipPath(Rect rect, TextDirection textDirection) -> Path
    // Returns a closed Path that describes the outer edge of this decoration.
    //
    // hitTest(Size size, Offset position, {TextDirection? textDirection}) -> bool
    // Tests whether the given point, on a rectangle of a given size, would be considered to hit the decoration or not. For example, if the decoration only draws a circle, this function might return true if the point was inside the circle and false otherwise.
    //
    // lerpFrom(Decoration? a, f32 t) -> Decoration?
    // Linearly interpolates from another Decoration (which may be of a different class) to this.
    // @protected
    //
    // lerpTo(Decoration? b, f32 t) -> Decoration?
    // Linearly interpolates from this to another Decoration (which may be of a different class).
    // @protected
    //
    // toStringShort() -> String
    // A brief description of this object, usually just the runtimeType and the hashCode.
    // override
}
