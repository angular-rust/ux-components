use crate::foundation::Id;

use super::{InteractState, KeyCode, ModState, MouseButton};

#[derive(Debug, Clone)]
pub struct TextEditChangeEvent {
    pub text: String,
    pub display_text: String,
    pub from_typing: bool,
}

#[derive(Debug, Clone)]
pub struct TextEditCommitEvent {
    pub text: String,
    pub display_text: String,
}

#[derive(Debug, Clone)]
pub struct SliderChangeEvent {
    pub value: f32,
    pub percent: f32,
}

#[derive(Debug, Clone)]
pub struct ScrollHandleVisibleEvent {
    pub vertical: bool,
    pub horizontal: bool,
}

#[derive(Debug, Clone)]
pub struct ProgressChangeEvent {
    pub value: f32,
    pub prev: f32,
}

#[derive(Debug, Clone)]
pub struct ListItemSelectEvent {
    pub index: Option<usize>,
    pub component: Option<Id>,
    pub mouse_event: Option<MouseEvent>,
}

#[derive(Debug, Clone)]
pub struct ListItemEnterEvent {
    pub index: Option<usize>,
    pub component: Option<Id>,
    pub mouse_event: Option<MouseEvent>,
}

#[derive(Debug, Clone)]
pub struct ListItemLeaveEvent {
    pub index: Option<usize>,
    pub component: Option<Id>,
    pub mouse_event: Option<MouseEvent>,
}

#[derive(Debug, Clone)]
pub struct DropdownSelectEvent {
    pub index: Option<usize>,
    pub component: Id,
    pub mouse_event: MouseEvent,
}

#[derive(Debug, Clone)]
pub struct ScaleChangeEvent {
    pub value: f32,
    pub prev: f32,
}

#[derive(Debug, Clone)]
pub struct WidgetRenameEvent {
    pub name: String,
    pub prev: String,
}

#[derive(Debug, Clone)]
pub struct WidgetClipEvent {
    pub clipped: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// The time in seconds when this mouse event occurred, useful for deltas
    pub timestamp: f32,
    /// The state this event is in
    pub state: InteractState,
    /// The button id, if the event `state` is `down` or `up`
    pub button: MouseButton,
    /// The x position in the window of the mouse event
    pub x: i32,
    /// The y position in the window of the mouse event
    pub y: i32,
    /// The relative x position if `state` is `move` or a window has grabbed state
    pub xrel: i32,
    /// The relative y position if `state` is `move` or a window has grabbed state
    pub yrel: i32,
    /// Whether or not the event should bubble further. set to false to stop propagation
    pub bubble: bool,
}

#[derive(Debug, Clone)]
/// A mint specific key event
pub struct KeyEvent {
    /// The time in seconds when this key event occurred, useful for deltas
    pub timestamp: f32,
    /// The state this event is in
    pub state: InteractState,
    /// The raw keycode of the event
    pub keycode: i32,
    /// The typed mint key for the event
    pub key: KeyCode,
    /// The modifier state
    pub mod_: ModState,
    /// Whether or not the event should bubble further. set to false to stop propagation
    pub bubble: bool,
}

#[derive(Debug, Clone)]
/// Information about a text input event
pub struct TextEvent {
    /// The text that this event has generated
    pub text: String,
    /// The type of text event
    pub event_type: TextEventType,
    /// The time in seconds when this touch event occurred, use for deltas
    pub timestamp: f32,
    /// The start position, if the `type` is `edit`
    pub start: i32,
    /// The length position, if the `type` is `edit`
    pub length: i32,
    /// Whether or not the event should bubble further. set to false to stop propagation
    pub bubble: bool,
}

/// A typed text event type
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(i32)]
pub enum TextEventType {
    /// An unknown event
    Unknown,
    /// An edit text typing event
    Edit,
    /// An input text typing event
    Input,
}
