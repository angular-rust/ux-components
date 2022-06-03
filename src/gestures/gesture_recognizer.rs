#[derive(Default)]
pub struct DeviceGestureSettings {
    // The touch slop value for pan gestures, in logical pixels, or null if it was not set.
    pub pan_slop: f32,

    // The touch slop value in logical pixels, or null if it was not set.
    pub touch_slop: f32,
}

#[derive(Default)]
pub struct GestureRecognizer {
    // Returns a very short pretty description of the gesture that the recognizer looks for, like 'tap' or 'horizontal drag'.
    pub debug_description: String,

    // Optional device specific configuration for device gestures that will take precedence over framework defaults.
    pub gesture_settings: DeviceGestureSettings,
}

// acceptGesture(int pointer) → void
// Called when this member wins the arena for the given pointer id.
// inherited
// addAllowedPointer(PointerDownEvent event) → void
// Registers a new pointer that's been checked to be allowed by this gesture recognizer. [...]
// @protected
// addPointer(PointerDownEvent event) → void
// Registers a new pointer that might be relevant to this gesture detector. [...]
// debugDescribeChildren() → List<DiagnosticsNode>
// Returns a list of DiagnosticsNode objects describing this node's children. [...]
// inherited
// debugFillProperties(DiagnosticPropertiesBuilder properties) → void
// Add additional properties associated with the node. [...]
// override
// dispose() → void
// Releases any resources used by the object. [...]
// @mustCallSuper
// getKindForPointer(int pointer) → PointerDeviceKind
// For a given pointer ID, returns the device kind associated with it. [...]
// @protected
// handleNonAllowedPointer(PointerDownEvent event) → void
// Handles a pointer being added that's not allowed by this recognizer. [...]
// @protected
// invokeCallback<T>(String name, RecognizerCallback<T> callback, {String debugReport()?}) → T?
// Invoke a callback provided by the application, catching and logging any exceptions. [...]
// @protected
// isPointerAllowed(PointerDownEvent event) → bool
// Checks whether or not a pointer is allowed to be tracked by this recognizer.
// @protected
// noSuchMethod(Invocation invocation) → dynamic
// Invoked when a non-existent method or property is accessed. [...]
// inherited
// rejectGesture(int pointer) → void
// Called when this member loses the arena for the given pointer id.
// inherited
// toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) → DiagnosticsNode
// Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep. [...]
// inherited
// toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) → String
// A string representation of this object. [...]
// inherited
// toStringDeep({String prefixLineOne = '', String? prefixOtherLines, DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
// Returns a string representation of this node and its descendants. [...]
// inherited
// toStringShallow({String joiner = ', ', DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
// Returns a one-line detailed description of the object. [...]
// inherited
// toStringShort() → String
// A brief description of this object, usually just the runtimeType and the hashCode. [...]
// inherited
