use super::Route;

pub trait PopPageCallback<T> {
    fn on_pop_page(&self, route: Box<dyn Route>, result: T) -> bool;
}
