use super::ScrollPosition;

/*
addListener(VoidCallback listener) -> void
Register a closure to be called when the object changes.
inherited
animateTo(double offset, {required Duration duration, required Curve curve}) -> Future<void>
Animates the position from its current value to the given value.
attach(ScrollPosition position) -> void
Register the given position with this controller.
createScrollPosition(ScrollPhysics physics, ScrollContext context, ScrollPosition? oldPosition) -> ScrollPosition
Creates a ScrollPosition for use by a Scrollable widget.
debugFillDescription(List<String> description) -> void
Add additional information to the given description for use by toString.
@mustCallSuper
detach(ScrollPosition position) -> void
Unregister the given position with this controller.
dispose() -> void
Discards any resources used by the object. After this is called, the object is not in a usable state and should be discarded (calls to addListener and removeListener will throw after the object is disposed).
override
jumpTo(double value) -> void
Jumps the scroll position from its current value to the given value, without animation, and without checking if the new value is in range.
noSuchMethod(Invocation invocation) -> dynamic
Invoked when a non-existent method or property is accessed.
inherited
notifyListeners() -> void
Call all the registered listeners.
@protected, @visibleForTesting, inherited
removeListener(VoidCallback listener) -> void
Remove a previously registered closure from the list of closures that are notified when the object changes.
inherited
toString() -> String
A string representation of this object.
override
*/
pub struct ScrollController {
    // Whether any ScrollPosition objects have attached themselves to the ScrollController using the attach method.
    pub has_clients: bool,

    // Whether any listeners are currently registered.
    pub has_listeners: bool,

    // The initial value to use for offset.
    pub initial_scroll_offset: f32,

    // Each time a scroll completes, save the current scroll offset with PageStorage and restore it if this controller's scrollable is recreated.
    pub keep_scroll_offset: bool,

    // The current scroll offset of the scrollable widget.
    pub offset: f32,

    // Returns the attached ScrollPosition, from which the actual scroll offset of the ScrollView can be obtained.
    pub position: ScrollPosition,

    // The currently attached positions.
    // pub positions: Iterable<ScrollPosition>,
}

impl Default for ScrollController {
    fn default() -> Self {
        Self {
            has_clients: Default::default(),
            has_listeners: Default::default(),
            initial_scroll_offset: Default::default(),
            keep_scroll_offset: Default::default(),
            offset: Default::default(),
            position: Default::default(),
            // positions: Default::default(),
        }
    }
}
