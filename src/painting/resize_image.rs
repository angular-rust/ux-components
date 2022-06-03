use std::future::Future;

use crate::{
    foundation::Key,
    painting::{
        DecoderCallback, ImageCache, ImageCacheStatus, ImageConfiguration, ImageErrorListener,
        ImageProvider, ImageStream, ImageStreamCompleter,
    },
};

pub struct ResizeImage {
    // Whether the width and height parameters should be clamped to the intrinsic width and height of the image.
    pub allow_upscaling: bool,

    // The height the image should decode to and cache.
    pub height: Option<i32>,

    // // The ImageProvider that this class wraps.
    // imageProvider: ImageProvider<Object>,

    // The width the image should decode to and cache.
    pub width: Option<i32>,
}

impl ResizeImage {
    // Converts a key into an ImageStreamCompleter, and begins fetching the image.
    // load(ResizeImageKey key, DecoderCallback decode) -> ImageStreamCompleter

    // Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.
    // obtainKey(ImageConfiguration configuration) -> Future<ResizeImageKey>

    // Called by resolve with the key returned by obtainKey.
    // resolveStreamForKey(ImageConfiguration configuration, ImageStream stream, ResizeImageKey key, ImageErrorListener handleError) -> void
}

impl ImageProvider for ResizeImage {
    fn create_stream(&self, configuration: ImageConfiguration) -> ImageStream {
        todo!()
    }

    fn evict(
        &self,
        cache: Option<ImageCache>,
        configuration: ImageConfiguration, /*= ImageConfiguration.empty*/
    ) -> Box<dyn Future<Output = bool>> {
        todo!()
    }

    fn load(&self, key: Key, decode: Box<DecoderCallback>) -> ImageStreamCompleter {
        todo!()
    }

    fn obtain_cache_status(
        &self,
        configuration: ImageConfiguration,
        handle_error: Option<Box<ImageErrorListener>>,
    ) -> Box<dyn Future<Output = Option<ImageCacheStatus>>> {
        todo!()
    }

    fn obtain_key(&self, configuration: ImageConfiguration) -> Box<dyn Future<Output = Key>> {
        todo!()
    }

    fn resolve(&self, configuration: ImageConfiguration) -> ImageStream {
        todo!()
    }

    fn resolve_stream_for_key(
        &self,
        configuration: ImageConfiguration,
        stream: ImageStream,
        key: Key,
        handle_error: Option<Box<ImageErrorListener>>,
    ) {
        todo!()
    }
}

// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }
