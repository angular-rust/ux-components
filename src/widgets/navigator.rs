#![allow(unused_imports)]
use std::future::Future;

use crate::foundation::Key;

use super::{
    BuildContext, NavigatorObserver, NavigatorState, Page, PopPageCallback, Route, RouteFactory,
    RouteListFactory, TransitionDelegate,
};

// Widget > StatefulWidget
pub struct Navigator {
    key: Key,
    pages: Vec<Page>,
    // on_pop_page: Option<Box<dyn PopPageCallback<T>>>,
    initial_route: String,
    // on_generate_initial_routes: Box<dyn RouteListFactory<T>>,
    // on_generate_route: Option<Box<dyn RouteFactory<T>>>,
    // on_unknown_route: Option<Box<dyn RouteFactory<T>>>,
    transition_delegate: TransitionDelegate,
    reports_route_update_to_engine: bool,
    observers: Vec<NavigatorObserver>,
    restoration_scope_id: String,
}

pub struct RoutePredicate;

pub struct Object;

pub struct RestorableRouteBuilder {}

impl Navigator {
    // self methods
    // Creates the mutable state for this widget at a given location in the tree.
    pub fn create_state(&self) -> NavigatorState {
        todo!()
    }

    // static methods

    // Whether the navigator that most tightly encloses the given context can be popped.
    pub fn can_pop(context: BuildContext) -> bool {
        todo!()
    }

    // Turn a route name into a set of Route objects.
    pub fn default_generate_initial_routes(
        navigator: NavigatorState,
        initial_route_name: String,
    ) -> Vec<Box<dyn Route>> {
        todo!()
    }

    // The state from the closest instance of this class that encloses the given context, if any.
    pub fn maybe_of(
        context: BuildContext,
        root_navigator: bool, /* = false*/
    ) -> Option<NavigatorState> {
        todo!()
    }

    // // Consults the current route's Route.willPop method, and acts accordingly, potentially popping the route as a result;
    // // returns whether the pop request should be considered handled.
    // pub fn maybe_pop<T>(
    //     context: BuildContext,
    //     result: T,
    // ) -> impl Future<Output = bool> {
    //     todo!()
    // }

    // The state from the closest instance of this class that encloses the given context.
    pub fn of(context: BuildContext, root_navigator: bool /* = false*/) -> NavigatorState {
        todo!()
    }

    // Pop the top-most route off the navigator that most tightly encloses the given context.
    pub fn pop<T>(context: BuildContext, result: T) {
        todo!()
    }

    // // Pop the current route off the navigator that most tightly encloses the given context and push a named route in its place.
    // pub fn pop_and_push_named<T, TO>(
    //     context: BuildContext,
    //     route_name: String,
    //     result: TO,
    //     arguments: Option<Object>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // Calls pop repeatedly on the navigator that most tightly encloses the given context until the predicate returns true.
    pub fn pop_until(context: BuildContext, predicate: RoutePredicate) {
        todo!()
    }

    // // Push the given route onto the navigator that most tightly encloses the given context.
    // pub fn push<T>(
    //     context: BuildContext,
    //     route: Box<dyn Route>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // // Push the given route onto the navigator that most tightly encloses the given context,
    // // and then remove all the previous routes until the predicate returns true.
    // pub fn push_and_remove_until<T>(
    //     context: BuildContext,
    //     new_route: Box<dyn Route>,
    //     predicate: RoutePredicate,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // // Push a named route onto the navigator that most tightly encloses the given context.
    // pub fn push_named<T>(
    //     context: BuildContext,
    //     route_name: String,
    //     arguments: Option<Object>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // // Push the route with the given name onto the navigator that most tightly encloses the given context,
    // // and then remove all the previous routes until the predicate returns true.
    // pub fn push_named_and_remove_until<T>(
    //     context: BuildContext,
    //     new_route_name: String,
    //     predicate: RoutePredicate,
    //     arguments: Option<Object>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // // Replace the current route of the navigator that most tightly encloses the given context by pushing the given route and then disposing
    // // the previous route once the new route has finished animating in.
    // pub fn push_replacement<T, TO>(
    //     context: BuildContext,
    //     new_route: Box<dyn Route>,
    //     result: Option<TO>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // // Replace the current route of the navigator that most tightly encloses the given context by pushing the route named route_name and
    // // then disposing the previous route once the new route has finished animating in.
    // pub fn push_replacement_named<T, TO>(
    //     context: BuildContext,
    //     route_name: String,
    //     result: Option<TO>,
    //     arguments: Option<Object>,
    // ) -> impl Future<Output = T> {
    //     todo!()
    // }

