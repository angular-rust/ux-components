use std::future::Future;

use crate::{
    foundation::Key,
    painting::{
        DecoderCallback, ImageCache, ImageCacheStatus, ImageConfiguration, ImageErrorListener,
        ImageProvider, ImageStream, ImageStreamCompleter,
    },
    services::AssetBundle,
};

use super::AssetBundleImageProvider;

pub struct ExactAssetImage {
    // The name of the asset.
    pub asset_name: String,

    // The bundle from which the image will be obtained.
    pub bundle: Option<Box<dyn AssetBundle>>,

    // The key to use to obtain the resource from the bundle. This is the argument passed to AssetBundle.load.
    pub key_name: String,

    // The name of the package from which the image is included.
    // See the documentation for the ExactAssetImage class itself for details.
    pub package: Option<String>,

    // The scale to place in the ImageInfo object of the image.
    pub scale: f64,
}

impl ExactAssetImage {
    pub fn new(
        asset_name: String,
        scale: Option<f64>, /*  = 1.0*/
        bundle: Option<Box<dyn AssetBundle>>,
        package: Option<String>,
    ) -> Self {
        todo!()
    }
}

impl AssetBundleImageProvider for ExactAssetImage {
    fn load(key: super::AssetBundleImageKey, decode: Box<DecoderCallback>) -> ImageStreamCompleter {
        todo!()
    }

    fn obtain_key(
        configuration: ImageConfiguration,
    ) -> Box<dyn Future<Output = super::AssetBundleImageKey>> {
        todo!()
    }

    fn resolve_stream_for_key(
        configuration: ImageConfiguration,
        stream: ImageStream,
        key: super::AssetBundleImageKey,
        handle_error: Option<Box<ImageErrorListener>>,
    ) {
        todo!()
    }
}

impl ImageProvider for ExactAssetImage {
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
