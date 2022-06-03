// TODO: Shadowed with primitives

// #[derive(Debug, Clone, Copy, PartialEq, Hash)]
// #[repr(i32)]
// pub enum TextAlign {
//     /// An unknown alignment
//     Unknown = 0,
//     /// Left aligned (horizontal)
//     Left = 1,
//     /// Right aligned (horizontal)
//     Right = 2,
//     /// Center aligned (horizontal/vertical)
//     Center = 3,
//     /// Top aligned (vertical)
//     Top = 4,
//     /// Bottom aligned (vertical)
//     Bottom = 5,
// }

/// A typed state for mouse, touch, or pressed/similar
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(i32)]
pub enum InteractState {
    /// An unknown state
    Unknown = 0,
    /// An none state
    None = 1,
    /// In a pressed state
    Down = 2,
    /// In a released state
    Up = 3,
    /// In a moving state
    Move = 4,
    /// A mouse wheel state
    Wheel = 5,
    /// A gamepad axis state
    Axis = 6,
}

/// A typed mouse button id
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(i32)]
pub enum MouseButton {
    /// no mouse buttons
    None = -1,
    /// left mouse button
    Left = 0,
    /// middle mouse button
    Middle = 1,
    /// right mouse button
    Right = 2,
    /// extra button pressed
    Extra1 = 3,
    /// extra button pressed
    Extra2 = 4,
}

/// A typed key id
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[repr(i32)]
pub enum KeyCode {
    /// no known key
    Unknown = -1,
    /// left arrow key
    Left = 0,
    /// right arrow key
    Right = 1,
    /// up arrow key
    Up = 2,
    /// down arrow key
    Down = 3,
    /// the backspace key
    Backspace = 4,
    /// the delete key
    Delete = 5,
    /// the tab key
    Tab = 6,
    /// the enter key
    Enter = 7,
    /// the escape key
    Escape = 8,
}

#[derive(Debug, Clone)]
/// Input modifier state
pub struct ModState {
    /// no modifiers are down
    pub none: bool,
    /// left shift key is down
    pub lshift: bool,
    /// right shift key is down
    pub rshift: bool,
    /// left ctrl key is down
    pub lctrl: bool,
    /// right ctrl key is down
    pub rctrl: bool,
    /// left alt/option key is down
    pub lalt: bool,
    /// right alt/option key is down
    pub ralt: bool,
    /// left windows/command key is down
    pub lmeta: bool,
    /// right windows/command key is down
    pub rmeta: bool,
    /// numlock is enabled
    pub num: bool,
    /// capslock is enabled
    pub caps: bool,
    /// mode key is down
    pub mode: bool,
    /// left or right ctrl key is down
    pub ctrl: bool,
    /// left or right shift key is down
    pub shift: bool,
    /// left or right alt/option key is down
    pub alt: bool,
    /// left or right windows/command key is down
    pub meta: bool,
}
