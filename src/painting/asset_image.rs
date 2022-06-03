use std::{default::default, future::Future};

use crate::{
    engine::d2,
    foundation::Key,
    painting::{
        DecoderCallback, ImageCache, ImageCacheStatus, ImageConfiguration, ImageErrorListener,
        ImageProvider, ImageStream, ImageStreamCompleter,
    },
    services::AssetBundle,
};

use super::{AssetBundleImageKey, AssetBundleImageProvider};

pub struct AssetImage {
    // The name of the main asset from the set of images to choose from. See the documentation for the AssetImage class itself for details.
    pub asset_name: String,

    // The bundle from which the image will be obtained.
    pub bundle: Option<Box<dyn AssetBundle>>,

    // The name used to generate the key to obtain the asset.
    // For local assets this is assetName, and for assets from packages the assetName is prefixed 'packages/
    pub key_name: String,

    // The name of the package from which the image is included. See the documentation for the AssetImage class itself for details.
    pub package: Option<String>,
}

impl AssetImage {
    // Converts a key into an ImageStreamCompleter, and begins fetching the image.
    // load(AssetBundleImageKey key, DecoderCallback decode) -> ImageStreamCompleter

    // Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.
    // obtainKey(ImageConfiguration configuration) -> Future<AssetBundleImageKey>

    // Called by resolve with the key returned by obtainKey.
    // resolveStreamForKey(ImageConfiguration configuration, ImageStream stream, AssetBundleImageKey key, ImageErrorListener handleError) -> void
}

impl AssetBundleImageProvider for AssetImage {
    fn load(key: AssetBundleImageKey, decode: Box<DecoderCallback>) -> ImageStreamCompleter {
        todo!()
    }

    fn obtain_key(
        configuration: ImageConfiguration,
    ) -> Box<dyn Future<Output = AssetBundleImageKey>> {
        todo!()
    }

    fn resolve_stream_for_key(
        configuration: ImageConfiguration,
        stream: ImageStream,
        key: AssetBundleImageKey,
        handle_error: Option<Box<ImageErrorListener>>,
    ) {
        todo!()
    }
}

impl ImageProvider for AssetImage {
    fn create_stream(&self, configuration: ImageConfiguration) -> ImageStream {
        let asset_name = format!("assets/{}", self.asset_name);

        ImageStream {
            image: Some(d2::Image::from_file(asset_name.as_str())),
            ..default()
        }
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

impl Default for AssetImage {
    fn default() -> Self {
        Self {
            asset_name: Default::default(),
            bundle: Default::default(),
            key_name: Default::default(),
            package: Default::default(),
        }
    }
}
