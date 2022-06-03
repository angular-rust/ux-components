use super::Route;

pub trait InitialRouteListFactory {
    // TODO: String generic should be fixed
    fn on_generate_initial_routes(&self, initial_route: String) -> Vec<Box<dyn Route>>;
}
