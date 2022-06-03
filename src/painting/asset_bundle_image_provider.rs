use std::future::Future;

use super::{
    DecoderCallback, ImageConfiguration, ImageErrorListener, ImageProvider, ImageStream,
    ImageStreamCompleter, AssetBundleImageKey,
};

// : ImageProvider<AssetBundleImageKey>
pub trait AssetBundleImageProvider: ImageProvider {
    // Converts a key into an ImageStreamCompleter, and begins fetching the image.
    fn load(key: AssetBundleImageKey, decode: Box<DecoderCallback>) -> ImageStreamCompleter;

    // Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.
    fn obtain_key(
        configuration: ImageConfiguration,
    ) -> Box<dyn Future<Output = AssetBundleImageKey>>;

    // Called by resolve with the key returned by obtainKey.
    fn resolve_stream_for_key(
        configuration: ImageConfiguration,
        stream: ImageStream,
        key: AssetBundleImageKey,
        handle_error: Option<Box<ImageErrorListener>>,
    );
}

// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }
