use super::{NavigatorState, Route};

pub trait RouteListFactory {
    fn on_generate_initial_routes(
        &self,
        navigator: NavigatorState,
        initial_route: String,
    ) -> Vec<Box<dyn Route>>;
}
