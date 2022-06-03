pub struct StreamController<T> {
    pub data: T,

    // done → Future
    // A future which is completed when the stream controller is done sending events. [...]
    // read-only, override
    // hashCode → int
    // The hash code for this object. [...]
    // read-only, inherited
    // hasListener → bool
    // Whether there is a subscriber on the Stream.
    // read-only
    // isClosed → bool
    // Whether the stream controller is closed for adding more events. [...]
    // read-only
    // isPaused → bool
    // Whether the subscription would need to buffer events. [...]
    // read-only
    // onCancel ↔ (FutureOr<void> Function?()?)
    // The callback which is called when the stream is canceled. [...]
    // read / write
    // onListen ↔ (void Function?()?)
    // The callback which is called when the stream is listened to. [...]
    // read / write
    // onPause ↔ (void Function?()?)
    // The callback which is called when the stream is paused. [...]
    // read / write
    // onResume ↔ (void Function?()?)
    // The callback which is called when the stream is resumed. [...]
    // read / write
    // runtimeType → Type
    // A representation of the runtime type of the object.
    // read-only, inherited
    // sink → StreamSink<T>
    // Returns a view of this object that only exposes the StreamSink interface.
    // read-only
    // stream → Stream<T>
    // The stream that this controller is controlling.
    // read-only
}