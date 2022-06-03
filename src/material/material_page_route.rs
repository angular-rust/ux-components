#![allow(unused_imports)]
use std::{future::Future, time::Duration};

use crate::{
    prelude::Color,
    ui::{ImageFilter, VoidCallback},
    widgets::{
        BuildContext, NavigatorState, NoneWidget, Route, RoutePopDisposition, TickerFuture, Widget,
        WidgetBuilder,
    },
};

// Route<T> -> OverlayRoute<T> -> TransitionRoute<T> -> ModalRoute<T> -> PageRoute<T>
pub struct MaterialPageRoute {
    // The animation that drives the route's transition and the previous route's forward transition.
    // pub animation: Option<Animation<f32>>,

    // The color to use for the modal barrier. If this is null, the barrier will be transparent.
    pub barrier_color: Option<Color>,

    // The curve that is used for animating the modal barrier in and out.
    // pub barrier_curve: Curve,

    // Whether you can dismiss this route by tapping the modal barrier.
    pub barrier_dismissible: bool,

    // The semantic label used for a dismissible barrier.
    pub barrier_label: Option<String>,

    // Builds the primary contents of the route.
    pub builder: WidgetBuilder,

    // Whether this route can be popped.
    pub can_pop: bool,

    // This future completes only once the transition itself has finished,
    // after the overlay entries have been removed from the navigator's overlay.
    // pub completed: Future<Option<T>>,

    // The animation controller that the route uses to drive the transitions.
    // pub controller: Option<AnimationController>,

    // When this route is popped (see Navigator.pop) if the result isn't specified or if it's null, this value will be used instead.
    // pub current_result: Option<T>,

    // A short description of this route useful for debugging.
    pub debug_label: String,

    // The filter to add to the barrier.
    pub filter: Option<ImageFilter>,

    // Controls whether didPop calls NavigatorState.finalizeRoute.
    pub finished_when_popped: bool,

    // Whether this page route is a full-screen dialog.
    pub fullscreen_dialog: bool,

    // Whether there is at least one active route underneath this route.
    pub has_active_route_below: bool,

    // True if one or more WillPopCallback callbacks exist.
    pub has_scoped_will_pop_callback: bool,

    // Whether this route is on the navigator.
    pub is_active: bool,

    // Whether this route is the top-most route on the navigator.
    pub is_current: bool,

    // Whether this route is the bottom-most active route on the navigator.
    pub is_first: bool,

    // Whether the route should remain in memory when it is inactive.
    pub maintain_state: bool,

    // The navigator that the route is in, if any.
    pub navigator: Option<NavigatorState>,

    // Whether this route is currently offstage.
    pub offstage: bool,

    // Whether the route obscures previous routes when the transition is complete.
    pub opaque: bool,

    // The overlay entries of this route.
    // pub overlay_entries: Vec<OverlayEntry>,

    // A future that completes when this route is popped off the navigator.
    // pub popped: Future<Option<T>>,

    // The restoration scope ID to be used for the RestorationScope surrounding this route.
    // pub restoration_scope_id: ValueListenable<Option<String>>,

    // The duration the transition going in reverse.
    pub reverse_transition_duration: Duration,

    // The animation for the route being pushed on top of this route.
    // This animation lets this route coordinate with the entrance and exit
    // transition of route pushed on top of this route.
    // pub secondary_animation: Option<Animation<f32>>,

    // Whether the semantics of the modal barrier are included in the semantics tree.
    pub semantics_dismissible: bool,

    // // The settings for this route.
    // pub settings: RouteSettings,

    // The build context for the subtree containing the primary content of this route.
    pub subtree_context: Option<BuildContext>,

    // The duration the transition going forwards.
    pub transition_duration: Duration,

    // Whether to takeover the controller created by createAnimationController.
    pub will_dispose_animation_controller: bool,

    // Whether calling didPop would return false.
    pub will_handle_pop_internally: bool,
}

impl MaterialPageRoute {
    // Adds a local history entry to this route.
    // pub fn add_local_history_entry(&self, entry: LocalHistoryEntry) {
    //     todo!()
    // }

    // Enables this route to veto attempts by the user to dismiss it.
    // pub fn add_scoped_will_pop_callback(&self, callback: WillPopCallback) {
    //     todo!()
    // }

    // Builds the primary contents of the route.
    pub fn build_content(&self, context: BuildContext) -> Box<dyn Widget> {
        todo!()
    }

    // Override this method to build the primary content of this route.
    // pub fn build_page(
    //     &self,
    //     context: BuildContext,
    //     animation: Animation<f32>,
    //     secondaryAnimation: Animation<f32>,
    // ) -> Box<dyn Widget> {
    //     todo!()
    // }

    // Override this method to wrap the child with one or more transition widgets that
    // define how the route arrives on and leaves the screen.
    // pub fn build_transitions(
    //     &self,
    //     context: BuildContext,
    //     animation: Animation<f32>,
    //     secondaryAnimation: Animation<f32>,
    //     child: Box<dyn Widget>,
    // ) -> Box<dyn Widget> {
    //     todo!()
    // }

