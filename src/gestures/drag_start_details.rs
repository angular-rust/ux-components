use std::time::Duration;

use crate::ui::{Offset, PointerDeviceKind};

pub struct DragStartDetails {
    // The global position at which the pointer contacted the screen.
    pub global_position: Offset,

    // The kind of the device that initiated the event.
    pub kind: Option<PointerDeviceKind>,

    // The local position in the coordinate system of the event receiver at which the pointer contacted the screen.
    pub local_position: Offset,

    // Recorded timestamp of the source pointer event that triggered the drag event.
    pub source_time_stamp: Option<Duration>,
}
