use super::{NavigatorState, Route};

pub trait RouteListFactory<T> {
    fn on_generate_initial_routes(
        &self,
        navigator: NavigatorState,
        initial_route: String,
    ) -> Vec<Route<T>>;
}
