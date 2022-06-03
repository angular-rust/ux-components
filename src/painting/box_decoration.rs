use crate::{prelude::Color, ui::BlendMode};

use super::{
    BorderRadiusGeometry, BoxBorder, BoxShadow, BoxShape, Decoration, DecorationImage,
    EdgeInsetsGeometry, Gradient,
};

pub struct BoxDecoration {
    // The blend mode applied to the color or gradient background of the box.
    pub background_blend_mode: Option<BlendMode>,

    // A border to draw above the background color, gradient, or image.
    pub border: Box<dyn BoxBorder>,

    // If non-null, the corners of this box are rounded by this BorderRadius.
    pub border_radius: Box<dyn BorderRadiusGeometry>,

    // A list of shadows cast by this box behind the box.
    pub box_shadow: Vec<BoxShadow>,

    // The color to fill in the background of the box.
    pub color: Color,

    // A gradient to use when filling the box.
    pub gradient: Box<dyn Gradient>,

    // An image to paint above the background color or gradient.
    pub image: Option<DecorationImage>,

    // Whether this decoration is complex enough to benefit from caching its painting. (from Decoration)
    pub is_complex: bool,

    // Returns the insets to apply when using this decoration on a box that has contents,
    // so that the contents do not overlap the edges of the decoration.
    // For example, if the decoration draws a frame around its edge, the padding would return the distance by which to inset
    // the children so as to not overlap the frame. (from Decoration)
    pub padding: Box<dyn EdgeInsetsGeometry>,

    // The shape to fill the background color, gradient, and image into and to cast as the boxShadow.
    pub shape: BoxShape,
}

impl BoxDecoration {
    // Creates a copy of this object but with the given fields replaced with the new values.
    // copyWith({Color? color, DecorationImage? image, BoxBorder? border, BorderRadiusGeometry? borderRadius, List<BoxShadow>? boxShadow, Gradient? gradient, BlendMode? backgroundBlendMode, BoxShape? shape}) -> BoxDecoration

    // Returns a new box decoration that is scaled by the given factor.
    pub fn scale(&self, factor: f32) -> BoxDecoration {
        todo!()
    }
}

impl Decoration for BoxDecoration {
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
