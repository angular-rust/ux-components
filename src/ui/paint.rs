use crate::prelude::Color;

use super::{
    BlendMode, ColorFilter, FilterQuality, ImageFilter, MaskFilter, PaintingStyle, Shader,
    StrokeCap, StrokeJoin,
};

pub struct Paint {
    // A blend mode to apply when a shape is drawn or a layer is composited.
    pub blend_mode: BlendMode,

    // The color to use when stroking or filling a shape.
    pub color: Color,

    // A color filter to apply when a shape is drawn or when a layer is composited.
    pub color_filter: Option<ColorFilter>,

    // Controls the performance vs quality trade-off to use when sampling bitmaps, as with an ImageShader,
    // or when drawing images, as with Canvas.drawImage, Canvas.drawImageRect, Canvas.drawImageNine or Canvas.drawAtlas.
    pub filter_quality: FilterQuality,

    // The ImageFilter to use when drawing raster images.
    pub image_filter: Option<ImageFilter>,

    // Whether the colors of the image are inverted when drawn.
    pub invert_colors: bool,

    // Whether to apply anti-aliasing to lines and images drawn on the canvas.
    pub is_antialias: bool,

    // A mask filter (for example, a blur) to apply to a shape after it has been drawn but before it has been composited into the image.
    pub mask_filter: Option<MaskFilter>,

    // The shader to use when stroking or filling a shape.
    pub shader: Option<Shader>,

    // The kind of finish to place on the end of lines drawn when style is set to PaintingStyle.stroke.
    pub stroke_cap: StrokeCap,

    // The kind of finish to place on the joins between segments.
    pub stroke_join: StrokeJoin,

    // The limit for miters to be drawn on segments when the join is set to StrokeJoin.miter and
    // the style is set to PaintingStyle.stroke. If this limit is exceeded,
    // then a StrokeJoin.bevel join will be drawn instead. This may cause some 'popping' of the
    // corners of a path if the angle between line segments is animated, as seen in the diagrams below.
    pub stroke_miter_limit: f32,

    // How wide to make edges drawn when style is set to PaintingStyle.stroke.
    // The width is given in logical pixels measured in the direction orthogonal to the direction of the path.
    pub stroke_width: f32,

    // Whether to paint inside shapes, the edges of shapes, or both.
    pub style: PaintingStyle,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            blend_mode: Default::default(),
            color: Default::default(),
            color_filter: Default::default(),
            filter_quality: Default::default(),
            image_filter: Default::default(),
            invert_colors: Default::default(),
            is_antialias: Default::default(),
            mask_filter: Default::default(),
            shader: Default::default(),
            stroke_cap: Default::default(),
            stroke_join: Default::default(),
            stroke_miter_limit: Default::default(),
            stroke_width: Default::default(),
            style: Default::default(),
        }
    }
}
