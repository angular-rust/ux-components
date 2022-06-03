use crate::{ui::{FilterQuality, ColorFilter, Rect}, material::AlignmentGeometry};

use super::{BoxFit, ImageProvider, ImageRepeat};

pub struct DecorationImage {
    // How to align the image within its bounds.
    pub alignment: Box<dyn AlignmentGeometry>,
    
    // The center slice for a nine-patch image.
    pub center_slice: Option<Rect>,
    
    // A color filter to apply to the image before painting it.
    pub color_filter: Option<ColorFilter>,
    
    // Used to set the filterQuality of the image.
    pub filter_quality: FilterQuality,
    
    // How the image should be inscribed into the box.
    pub fit: BoxFit,
    
    // The image to be painted into the decoration.
    pub image: Box<dyn ImageProvider>,
    
    // Whether the colors of the image are inverted when drawn.
    pub invert_colors: bool,
    
    // Whether to paint the image with anti-aliasing.
    pub is_anti_alias: bool,
    
    // Whether to paint the image in the direction of the TextDirection.
    pub match_text_direction: bool,
    
    // An optional error callback for errors emitted when loading image.
    // on_error: ImageErrorListener?,
    
    // If non-null, the value is multiplied with the opacity of each image pixel before painting onto the canvas.
    pub opacity: f32,
    
    // How to paint any portions of the box that would not otherwise be covered by the image.
    pub repeat: ImageRepeat,
    
    // Defines image pixels to be shown per logical pixels.
    pub scale: f32,
}

impl DecorationImage {
    // Creates a DecorationImagePainter for this DecorationImage.
    // pub fn create_painter(onChanged: VoidCallback) -> DecorationImagePainter {
    // }    
}