use super::{Route, RouteSettings};

pub trait RouteFactory<T> {
    fn on_generate_route(&self, settings: RouteSettings<T>) -> Option<Box<dyn Route>>;
}
