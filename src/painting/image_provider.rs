/*
createStream(ImageConfiguration configuration) -> ImageStream
Called by resolve to create the ImageStream it returns.
@protected
evict({ImageCache? cache, ImageConfiguration configuration = ImageConfiguration.empty}) -> Future<bool>
Evicts an entry from the image cache.
load(T key, DecoderCallback decode) -> ImageStreamCompleter
Converts a key into an ImageStreamCompleter, and begins fetching the image.
obtainCacheStatus({required ImageConfiguration configuration, ImageErrorListener? handleError}) -> Future<ImageCacheStatus?>
Returns the cache location for the key that this ImageProvider creates.
obtainKey(ImageConfiguration configuration) -> Future<T>
Converts an ImageProvider's settings plus an ImageConfiguration to a key that describes the precise image to load.
resolve(ImageConfiguration configuration) -> ImageStream
Resolves this image provider using the given configuration, returning an ImageStream.
@nonVirtual
resolveStreamForKey(ImageConfiguration configuration, ImageStream stream, T key, ImageErrorListener handleError) -> void
Called by resolve with the key returned by obtainKey.
*/
pub struct ImageProvider<T: Default>(T);

impl<T: Default> Default for ImageProvider<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
