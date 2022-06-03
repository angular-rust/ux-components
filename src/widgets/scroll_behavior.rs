/*
buildOverscrollIndicator(BuildContext context, Widget child, ScrollableDetails details) -> Widget
Applies a GlowingOverscrollIndicator to the child widget on TargetPlatform.android and TargetPlatform.fuchsia.

buildScrollbar(BuildContext context, Widget child, ScrollableDetails details) -> Widget
Applies a RawScrollbar to the child widget on desktop platforms.

buildViewportChrome(BuildContext context, Widget child, AxisDirection axisDirection) -> Widget
Wraps the given widget, which scrolls in the given AxisDirection.

@Deprecated("Migrate to buildOverscrollIndicator. " "This feature was deprecated after v2.1.0-11.0.pre.")
copyWith({bool scrollbars = true, bool overscroll = true, Set<PointerDeviceKind>? dragDevices, ScrollPhysics? physics, TargetPlatform? platform}) -> ScrollBehavior
Creates a copy of this ScrollBehavior, making it possible to easily toggle scrollbar and overscrollIndicator effects.

getPlatform(BuildContext context) -> TargetPlatform
The platform whose scroll physics should be implemented.

getScrollPhysics(BuildContext context) -> ScrollPhysics
The scroll physics to use for the platform given by getPlatform.

shouldNotify(covariant ScrollBehavior oldDelegate) -> bool
Called whenever a ScrollConfiguration is rebuilt with a new ScrollBehavior of the same runtimeType.

toString() -> String
A string representation of this object.
override

velocityTrackerBuilder(BuildContext context) -> GestureVelocityTrackerBuilder
Specifies the type of velocity tracker to use in the descendant Scrollables drag gesture recognizers, for estimating the velocity of a drag gesture.
*/
pub struct ScrollBehavior;

impl Default for ScrollBehavior {
    fn default() -> Self {
        Self {}
    }
}
