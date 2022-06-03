use std::{collections::HashMap, future::Future};

use crate::{
    foundation::Key,
    painting::{
        DecoderCallback, ImageCache, ImageCacheStatus, ImageConfiguration, ImageErrorListener,
        ImageProvider, ImageStream, ImageStreamCompleter,
    },
};

pub struct NetworkImage {
    // The HTTP headers that will be used with HttpClient.get to fetch image from network.
    pub headers: Option<HashMap<String, String>>,

    // The scale to place in the ImageInfo object of the image.
    pub scale: f64,

    // The URL from which the image will be fetched.
    pub url: String,
}

impl NetworkImage {
    // Converts a key into an ImageStreamCompleter, and begins fetching the image.
    // load(NetworkImage key, DecoderCallback decode) -> ImageStreamCompleter

    // obtainKey(ImageConfiguration configuration) -> Future<NetworkImage>
    // Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.

    // Called by resolve with the key returned by obtainKey.
    // resolveStreamForKey(ImageConfiguration configuration, ImageStream stream, NetworkImage key, ImageErrorListener handleError) -> void
}

impl ImageProvider for NetworkImage {
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
