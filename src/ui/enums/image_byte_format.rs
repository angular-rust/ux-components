#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageByteFormat {
    // Raw RGBA format.
    // Unencoded bytes, in RGBA row-primary form with premultiplied alpha, 8 bits per channel.
    RawRgba = 0,

    
    // Raw straight RGBA format.
    // Unencoded bytes, in RGBA row-primary form with straight alpha, 8 bits per channel.
    RawStraightRgba = 1,
    
    // Raw unmodified format.
    // Unencoded bytes, in the image's existing format. For example, a grayscale image may use a single 8-bit channel for each pixel.
    RawUnmodified = 2,

    // PNG format.
    // A loss-less compression format for images. This format is well suited for images with hard edges, 
    // such as screenshots or sprites, and images with text. Transparency is supported. 
    // The PNG format supports images up to 2,147,483,647 pixels in either dimension, though in practice 
    // available memory provides a more immediate limitation on maximum image size.
    // PNG images normally use the .png file extension and the image/png MIME type.
    
    // See also:
    
    // en.wikipedia.org/wiki/Portable_Network_Graphics, the Wikipedia page on PNG.
    // tools.ietf.org/rfc/rfc2083.txt, the PNG standard.
    Png = 3,
}

impl Default for ImageByteFormat {
    fn default() -> Self {
        Self::RawRgba
    }
}