    // Returns true if previousRoute should animate when this route is pushed on top of it
    // or when then this route is popped off of it.
    // pub fn can_transition_from(&self, previousRoute: TransitionRoute) -> bool {
    //     todo!()
    // }

    // Returns true if this route supports a transition animation that runs when nextRoute
    // is pushed on top of it or when nextRoute is popped off of it.
    // pub fn can_transition_to(&self, nextRoute: TransitionRoute) -> bool {
    //     todo!()
    // }

    // Called whenever the Navigator has updated in some manner that might affect routes,
    // to indicate that the route may wish to rebuild as well.
    pub fn changed_external_state(&self) {
        todo!()
    }

    // Called whenever the internal state of the route has changed.
    pub fn changed_internal_state(&self) {
        todo!()
    }

    // Called to create the animation that exposes the current progress of the transition
    // controlled by the animation controller created by createAnimationController().
    // pub fn create_animation(&self) -> Animation<f32> {
    //     todo!()
    // }

    // Called to create the animation controller that will drive the transitions to this
    // route from the previous one, and back to the previous route from this one.
    // pub fn create_animation_controller(&self) -> AnimationController {
    //     todo!()
    // }

    // Subclasses should override this getter to return the builders for the overlay.
    // pub fn create_overlay_entries(&self) -> Iterable<OverlayEntry> {
    //     todo!()
    // }

    // Called after install when the route is added to the navigator.
    pub fn did_add(&self) {
        todo!()
    }

    // This route's next route has changed to the given new route.
    pub fn did_change_next(&self, next_route: Option<Box<dyn Route>>) {
        todo!()
    }

    // This route's previous route has changed to the given new route.
    pub fn did_change_previous(&self, previous_route: Option<Box<dyn Route>>) {
        todo!()
    }

    // The route was popped or is otherwise being removed somewhat gracefully.
    // pub fn did_complete(&self, result: Option<T>) {
    //     todo!()
    // }

    // A request was made to pop this route. If the route can handle it internally
    // (e.g. because it has its own stack of internal state) then return false,
    // otherwise return true (by returning the value of calling super.didPop).
    // Returning false will prevent the default behavior of NavigatorState.pop.
    // pub fn did_pop(&self, result: Option<T>) -> bool {
    //     todo!()
    // }

    // The given route, which was above this one, has been popped off the navigator.
    pub fn did_pop_next(&self, next_route: Box<dyn Route>) {
        todo!()
    }

    // Called after install when the route is pushed onto the navigator.
    pub fn did_push(&self) -> TickerFuture {
        todo!()
    }

    // Called after install when the route replaced another in the navigator.
    pub fn did_replace(&self, old_route: Option<Box<dyn Route>>) {
        todo!()
    }

    // Discards any resources used by the object.
    pub fn dispose(&self) {
        todo!()
    }

    // Called when the route is inserted into the navigator.
    pub fn install(&self) {
        todo!()
    }

    // Remove a local history entry from this route.
    // pub fn remove_local_history_entry(entry: LocalHistoryEntry) {
    //     todo!()
    // }

    // Remove one of the callbacks run by willPop.
    // pub fn remove_scoped_will_pop_callback(&self, callback: WillPopCallback) {
    //     todo!()
    // }

    // Schedule a call to buildTransitions.
    pub fn set_state(&self, func: VoidCallback) {
        todo!()
    }

    // Returns RoutePopDisposition.doNotPop if any of callbacks added with addScopedWillPopCallback
    // returns either false or null. If they all return true, the base Route.willPop's result will be returned.
    // The callbacks will be called in the order they were added, and will only be called
    // if all previous callbacks returned true.
    // pub fn will_pop(&self) -> Future<RoutePopDisposition> {
    //     todo!()
    // }
}

impl Default for MaterialPageRoute {
    fn default() -> Self {
        Self {
            // animation: Default::default(),
            barrier_color: Default::default(),
            // barrier_curve: Default::default(),
            barrier_dismissible: Default::default(),
            barrier_label: Default::default(),
            builder: box |_| box NoneWidget,
            can_pop: Default::default(),
            // completed: Default::default(),
            // controller: Default::default(),
            // current_result: Default::default(),
            debug_label: Default::default(),
            filter: Default::default(),
            finished_when_popped: Default::default(),
            fullscreen_dialog: Default::default(),
            has_active_route_below: Default::default(),
            has_scoped_will_pop_callback: Default::default(),
            is_active: Default::default(),
            is_current: Default::default(),
            is_first: Default::default(),
            maintain_state: Default::default(),
            navigator: Default::default(),
            offstage: Default::default(),
            opaque: Default::default(),
            // overlay_entries: Default::default(),
            // popped: Default::default(),
            // restoration_scope_id: Default::default(),
            reverse_transition_duration: Default::default(),
            // secondary_animation: Default::default(),
            semantics_dismissible: Default::default(),
            // settings: Default::default(),
            subtree_context: Default::default(),
            transition_duration: Default::default(),
            will_dispose_animation_controller: Default::default(),
            will_handle_pop_internally: Default::default(),
        }
    }
}
