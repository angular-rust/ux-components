use crate::painting::{Axis, AxisDirection};

/*
absorb(ScrollPosition other) -> void
Take any current applicable state from the given ScrollPosition.
@mustCallSuper, @protected
addListener(VoidCallback listener) -> void
Register a closure to be called when the object changes.
inherited
animateTo(double to, {required Duration duration, required Curve curve}) -> Future<void>
Animates the position from its current value to the given value.
override
applyBoundaryConditions(double value) -> double
Returns the overscroll by applying the boundary conditions.
@protected
applyContentDimensions(double minScrollExtent, double maxScrollExtent) -> bool
Called when the viewport's content extents are established.
override
applyNewDimensions() -> void
Notifies the activity that the dimensions of the underlying viewport or contents have changed.
@mustCallSuper, @protected
applyViewportDimension(double viewportDimension) -> bool
Called when the viewport's extents are established.
override
beginActivity(ScrollActivity? newActivity) -> void
Change the current activity, disposing of the old one and sending scroll notifications as necessary.
copyWith({double? minScrollExtent, double? maxScrollExtent, double? pixels, double? viewportDimension, AxisDirection? axisDirection}) -> ScrollMetrics
Creates a ScrollMetrics that has the same properties as this object.
inherited
correctBy(double correction) -> void
Apply a layout-time correction to the scroll offset.
override
correctForNewDimensions(ScrollMetrics oldPosition, ScrollMetrics newPosition) -> bool
Verifies that the new content and viewport dimensions are acceptable.
@protected
correctPixels(double value) -> void
Change the value of pixels to the new value, without notifying any customers.
debugFillDescription(List<String> description) -> void
Add additional information to the given description for use by toString.
override
didEndScroll() -> void
Called by beginActivity to report when an activity has ended.
didOverscrollBy(double value) -> void
Called by setPixels to report overscroll when an attempt is made to change the pixels position. Overscroll is the amount of change that was not applied to the pixels value.
didStartScroll() -> void
Called by beginActivity to report when an activity has started.
didUpdateScrollDirection(ScrollDirection direction) -> void
Dispatches a notification that the userScrollDirection has changed.
didUpdateScrollMetrics() -> void
Dispatches a notification that the ScrollMetrics have changed.
didUpdateScrollPositionBy(double delta) -> void
Called by setPixels to report a change to the pixels position.
dispose() -> void
Discards any resources used by the object. After this is called, the object is not in a usable state and should be discarded (calls to addListener and removeListener will throw after the object is disposed).
override
drag(DragStartDetails details, VoidCallback dragCancelCallback) -> Drag
Start a drag activity corresponding to the given DragStartDetails.
ensureVisible(RenderObject object, {double alignment = 0.0, Duration duration = Duration.zero, Curve curve = Curves.ease, ScrollPositionAlignmentPolicy alignmentPolicy = ScrollPositionAlignmentPolicy.explicit, RenderObject? targetRenderObject}) -> Future<void>
Animates the position such that the given object is as visible as possible by just scrolling this position.
forcePixels(double value) -> void
Change the value of pixels to the new value, and notify any customers, but without honoring normal conventions for changing the scroll offset.
@protected
hold(VoidCallback holdCancelCallback) -> ScrollHoldController
Stop the current activity and start a HoldScrollActivity.
jumpTo(double value) -> void
Jumps the scroll position from its current value to the given value, without animation, and without checking if the new value is in range.
override
jumpToWithoutSettling(double value) -> void
Deprecated. Use jumpTo or a custom ScrollPosition instead.
@Deprecated('This will lead to bugs.')
moveTo(double to, {Duration? duration, Curve? curve, bool? clamp = true}) -> Future<void>
Calls jumpTo if duration is null or Duration.zero, otherwise animateTo is called.
override
noSuchMethod(Invocation invocation) -> dynamic
Invoked when a non-existent method or property is accessed.
inherited
notifyListeners() -> void
Call all the registered listeners.
override
pointerScroll(double delta) -> void
Changes the scrolling position based on a pointer signal from current value to delta without animation and without checking if new value is in range, taking min/max scroll extent into account.
recommendDeferredLoading(BuildContext context) -> bool
Provides a heuristic to determine if expensive frame-bound tasks should be deferred.
removeListener(VoidCallback listener) -> void
Remove a previously registered closure from the list of closures that are notified when the object changes.
inherited
restoreOffset(double offset, {bool initialRestore = false}) -> void
Called by context to restore the scroll offset to the provided value.
restoreScrollOffset() -> void
Called whenever the ScrollPosition is created, to restore the scroll offset if possible.
@protected
saveOffset() -> void
Called whenever scrolling ends, to persist the current scroll offset for state restoration purposes.
@protected
saveScrollOffset() -> void
Called whenever scrolling ends, to store the current scroll offset in a storage mechanism with a lifetime that matches the app's lifetime.
@protected
setPixels(double newPixels) -> double
Update the scroll position (pixels) to a given pixel value.
toString() -> String
A string representation of this object.
inherited
*/
pub struct ScrollPosition {
    // The currently operative ScrollActivity.
    // pub activity: ScrollActivity,

