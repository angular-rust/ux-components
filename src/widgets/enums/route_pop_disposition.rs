#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutePopDisposition {
    // Pop the route.
    // If Route.willPop returns pop then the back button will actually pop the current route.
    Pop = 0,
    // Do not pop the route.
    // If Route.willPop returns doNotPop then the back button will be ignored.
    DoNotPop = 1,
    // Delegate this to the next level of navigation.
    // If Route.willPop returns bubble then the back button will be handled by the SystemNavigator, which will usually close the application.
    Bubble = 2,
}

impl Default for RoutePopDisposition {
    fn default() -> Self {
        Self::Pop
    }
}
