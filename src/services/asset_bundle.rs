use bytes::Bytes;
use std::future::Future;

pub struct NetworkAssetBundle {}

impl NetworkAssetBundle {
    // Creates a network asset bundle that resolves asset keys as URLs relative to the given base URL.
    // pub fn new(base_url: Uri) -> Self {
    pub fn new(base_url: String) -> Self {
        todo!()
    }
}

impl AssetBundle for NetworkAssetBundle {
    fn clear(&self) {
        todo!()
    }

    fn evict(&self, key: String) {
        todo!()
    }

    fn load(&self, key: String) -> Box<dyn Future<Output = Bytes>> {
        todo!()
    }

    fn load_string(
        &self,
        key: String,
        cache: bool, /*= true*/
    ) -> Box<dyn Future<Output = String>> {
        todo!()
    }
}

pub struct PlatformAssetBundle {}

impl CachingAssetBundle for PlatformAssetBundle {}

impl AssetBundle for PlatformAssetBundle {
    fn clear(&self) {
        todo!()
    }

    fn evict(&self, key: String) {
        todo!()
    }

    fn load(&self, key: String) -> Box<dyn Future<Output = Bytes>> {
        todo!()
    }

    fn load_string(
        &self,
        key: String,
        cache: bool, /*= true*/
    ) -> Box<dyn Future<Output = String>> {
        todo!()
    }
}

pub trait CachingAssetBundle: AssetBundle {}

pub trait AssetBundle {
    // If this is a caching asset bundle, clear all cached data.
    fn clear(&self);

    // If this is a caching asset bundle, and the given key describes a cached asset,
    // then evict the asset from the cache so that the next time it is loaded, the cache will be reread from the asset bundle.
    fn evict(&self, key: String);

    // Retrieve a binary resource from the asset bundle as a data stream.
    fn load(&self, key: String) -> Box<dyn Future<Output = Bytes>>;

    // Retrieve a string from the asset bundle.
    fn load_string(
        &self,
        key: String,
        cache: bool, /*= true*/
    ) -> Box<dyn Future<Output = String>>;

    // // Retrieve a string from the asset bundle, parse it with the given function, and return the function's result.
    // fn load_structured_data<T>(
    //     &self,
    //     key: String,
    //     parser: Box<dyn Fn(String) -> Box<dyn Future<Output = T>>>,
    // ) -> Box<dyn Future<Output = T>>;
}

// impl<T: Default> Default for AssetBundle {
//     fn default() -> Self {
//         Self
//     }
// }
