/*
activate() -> void
Called when this object is reinserted into the tree after having been removed via deactivate.
override

build(BuildContext context) -> Widget
Describes the part of the user interface represented by this widget.
override

canPop() -> bool
Whether the navigator can be popped.
deactivate() -> void

Called when this object is removed from the tree.
override

didChangeDependencies() -> void
Called when a dependency of this State object changes.
override

didStartUserGesture() -> void
The navigator is being controlled by a user gesture.

didStopUserGesture() -> void
A user gesture completed.

didToggleBucket(RestorationBucket? oldBucket) -> void
Called when bucket switches between null and non-null values.
override

didUpdateWidget(covariant Navigator oldWidget) -> void
Called whenever the widget configuration changes.
override

dispose() -> void
Called when this object is removed from the tree permanently.
override

finalizeRoute(Route route) -> void
Complete the lifecycle for a route that has been popped off the navigator.

initState() -> void
Called when this object is inserted into the tree.
override

maybePop<T extends Object?>([T? result]) -> Future<bool>
Consults the current route's Route.willPop method, and acts accordingly, potentially popping the route as a result; returns whether the pop request should be considered handled.
@optionalTypeArgs

pop<T extends Object?>([T? result]) -> void
Pop the top-most route off the navigator.
@optionalTypeArgs

popAndPushNamed<T extends Object?, TO extends Object?>(String routeName, {TO? result, Object? arguments}) -> Future<T?>
Pop the current route off the navigator and push a named route in its place.
@optionalTypeArgs

popUntil(RoutePredicate predicate) -> void
Calls pop repeatedly until the predicate returns true.

push<T extends Object?>(Route<T> route) -> Future<T?>
Push the given route onto the navigator.
@optionalTypeArgs

pushAndRemoveUntil<T extends Object?>(Route<T> newRoute, RoutePredicate predicate) -> Future<T?>
Push the given route onto the navigator, and then remove all the previous routes until the predicate returns true.
@optionalTypeArgs

pushNamed<T extends Object?>(String routeName, {Object? arguments}) -> Future<T?>
Push a named route onto the navigator.
@optionalTypeArgs

pushNamedAndRemoveUntil<T extends Object?>(String newRouteName, RoutePredicate predicate, {Object? arguments}) -> Future<T?>
Push the route with the given name onto the navigator, and then remove all the previous routes until the predicate returns true.
@optionalTypeArgs

pushReplacement<T extends Object?, TO extends Object?>(Route<T> newRoute, {TO? result}) -> Future<T?>
Replace the current route of the navigator by pushing the given route and then disposing the previous route once the new route has finished animating in.
@optionalTypeArgs

pushReplacementNamed<T extends Object?, TO extends Object?>(String routeName, {TO? result, Object? arguments}) -> Future<T?>
Replace the current route of the navigator by pushing the route named routeName and then disposing the previous route once the new route has finished animating in.

removeRoute(Route route) -> void
Immediately remove route from the navigator, and Route.dispose it.

removeRouteBelow(Route anchorRoute) -> void
Immediately remove a route from the navigator, and Route.dispose it. The route to be removed is the one below the given anchorRoute.

replace<T extends Object?>({required Route oldRoute, required Route<T> newRoute}) -> void
Replaces a route on the navigator with a new route.
@optionalTypeArgs

replaceRouteBelow<T extends Object?>({required Route anchorRoute, required Route<T> newRoute}) -> void
Replaces a route on the navigator with a new route. The route to be replaced is the one below the given anchorRoute.
@optionalTypeArgs

restorablePopAndPushNamed<T extends Object?, TO extends Object?>(String routeName, {TO? result, Object? arguments}) -> String
Pop the current route off the navigator and push a named route in its place.
@optionalTypeArgs

restorablePush<T extends Object?>(RestorableRouteBuilder<T> routeBuilder, {Object? arguments}) -> String
Push a new route onto the navigator.
@optionalTypeArgs

restorablePushAndRemoveUntil<T extends Object?>(RestorableRouteBuilder<T> newRouteBuilder, RoutePredicate predicate, {Object? arguments}) -> String
Push a new route onto the navigator, and then remove all the previous routes until the predicate returns true.
@optionalTypeArgs

restorablePushNamed<T extends Object?>(String routeName, {Object? arguments}) -> String
Push a named route onto the navigator.
@optionalTypeArgs

restorablePushNamedAndRemoveUntil<T extends Object?>(String newRouteName, RoutePredicate predicate, {Object? arguments}) -> String
Push the route with the given name onto the navigator, and then remove all the previous routes until the predicate returns true.
@optionalTypeArgs

restorablePushReplacement<T extends Object?, TO extends Object?>(RestorableRouteBuilder<T> routeBuilder, {TO? result, Object? arguments}) -> String
Replace the current route of the navigator by pushing a new route and then disposing the previous route once the new route has finished animating in.
@optionalTypeArgs

restorablePushReplacementNamed<T extends Object?, TO extends Object?>(String routeName, {TO? result, Object? arguments}) -> String
Replace the current route of the navigator by pushing the route named routeName and then disposing the previous route once the new route has finished animating in.
@optionalTypeArgs

restorableReplace<T extends Object?>({required Route oldRoute, required RestorableRouteBuilder<T> newRouteBuilder, Object? arguments}) -> String
Replaces a route on the navigator with a new route.
@optionalTypeArgs

restorableReplaceRouteBelow<T extends Object?>({required Route anchorRoute, required RestorableRouteBuilder<T> newRouteBuilder, Object? arguments}) -> String
Replaces a route on the navigator with a new route. The route to be replaced is the one below the given anchorRoute.
@optionalTypeArgs

restoreState(RestorationBucket? oldBucket, bool initialRestore) -> void
Called to initialize or restore the RestorablePropertys used by the State object.
*/

// State<Navigator>
pub struct NavigatorState;

impl Default for NavigatorState {
    fn default() -> Self {
        Self {}
    }
}
