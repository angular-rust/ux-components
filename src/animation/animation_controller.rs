use std::time::Duration;

use super::{AnimationStatus, AnimationBehavior, Animation};

pub struct AnimationController {
    // The behavior of the controller when AccessibilityFeatures.disableAnimations is true.
    pub animation_behavior: AnimationBehavior,
    
    // The length of time this animation should last.
    pub duration: Option<Duration>,
    
    // Whether this animation is currently animating in either the forward or reverse direction.
    pub is_animating: bool,
    
    // Whether this animation is stopped at the end.
    pub is_completed: bool,
    
    // Whether this animation is stopped at the beginning.
    pub is_dismissed: bool,
    
    // The amount of time that has passed between the time the animation started and 
    // the most recent tick of the animation.
    pub last_elapsed_duration: Option<Duration>,
    
    // The value at which this animation is deemed to be dismissed.
    pub lower_bound: f32,
    
    // The length of time this animation should last when going in reverse.
    pub reverse_duration: Option<Duration>,
    
    // The current status of this animation.
    pub status: AnimationStatus,
    
    // The value at which this animation is deemed to be completed.
    pub upper_bound: f32,
    
    // The current value of the animation.
    pub value: f32,
    
    // The rate of change of value per second.
    pub velocity: f32,
    
    // Returns an Animation<f32> for this animation controller, so that a pointer 
    // to this object can be passed around without allowing users of that 
    // pointer to mutate the AnimationController state.
    pub view: Animation<f32>,
}

impl AnimationController {
    // addListener(VoidCallback listener) → void
    // Calls the listener every time the value of the animation changes.
    // inherited
    // addStatusListener(AnimationStatusListener listener) → void
    // Calls listener every time the status of the animation changes.
    // inherited
    // animateBack(double target, {Duration? duration, Curve curve = Curves.linear}) → TickerFuture
    // Drives the animation from its current value to target. 
    // animateTo(double target, {Duration? duration, Curve curve = Curves.linear}) → TickerFuture
    // Drives the animation from its current value to target. 
    // animateWith(Simulation simulation) → TickerFuture
    // Drives the animation according to the given simulation. 
    // clearListeners() → void
    // Removes all listeners added with addListener.
    // @protected, inherited
    // clearStatusListeners() → void
    // Removes all listeners added with addStatusListener.
    // @protected, inherited
    // didRegisterListener() → void
    // This implementation ignores listener registrations.
    // @protected, inherited
    // didUnregisterListener() → void
    // This implementation ignores listener registrations.
    // @protected, inherited
    // dispose() → void
    // Release the resources used by this object. The object is no longer usable after this method is called.
    // override
    // drive<U>(Animatable<U> child) → Animation<U>
    // Chains a Tween (or CurveTween) to this Animation.
    // @optionalTypeArgs, inherited
    // fling({double velocity = 1.0, SpringDescription? springDescription, AnimationBehavior? animationBehavior}) → TickerFuture
    // Drives the animation with a spring (within lowerBound and upperBound) and initial velocity. 
    // forward({double? from}) → TickerFuture
    // Starts running this animation forwards (towards the end). 
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // notifyListeners() → void
    // Calls all the listeners.
    // @protected, inherited
    // notifyStatusListeners(AnimationStatus status) → void
    // Calls all the status listeners.
    // @protected, inherited
    // removeListener(VoidCallback listener) → void
    // Stop calling the listener every time the value of the animation changes.
    // inherited
    // removeStatusListener(AnimationStatusListener listener) → void
    // Stops calling the listener every time the status of the animation changes.
    // inherited
    // repeat({double? min, double? max, bool reverse = false, Duration? period}) → TickerFuture
    // Starts running this animation in the forward direction, and restarts the animation when it completes. 
    // reset() → void
    // Sets the controller's value to lowerBound, stopping the animation (if in progress), and resetting to its beginning point, or dismissed state. 
    // resync(TickerProvider vsync) → void
    // Recreates the Ticker with the new TickerProvider. 
    // reverse({double? from}) → TickerFuture
    // Starts running this animation in reverse (towards the beginning). 
    // stop({bool canceled = true}) → void
    // Stops running this animation. 
    // toString() → String
    // A string representation of this object.
    // inherited
    // toStringDetails() → String
    // Provides a string describing the status of this object, but not including information about the object itself.
    // override
}