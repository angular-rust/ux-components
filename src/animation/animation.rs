use super::AnimationStatus;

pub struct Animation<T> {
    // Whether this animation is stopped at the end.
    pub is_completed: bool,
    
    // Whether this animation is stopped at the beginning.
    pub is_dismissed: bool,
    
    // The current status of this animation.
    pub status: AnimationStatus,
    
    // The current value of the animation.
    pub value: T,
}

impl<T> Animation<T> {
    // Calls the listener every time the value of the animation changes.
    // addListener(VoidCallback listener) → void
    
    // Calls listener every time the status of the animation changes. 
    // addStatusListener(AnimationStatusListener listener) → void
    
    // Chains a Tween (or CurveTween) to this Animation.
    // drive<U>(Animatable<U> child) → Animation<U>
    
    // Stop calling the listener every time the value of the animation changes.
    // removeListener(VoidCallback listener) → void
    
    // Stops calling the listener every time the status of the animation changes. 
    // removeStatusListener(AnimationStatusListener listener) → void    
}