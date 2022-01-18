use super::Route;

pub trait PopPageCallback<T> {
    fn on_pop_page(&self, route: Route<T>, result: T) -> bool;
}