    // Immediately remove route from the navigator that most tightly encloses the given context, and Route.dispose it.
    pub fn remove_route(context: BuildContext, route: Box<dyn Route>) {
        todo!()
    }

    // Immediately remove a route from the navigator that most tightly encloses the given context, and Route.dispose it.
    // The route to be removed is the one below the given anchorRoute.
    pub fn remove_route_below(context: BuildContext, anchor_route: Box<dyn Route>) {
        todo!()
    }

    // Replaces a route on the navigator that most tightly encloses the given context with a new route.
    pub fn replace<T>(
        context: BuildContext,
        old_route: Box<dyn Route>,
        new_route: Box<dyn Route>,
    ) {
        todo!()
    }

    // Replaces a route on the navigator that most tightly encloses the given context with a new route.
    // The route to be replaced is the one below the given anchorRoute.
    pub fn replace_route_below<T>(
        context: BuildContext,
        anchor_route: Box<dyn Route>,
        new_route: Box<dyn Route>,
    ) {
        todo!()
    }

    // Pop the current route off the navigator that most tightly encloses the given context and push a named route in its place.
    pub fn restorable_pop_and_push_named<T, TO>(
        context: BuildContext,
        route_name: String,
        result: Option<TO>,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Push a new route onto the navigator that most tightly encloses the given context.
    pub fn restorable_push(
        context: BuildContext,
        route_builder: RestorableRouteBuilder,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Push a new route onto the navigator that most tightly encloses the given context,
    // and then remove all the previous routes until the predicate returns true.
    pub fn restorable_push_and_remove_until(
        context: BuildContext,
        new_route_builder: RestorableRouteBuilder,
        predicate: RoutePredicate,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Push a named route onto the navigator that most tightly encloses the given context.
    pub fn restorable_push_named<T>(
        context: BuildContext,
        route_name: String,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Push the route with the given name onto the navigator that most tightly encloses the given context,
    // and then remove all the previous routes until the predicate returns true.
    pub fn restorable_push_named_and_remove_until(
        context: BuildContext,
        new_route_name: String,
        predicate: RoutePredicate,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Replace the current route of the navigator that most tightly encloses the given context by pushing a new route and then
    // disposing the previous route once the new route has finished animating in.
    pub fn restorable_push_replacement<T, TO>(
        context: BuildContext,
        route_builder: RestorableRouteBuilder,
        result: Option<TO>,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Replace the current route of the navigator that most tightly encloses the given context by pushing the route named route_name and then
    // disposing the previous route once the new route has finished animating in.
    pub fn restorable_push_replacement_named<T, TO>(
        context: BuildContext,
        route_name: String,
        result: Option<TO>,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Replaces a route on the navigator that most tightly encloses the given context with a new route.
    pub fn restorable_replace<T>(
        context: BuildContext,
        old_route: Box<dyn Route>,
        new_route_builder: RestorableRouteBuilder,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }

    // Replaces a route on the navigator that most tightly encloses the given context with a new route.
    // The route to be replaced is the one below the given anchorRoute.
    pub fn restorable_replace_route_below<T>(
        context: BuildContext,
        anchor_route: Box<dyn Route>,
        new_route_builder: RestorableRouteBuilder,
        arguments: Option<Object>,
    ) -> String {
        todo!()
    }
}
