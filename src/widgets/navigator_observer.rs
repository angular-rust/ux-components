// didPop(Route route, Route? previousRoute) -> void
// The Navigator popped route.
//
// didPush(Route route, Route? previousRoute) -> void
// The Navigator pushed route.
//
// didRemove(Route route, Route? previousRoute) -> void
// The Navigator removed route.
//
// didReplace({Route? newRoute, Route? oldRoute}) -> void
// The Navigator replaced oldRoute with newRoute.
//
// didStartUserGesture(Route route, Route? previousRoute) -> void
// The Navigator's routes are being moved by a user gesture.
//
// didStopUserGesture() -> void
// User gesture is no longer controlling the Navigator.

pub struct NavigatorObserver();