    // Whether a viewport is allowed to change pixels implicitly to respond to a call to RenderObject.showOnScreen.
    pub allow_implicit_scrolling: bool,

    // Whether the pixels value is exactly at the minScrollExtent or the maxScrollExtent.
    pub at_edge: bool,

    // The axis in which the scroll view scrolls.
    pub axis: Axis,

    // The direction in which the scroll view scrolls.
    pub axis_direction: AxisDirection,

    // Where the scrolling is taking place.
    // pub context: ScrollContext,

    // The quantity of content conceptually "below" the viewport in the scrollable. This is the content below the content described by extentInside.
    pub extent_after: f32,

    // The quantity of content conceptually "above" the viewport in the scrollable. This is the content above the content described by extentInside.
    pub extent_before: f32,

    // The quantity of content conceptually "inside" the viewport in the scrollable.
    pub extent_inside: f32,

    // Whether the minScrollExtent and the maxScrollExtent properties are available.
    pub has_content_dimensions: bool,

    // Whether any listeners are currently registered.
    pub has_listeners: bool,

    // Whether the pixels property is available.
    pub has_pixels: bool,

    // Whether the viewportDimension property is available.
    pub has_viewport_dimension: bool,

    // Whether viewportDimension, minScrollExtent, maxScrollExtent, outOfRange, and atEdge are available.
    pub have_dimensions: bool,

    // This notifier's value is true if a scroll is underway and false if the scroll position is idle.
    // pub is_scrolling_notifier: ValueNotifier<bool>,

    // Save the current scroll offset with PageStorage and restore it if this scroll position's scrollable is recreated.
    pub keep_scroll_offset: bool,

    // The maximum in-range value for pixels.
    pub max_scroll_extent: f32,

    // The minimum in-range value for pixels.
    pub min_scroll_extent: f32,

    // Whether the pixels value is outside the minScrollExtent and maxScrollExtent.
    pub out_of_range: bool,

    // How the scroll position should respond to user input.
    // pub physics: ScrollPhysics,

    // The number of pixels to offset the children in the opposite of the axis direction.
    pub pixels: f32,

    // The direction in which the user is trying to change pixels, relative to the viewport's RenderViewportBase.axisDirection.
    // pub user_scroll_direction: ScrollDirection,

    // The extent of the viewport along the axisDirection.
    pub viewport_dimension: f32,
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self {
            // activity: Default::default(),
            allow_implicit_scrolling: Default::default(),
            at_edge: Default::default(),
            axis: Default::default(),
            axis_direction: Default::default(),
            // context: Default::default(),
            extent_after: Default::default(),
            extent_before: Default::default(),
            extent_inside: Default::default(),
            has_content_dimensions: Default::default(),
            has_listeners: Default::default(),
            has_pixels: Default::default(),
            has_viewport_dimension: Default::default(),
            have_dimensions: Default::default(),
            // is_scrolling_notifier: Default::default(),
            keep_scroll_offset: Default::default(),
            max_scroll_extent: Default::default(),
            min_scroll_extent: Default::default(),
            out_of_range: Default::default(),
            // physics: Default::default(),
            pixels: Default::default(),
            // user_scroll_direction: Default::default(),
            viewport_dimension: Default::default(),
        }
    }
}
