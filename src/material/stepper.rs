use crate::{
    elements::{Element, StepperElement},
    foundation::{Id, Key, ValueChanged, WidgetProperties},
    ui::VoidCallback,
    widgets::Widget,
};

use super::Step;

pub struct Stepper {
    pub key: Key,
    pub steps: Vec<Step>,
    // pub physics: ScrollPhysics,
    // pub stepper_type: StepperType,
    pub current_step: i32,
    pub on_step_tapped: Option<ValueChanged<i32>>,
    pub on_step_continue: Option<VoidCallback>,
    pub on_step_cancel: Option<VoidCallback>,
    // pub controls_builder: ControlsWidgetBuilder,
    pub elevation: f32,
}

impl Default for Stepper {
    fn default() -> Self {
        Self {
            key: Default::default(),
            steps: Default::default(),
            // physics: Default::default(),
            // stepper_type: Default::default(),
            current_step: Default::default(),
            on_step_tapped: Default::default(),
            on_step_continue: Default::default(),
            on_step_cancel: Default::default(),
            // controls_builder: Default::default(),
            elevation: Default::default(),
        }
    }
}

impl Widget for Stepper {
    fn create_element(&self) -> Box<dyn Element> {
        box StepperElement::new(self)
    }
}

impl WidgetProperties for Stepper {
    fn key(&self) -> &Key {
        &self.key
    }

    fn x(&self) -> f32 {
        // self.x
        0.0
    }

    fn y(&self) -> f32 {
        // self.y
        0.0
    }

    fn w(&self) -> f32 {
        // self.w
        0.0
    }

    fn h(&self) -> f32 {
        // self.h
        0.0
    }

    fn w_min(&self) -> f32 {
        // self.w_min
        0.0
    }

    fn h_min(&self) -> f32 {
        // self.h_min
        0.0
    }

    fn w_max(&self) -> f32 {
        // self.w_max
        0.0
    }

    fn h_max(&self) -> f32 {
        // self.h_max
        0.0
    }

    fn parent(&self) -> Option<Id> {
        // self.parent
        None
    }

    fn depth(&self) -> f32 {
        // self.depth
        0.0
    }

    fn visible(&self) -> bool {
        // self.visible
        true
    }

    fn mouse_input(&self) -> bool {
        // self.mouse_input
        true
    }

    fn key_input(&self) -> bool {
        // self.key_input
        true
    }

    fn renderable(&self) -> bool {
        // self.renderable
        true
    }

    fn internal_visible(&self) -> bool {
        // self.internal_visible
        true
    }
}
