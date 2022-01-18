use crate::foundation::{Id, Key, WidgetProperties};

/// Options for constructing a Slider
#[derive(Debug, Clone)]
pub struct SliderProperties {
    /// Whether or not the slider is vertical. default: false
    pub vertical: bool,

    /// Inverts the sliders default direction. If true, horizontal slider goes from right to left and vertical slider from top to bottom. default: false
    pub invert: bool,

    /// The slider minimum value. default: 0
    pub min: f32,
    /// The slider maximum value. default: 1
    pub max: f32,
    /// The slider initial value. default: 1
    pub value: f32,
    /// The slider step value. default: none
    pub step: Option<f32>,

    // Widget options
    /// The key to identify widget
    pub key: Key,

    /// The control x position, relative to its container
    pub x: f32,
    /// The control y position, relative to its container
    pub y: f32,

    /// The control width
    pub w: f32,
    /// The control height
    pub h: f32,

    /// The control minimum width
    pub w_min: f32,
    /// The control minimum height
    pub h_min: f32,

    /// The control maximum width
    pub w_max: f32,
    /// The control maximum height
    pub h_max: f32,

    /// The control parent, if any
    pub parent: Option<Id>, // should used to fetch WidgetComponent

    /// The control depth. Usually set internally
    pub depth: f32,
    /// Whether or not the control is visible at creation
    pub visible: bool,
    /// Whether or not the control responds to mouse input
    pub mouse_input: bool,
    /// Whether or not the control responds to key input
    pub key_input: bool,
    /// Whether or not the control emits render signals from the canvas render call
    pub renderable: bool,

    /// Internal. Internal parent visibility for creating sub controls.
    pub internal_visible: bool,
}

impl WidgetProperties for SliderProperties {
    fn key(&self) -> &Key {
        &self.key
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn w(&self) -> f32 {
        self.w
    }

    fn h(&self) -> f32 {
        self.h
    }

    fn w_min(&self) -> f32 {
        self.w_min
    }

    fn h_min(&self) -> f32 {
        self.h_min
    }

    fn w_max(&self) -> f32 {
        self.w_max
    }

    fn h_max(&self) -> f32 {
        self.h_max
    }

    fn parent(&self) -> Option<Id> {
        self.parent
    }

    fn depth(&self) -> f32 {
        self.depth
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn mouse_input(&self) -> bool {
        self.mouse_input
    }

    fn key_input(&self) -> bool {
        self.key_input
    }

    fn renderable(&self) -> bool {
        self.renderable
    }

    fn internal_visible(&self) -> bool {
        self.internal_visible
    }
}

impl Default for SliderProperties {
    fn default() -> Self {
        Self {
            key: Default::default(),
            vertical: false,
            invert: false,
            min: 0.0,
            max: 1.0,
            value: 1.0,
            step: Default::default(),
            x: Default::default(),
            y: Default::default(),
            w: 32.0,
            h: 32.0,
            w_min: Default::default(),
            h_min: Default::default(),
            w_max: Default::default(),
            h_max: Default::default(),
            parent: Default::default(),
            depth: Default::default(),
            visible: true,
            mouse_input: true,
            key_input: Default::default(),
            renderable: true,
            internal_visible: Default::default(),
        }
    }
}
