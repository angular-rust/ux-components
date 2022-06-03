use super::Velocity;

pub struct DragEndDetails {
    // The velocity the pointer was moving along the primary axis when it
    // stopped contacting the screen, in logical pixels per second.
    pub primary_velocity: Option<f32>,

    // The velocity the pointer was moving when it stopped contacting the screen.
    pub velocity: Velocity,
}
