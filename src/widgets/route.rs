#![allow(unused_imports)]
use std::{marker::PhantomData, future::Future, time::Duration};

use crate::ui::VoidCallback;

use super::{RouteSettings, RoutePopDisposition};

// impl FutureExtensions<T> {
//     // Completely ignores this future and its result.
//     fn ignore();
    
//     // Handles errors on this future. 
//     onError<E extends Object>(FutureOr<T> handleError(E error, StackTrace stackTrace), {bool test(E error)?}) -> Future<T>    
// }

// located in scheduler
// Implemented Future<void> 
// Available Extensions FutureExtensions 
pub struct TickerFuture {
    // A future that resolves when this future resolves or throws when the ticker is canceled.
    // or_cancel: Box<dyn Future<Output = ()>, 
}

impl TickerFuture {
    // // Creates a Stream containing the result of this future.
    // fn as_stream() -> impl Stream<Output = ()> {
    //     todo!()
    // }
    
    // // Handles errors emitted by this Future.
    // fn catch_error(on_error: impl Fn(), test: impl Fn(Object) -> Option<bool>) -> impl Future<Output = ()> {
    //     todo!()
    // }
    
    // // Register callbacks to be called when this future completes.
    // fn then<R>(on_value: impl Fn(void value) -> FutureOr<R>, onError: Option<Function>) -> Future<R> {
    //     todo!()
    // }
    
    // // Time-out the future computation after timeLimit has passed.
    // fn timeout(time_limit: Duration, on_timeout: Option<impl Fn() -> FutureOr<Ouput = ()>>) -> Future<void> {
    //     todo!()
    // }
    
    // // Registers a function to be called when this future completes.
    // fn when_complete(action: impl Fn()) -> impl Future<Output = ()> {
    //     todo!()
    // }
    
    // Calls callback either when this future resolves or when the ticker is canceled. 
    fn when_complete_or_cancel(callback: VoidCallback) {
        todo!()
    }
}

// TODO: String generic should be fixed
pub trait Route {
    // When this route is popped (see Navigator.pop) if the result isn't specified or if it's null, this value will be used instead.
    // currentResult: T?
    
    // Whether there is at least one active route underneath this route.
    // hasActiveRouteBelow: bool
    
    // Whether this route is on the navigator.
    // isActive: bool

    // Whether this route is the top-most route on the navigator.
    // isCurrent: bool
    
    // Whether this route is the bottom-most active route on the navigator.
    // isFirst: bool
    
    // The navigator that the route is in, if any.
    // navigator: NavigatorState?
    
    // The overlay entries of this route.
    // overlayEntries: List<OverlayEntry>
    
    // A future that completes when this route is popped off the navigator.
    // popped: Future<T?>
    
    // The restoration scope ID to be used for the RestorationScope surrounding this route.
    // restorationScopeId: ValueListenable<String?>
    
    // A representation of the runtime type of the object.
    // runtimeType: Type
    
    // The settings for this route.
    // settings: RouteSettings

    // Whether calling didPop would return false.
    // willHandlePopInternally: bool


    // Called whenever the Navigator has updated in some manner that might affect routes, to indicate that the route may wish to rebuild as well.
    fn changed_external_state(&self);

    // Called whenever the internal state of the route has changed.
    fn changed_internal_state(&self);

    // Called after install when the route is added to the navigator.
    fn did_add(&self);
    
    // This route's next route has changed to the given new route.
    fn did_change_next(&self, next_route: Option<Box<dyn Route>>);

    // This route's previous route has changed to the given new route.
    fn did_change_previous(&self, previous_route: Option<Box<dyn Route>>);

    // // The route was popped or is otherwise being removed somewhat gracefully.
    // fn did_complete<T>(&self, result: Option<T>);

    // // A request was made to pop this route. If the route can handle it internally 
    // // (e.g. because it has its own stack of internal state) then return false, 
    // // otherwise return true (by returning the value of calling super.didPop). 
    // // Returning false will prevent the default behavior of NavigatorState.pop.
    // fn did_pop<T>(&self, result: Option<T>) -> bool;

    // The given route, which was above this one, has been popped off the navigator.
    fn did_pop_next(&self, next_route: Box<dyn Route>);

    // Called after install when the route is pushed onto the navigator.
    fn did_push(&self) -> TickerFuture;
    
    // Called after install when the route replaced another in the navigator.
    fn did_replace(&self, old_route: Option<Box<dyn Route>>);

    // Discards any resources used by the object.
    fn dispose(&self);
    
    // Called when the route is inserted into the navigator.
    fn install(&self);
    
    // Returns whether calling Navigator.maybePop when this Route is current (isCurrent) should do anything.
    fn will_pop(&self) -> Box<dyn Future<Output = RoutePopDisposition>>;
}
