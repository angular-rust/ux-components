use std::time::Duration;

use crate::ui::Offset;

pub struct DragUpdateDetails {
    // The amount the pointer has moved in the coordinate space of
    // the event receiver since the previous update.
    pub delta: Offset,

    // The pointer's global position when it triggered this update.
    pub global_position: Offset,

    // The local position in the coordinate system of the event receiver
    // at which the pointer contacted the screen.
    pub local_position: Offset,

    // The amount the pointer has moved along the primary axis in the coordinate
    // space of the event receiver since the previous update.
    pub primary_delta: Option<f32>,

    // Recorded timestamp of the source pointer event that triggered the drag event.
    pub source_time_stamp: Option<Duration>,
}
