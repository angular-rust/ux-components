use crate::foundation::Key;

use super::{
    NavigatorObserver, Page, PopPageCallback, RouteFactory, RouteListFactory, TransitionDelegate,
};

// createState() → NavigatorState
// Creates the mutable state for this widget at a given location in the tree. [...]
// override

// inherit StatefulWidget

pub struct Navigator<T> {
    key: Key,
    pages: Vec<Page>,
    on_pop_page: Option<Box<dyn PopPageCallback<T>>>,
    initial_route: String,
    on_generate_initial_routes: Box<dyn RouteListFactory<T>>,
    on_generate_route: Option<Box<dyn RouteFactory<T>>>,
    on_unknown_route: Option<Box<dyn RouteFactory<T>>>,
    transition_delegate: TransitionDelegate,
    reports_route_update_to_engine: bool,
    observers: Vec<NavigatorObserver>,
    restoration_scope_id: String,
}
