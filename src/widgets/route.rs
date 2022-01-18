use std::marker::PhantomData;

use super::RouteSettings;

// changedExternalState() -> void
// Called whenever the Navigator has updated in some manner that might affect routes, to indicate that the route may wish to rebuild as well.
// @mustCallSuper, @protected
//
// changedInternalState() -> void
// Called whenever the internal state of the route has changed.
// @mustCallSuper, @protected
//
// didAdd() -> void
// Called after install when the route is added to the navigator.
// @mustCallSuper, @protected
//
// didChangeNext(Route? nextRoute) -> void
// This route's next route has changed to the given new route.
// @mustCallSuper, @protected
//
// didChangePrevious(Route? previousRoute) -> void
// This route's previous route has changed to the given new route.
// @mustCallSuper, @protected
//
// didComplete(T? result) -> void
// The route was popped or is otherwise being removed somewhat gracefully.
// @mustCallSuper, @protected
//
// didPop(T? result) -> bool
// A request was made to pop this route. If the route can handle it internally (e.g. because it has its own stack of internal state) then return false, otherwise return true (by returning the value of calling super.didPop). Returning false will prevent the default behavior of NavigatorState.pop.
// @mustCallSuper
//
// didPopNext(Route nextRoute) -> void
// The given route, which was above this one, has been popped off the navigator.
// @mustCallSuper, @protected
//
// didPush() -> TickerFuture
// Called after install when the route is pushed onto the navigator.
// @mustCallSuper, @protected
//
// didReplace(Route? oldRoute) -> void
// Called after install when the route replaced another in the navigator.
// @mustCallSuper, @protected
//
// dispose() -> void
// Discards any resources used by the object.
// @mustCallSuper, @protected
//
// install() -> void
// Called when the route is inserted into the navigator.
// @mustCallSuper, @protected
//
// willPop() -> Future<RoutePopDisposition>
// Returns whether calling Navigator.maybePop when this Route is current (isCurrent) should do anything.

// TODO: String generic should be fixed
pub struct Route<T> {
    settings: RouteSettings<String>,
    phantom: PhantomData<T>,
}
