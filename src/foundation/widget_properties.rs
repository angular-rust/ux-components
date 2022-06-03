use super::{Id, Key};

/// Options for constructing a control
pub trait WidgetProperties {
    // /// Generic framework/user specific options,
    // /// which can be strong typed on the receiving end.
    // fn options(&self) -> &HashMap<String, String>;

    // /// Generic framework/user specific data to store on the control `user` property,
    // /// which can be strong typed on the receiving end.
    // fn user(&self) -> &HashMap<String, String>;

    /// The widdget key
    fn key(&self) -> &Key;

    /// The control x position, relative to its container
    fn x(&self) -> f32;
    /// The control y position, relative to its container
    fn y(&self) -> f32;

    /// The control width
    fn w(&self) -> f32;
    /// The control height
    fn h(&self) -> f32;

    /// The control minimum width
    fn w_min(&self) -> f32;
    /// The control minimum height
    fn h_min(&self) -> f32;

    /// The control maximum width
    fn w_max(&self) -> f32;
    /// The control maximum height
    fn h_max(&self) -> f32;

    /// The control parent, if any
    fn parent(&self) -> Option<Id>; // should used to fetch WidgetComponent

    /// The control depth. Usually set internally
    fn depth(&self) -> f32;
    /// Whether or not the control is visible at creation
    fn visible(&self) -> bool;
    /// Whether or not the control responds to mouse input
    fn mouse_input(&self) -> bool;
    /// Whether or not the control responds to key input
    fn key_input(&self) -> bool;
    /// Whether or not the control emits render signals from the canvas render call
    fn renderable(&self) -> bool;
    /// Internal. Internal parent visibility for creating sub controls.
    fn internal_visible(&self) -> bool;
}
