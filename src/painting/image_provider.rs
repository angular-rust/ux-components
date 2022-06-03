use std::future::Future;

use crate::{
    engine::d2,
    foundation::{Key, TargetPlatform},
    services::AssetBundle,
    ui::{Codec, Locale, Size, TextDirection},
};

pub struct ImageConfiguration {
    // The preferred AssetBundle to use if the ImageProvider needs one and does not have one already selected.
    pub bundle: Option<Box<dyn AssetBundle>>,

    // The device pixel ratio where the image will be shown.
    pub device_pixel_ratio: Option<f32>,

    // The language and region for which to select the image.
    pub locale: Option<Locale>,

    // The TargetPlatform for which assets should be used.
    // This allows images to be specified in a platform-neutral fashion yet use different assets on different platforms,
    // to match local conventions e.g. for color matching or shadows.
    pub platform: Option<TargetPlatform>,

    // The size at which the image will be rendered.
    pub size: Option<Size>,

    // The reading direction of the language for which to select the image.
    pub text_direction: Option<TextDirection>,
}

impl Default for ImageConfiguration {
    fn default() -> Self {
        Self {
            bundle: Default::default(),
            device_pixel_ratio: Default::default(),
            locale: Default::default(),
            platform: Default::default(),
            size: Default::default(),
            text_direction: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    // The raw image pixels.
    pub image: d2::Image,

    // The linear scale factor for drawing this image at its intended size.
    pub scale: f32,

    // The size of raw image pixels in bytes.
    pub size_bytes: i32,
}

impl ImageInfo {
    // Creates an ImageInfo with a cloned image.
    pub fn clone() -> ImageInfo {
        todo!()
    }

    // Disposes of this object.
    pub fn dispose() {}

    // Whether this ImageInfo is a clone of the other.
    pub fn is_clone_of(other: ImageInfo) -> bool {
        todo!()
    }
}

pub struct ImageChunkEvent {
    // The number of bytes that have been received across the wire thus far.
    pub cumulative_bytes_loaded: i32,

    // The expected number of bytes that need to be received to finish loading the image.
    pub expected_total_bytes: Option<i32>,
}

// Fn(ImageChunkEvent event)
pub type ImageChunkListener = dyn FnMut(ImageChunkEvent);

// Fn(image: ImageInfo, synchronous_call: bool)
pub type ImageListener = dyn FnMut(ImageInfo, bool);

#[allow(dead_code)]
pub struct ImageStreamListener {
    // Callback for getting notified when a chunk of bytes has been received during the loading of the image.
    on_chunk: Option<Box<ImageChunkListener>>,

    // Callback for getting notified when an error occurs while loading an image.
    on_error: Option<Box<ImageErrorListener>>,

    // Callback for getting notified that an image is available.
    on_image: Box<ImageListener>,
}

impl ImageStreamListener {
    pub fn new(
        on_image: Box<ImageListener>,
        on_chunk: Option<Box<ImageChunkListener>>,
        on_error: Option<Box<ImageErrorListener>>,
    ) -> Self {
        Self {
            on_chunk,
            on_error,
            on_image,
        }
    }
}

impl Default for ImageStreamListener {
    fn default() -> Self {
        Self {
            on_chunk: Default::default(),
            on_error: Default::default(),
            on_image: box |_, _| {}, // NullObject
        }
    }
}

#[allow(dead_code)]
pub struct ImageStream {
    // The completer that has been assigned to this image stream.
    pub completer: Option<ImageStreamCompleter>,

    // Returns an object which can be used with == to determine if this ImageStream shares the same listeners list as another ImageStream.
    pub key: Key,

    // KISS
    pub image: Option<d2::Image>,
}

impl ImageStream {
    // Adds a listener callback that is called whenever a new concrete ImageInfo object is available.
    // If a concrete image is already available, this object will call the listener synchronously.
    pub fn add_listener(&self, listener: ImageStreamListener) {
        if let Some(image) = self.image {
            let mut handle = listener.on_image;
            handle(
                ImageInfo {
                    image,
                    scale: 1.0,
                    size_bytes: 0,
                },
                true,
            )
        }
    }

    // Stops listening for events from this stream's ImageStreamCompleter.
    pub fn remove_listener(&self, listener: ImageStreamListener) {}

    // Assigns a particular ImageStreamCompleter to this ImageStream.
    pub fn set_completer(&self, value: ImageStreamCompleter) {}
}

impl Default for ImageStream {
    fn default() -> Self {
        Self {
            completer: Default::default(),
            key: Default::default(),
            image: Default::default(),
        }
    }
}

pub struct ImageCacheStatus {
    // An image that has been submitted to ImageCache.putIfAbsent, has completed, fits based on the sizing rules of the cache, and has not been evicted.
    pub keep_alive: bool,

    // An image that has been submitted to ImageCache.putIfAbsent and has at least one listener on its ImageStreamCompleter.
    pub live: bool,

    // An image that has been submitted to ImageCache.putIfAbsent, but not yet completed.
    pub pending: bool,

    // An image that is tracked in some way by the ImageCache, whether pending, keepAlive, or live.
    pub tracked: bool,

    // An image that either has not been submitted to ImageCache.putIfAbsent or has otherwise been evicted from the keepAlive and live caches.
    pub untracked: bool,
}

// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

pub struct ImageCache {
    // The current number of cached entries.
    pub current_size: i32,

    // The current size of cached entries in bytes.
    pub current_size_bytes: i32,

    // The number of live images being held by the ImageCache.
    pub live_image_count: i32,

    // Maximum number of entries to store in the cache.
    pub maximum_size: i32,

    // Maximum size of entries to store in the cache in bytes.
    pub maximum_size_bytes: i32,

    // The number of images being tracked as pending in the ImageCache.
    pub pending_image_count: i32,
}

impl ImageCache {
    // Evicts all pending and keepAlive entries from the cache.
    pub fn clear() {}

    // Clears any live references to images in this cache.
    pub fn clear_live_images() {}

    // Returns whether this key has been previously added by putIfAbsent.
    pub fn contains_key(key: Key) -> bool {
        todo!()
    }

    // Evicts a single entry from the cache, returning true if successful.
    pub fn evict(key: Key, include_live: bool /* = true*/) -> bool {
        todo!()
    }

    // Returns the previously cached ImageStream for the given key, if available;
    // if not, calls the given callback to obtain it first. In either case, the key is moved to the 'most recently used' position.
    pub fn put_if_absent(
        key: Key,
        loader: ImageStreamCompleter,
        on_error: Option<Box<ImageErrorListener>>,
    ) -> Option<ImageStreamCompleter> {
        todo!()
    }

    // The ImageCacheStatus information for the given key.
    pub fn status_for_key(key: Key) -> ImageCacheStatus {
        todo!()
    }
}

// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

#[allow(dead_code)]
pub struct ImageStreamCompleter {
    // Whether any listeners are currently registered.
    has_listeners: bool,
}

impl ImageStreamCompleter {
    // Adds a listener callback that is called whenever a new concrete ImageInfo object is available or an error is reported.
    // If a concrete image is already available, or if an error has been already reported,
    // this object will notify the listener synchronously.
    pub fn add_listener(listener: ImageStreamListener) {}

    // Adds a callback to call when removeListener results in an empty list of listeners and there are no keepAlive handles outstanding.
    pub fn add_on_last_listener_removed_callback(callback: Box<dyn Fn()>) {}

    // // Creates an ImageStreamCompleterHandle that will prevent this stream from being disposed at least until the handle is disposed.
    // pub fn keep_alive() -> ImageStreamCompleterHandle {
    //     todo!()
    // }

    // Stops the specified listener from receiving image stream events.
    pub fn remove_listener(listener: ImageStreamListener) {}

    // Removes a callback previously supplied to addOnLastListenerRemovedCallback.
    pub fn remove_on_last_listener_removed_callback(callback: Box<dyn Fn()>) {}

    // // Calls all the registered error listeners to notify them of an error that occurred while resolving the image.
    // pub fn report_error(
    //     context: Option<DiagnosticsNode>,
    //     exception: Object,
    //     stack: Option<StackTrace>,
    //     information_collector: Option<InformationCollector>,
    //     silent: bool, /*= false*/
    // ) {
    // }

    // Calls all the registered ImageChunkListeners (listeners with an ImageStreamListener.onChunk specified) to notify them of a new ImageChunkEvent.
    pub fn report_image_chunk_event(event: ImageChunkEvent) {}

    // Calls all the registered listeners to notify them of a new image.
    pub fn set_image(image: ImageInfo) {}
}

impl Default for ImageStreamCompleter {
    fn default() -> Self {
        Self {
            has_listeners: Default::default(),
        }
    }
}

// Fn(bytes: Uint8List, cache_width: Option<i32>, cache_height: Option<i32>, allow_upscaling: bool) -> impl Future<Codec>
pub type DecoderCallback =
    dyn Fn(&[u8], Option<i32>, Option<i32>, bool) -> Box<dyn Future<Output = Codec>>;

// Fn(exception: Object, stack_trace: Option<StackTrace>)
pub type ImageErrorListener = dyn Fn();

// Key is Generic here
pub trait ImageProvider /*<T>*/ {
    // Called by resolve to create the ImageStream it returns.
    fn create_stream(&self, configuration: ImageConfiguration) -> ImageStream;

    // Evicts an entry from the image cache.
    fn evict(
        &self,
        cache: Option<ImageCache>,
        configuration: ImageConfiguration, /*= ImageConfiguration.empty*/
    ) -> Box<dyn Future<Output = bool>>;

    // Converts a key into an ImageStreamCompleter, and begins fetching the image.
    fn load(&self, key: Key, decode: Box<DecoderCallback>) -> ImageStreamCompleter;

    // Returns the cache location for the key that this ImageProvider creates.
    fn obtain_cache_status(
        &self,
        configuration: ImageConfiguration,
        handle_error: Option<Box<ImageErrorListener>>,
    ) -> Box<dyn Future<Output = Option<ImageCacheStatus>>>;

    // Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.
    fn obtain_key(&self, configuration: ImageConfiguration) -> Box<dyn Future<Output = Key>>;

    // Resolves this image provider using the given configuration, returning an ImageStream.
    fn resolve(&self, configuration: ImageConfiguration) -> ImageStream;

    // Called by resolve with the key returned by obtainKey.
    fn resolve_stream_for_key(
        &self,
        configuration: ImageConfiguration,
        stream: ImageStream,
        key: Key,
        handle_error: Option<Box<ImageErrorListener>>,
    );
}
