use std::time::Duration;

use cgmath::Matrix4;

use crate::ui::{Offset, PointerDeviceKind};

pub struct PointerExitEvent {
    // Bit field using the *Button constants such as kPrimaryMouseButton, kSecondaryStylusButton, etc.
    pub buttons: usize,
    
    // Distance in logical pixels that the pointer moved since the last PointerMoveEvent or PointerHoverEvent.
    pub delta: Offset,
    
    // Unique identifier for the pointing device, reused across interactions.
    pub device: usize,
    
    // The distance of the detected object from the input surface.
    pub distance: f32,
    
    // The maximum value that distance can return for this pointer.
    pub distance_max: f32,
    
    // The minimum value that distance can return for this pointer.
    pub distance_min: f32,
    
    // Set if the pointer is currently down.
    pub down: bool,
    
    // Unique identifier that ties the PointerEvent to the embedder event that created it.
    pub embedder_id: usize,
    
    // The kind of input device for which the event was generated.
    pub kind: PointerDeviceKind,
    
    // The delta transformed into the event receiver's local coordinate system according to transform.
    pub local_delta: Offset,
    
    // The position transformed into the event receiver's local coordinate system according to transform.
    pub local_position: Offset,
    
    // Set if an application from a different security domain is in any way obscuring this application's window.
    pub obscured: bool,
    
    // The orientation angle of the detected object, in radians.
    pub orientation: f32,
    
    // // The original un-transformed PointerEvent before any transforms were applied.
    // pub original: Option<PointerEvent>,
    
    // Opaque platform-specific data associated with the event.
    pub platform_data: usize,
    
    // Unique identifier for the pointer, not reused. Changes for each new pointer down event.
    pub pointer: usize,
    
    // Coordinate of the position of the pointer, in logical pixels in the global coordinate space.
    pub position: Offset,
    
    // The pressure of the touch.
    pub pressure: f32,
    
    // The maximum value that pressure can return for this pointer.
    pub pressure_max: f32,
    
    // The minimum value that pressure can return for this pointer.
    pub pressure_min: f32,
    
    // The radius of the contact ellipse along the major axis, in logical pixels.
    pub radius_major: f32,
    
    // The maximum value that could be reported for radiusMajor and radiusMinor for this pointer, in logical pixels.
    pub radius_max: f32,
    
    // The minimum value that could be reported for radiusMajor and radiusMinor for this pointer, in logical pixels.
    pub radius_min: f32,
    
    // The radius of the contact ellipse along the minor axis, in logical pixels.
    pub radius_minor: f32,
    
    // The area of the screen being pressed.
    pub size: f32,
    
    // Set if the event was synthesized by Flutter.
    pub synthesized: bool,
    
    // The tilt angle of the detected object, in radians.
    pub tilt: f32,
    
    // Time of event dispatch, relative to an arbitrary timeline.
    pub time_stamp: Duration,
    
    // The transformation used to transform this event from the global coordinate space into the coordinate space of the event receiver.
    pub transform: Option<Matrix4<f32>>,
}

impl PointerExitEvent {
    // Transforms the event from the global coordinate space into the coordinate space of an event receiver.
    pub fn transformed(&self, transform: Matrix4<f32>) -> Self {
        todo!()
    }
}