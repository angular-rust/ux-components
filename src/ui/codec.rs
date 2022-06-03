use bytes::Bytes;
use std::future::Future;

use crate::ui::ImageByteFormat;

pub struct DartImage {
    // The number of image pixels along the image's vertical axis.
    pub height: i32,

    // The number of image pixels along the image's horizontal axis.
    pub width: i32,
}

impl DartImage {
    // Creates a disposable handle to this image.
    pub fn clone() -> DartImage {
        todo!()
    }

    // Release this handle's claim on the underlying Image. This handle is no longer usable after this method is called.
    pub fn dispose() {}

    // Returns true if other is a clone of this and thus shares the same underlying image memory, even if this or other is disposed.
    pub fn is_clone_of(other: DartImage) -> bool {
        todo!()
    }

    // Converts the Image object into a byte array.
    // pub fn to_byte_data(
    //     format: Option<ImageByteFormat>, /*= ImageByteFormat.rawRgba*/
    // ) -> impl Future<Output = Option<Bytes>> {
    pub fn to_byte_data(
        format: Option<ImageByteFormat>, /*= ImageByteFormat.rawRgba*/
    ) -> Box<dyn Future<Output = Option<Bytes>>> {
        todo!()
    }
}

pub struct FrameInfo {
    // The duration this frame should be shown.
// duration: Duration,

// The Image object for this frame.
// image: Image, // dart Image
}

pub struct Codec {
    // Number of frames in this image.
    pub frame_count: i32,

    // Number of times to repeat the animation.
    pub repetition_count: i32,
}

impl Codec {
    // Release the resources used by this object. The object is no longer usable after this method is called.
    pub fn dispose() {}

    // Fetches the next animation frame.
    // pub fn getNextFrame() -> impl Future<Output = FrameInfo> {
    pub fn get_next_frame() -> Box<dyn Future<Output = FrameInfo>> {
        todo!()
    }
}
// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }
