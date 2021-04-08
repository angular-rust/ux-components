#![allow(unused_variables)]

use super::Widget;
use crate::prelude::*;
use std::fmt;
use std::{boxed::Box as Box_, cell::RefCell};

#[derive(Clone, Debug)]
pub struct FloatingWidget {
    pub stage: Option<clutter::Stage>,
    pub paint_matrix: Option<cogl::Matrix>,
    pub pick_matrix: Option<cogl::Matrix>,
    pub pick_handler: u64,
    pub paint_handler: u64,
    widget: Widget,
}

impl FloatingWidget {}

impl Object for FloatingWidget {}
impl Is<FloatingWidget> for FloatingWidget {}

impl AsRef<FloatingWidget> for FloatingWidget {
    fn as_ref(&self) -> &FloatingWidget {
        self
    }
}

impl Is<Widget> for FloatingWidget {}

impl AsRef<Widget> for FloatingWidget {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<clutter::Actor> for FloatingWidget {}

impl AsRef<clutter::Actor> for FloatingWidget {
    fn as_ref(&self) -> &clutter::Actor {
        let actor: &clutter::Actor = self.widget.as_ref();
        actor
    }
}

pub const NONE_FLOATING_WIDGET: Option<&FloatingWidget> = None;

impl fmt::Display for FloatingWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatingWidget")
    }
}
