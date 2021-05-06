#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, Stage, Widget};
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct FloatingWidgetProps {
    pub stage: Option<Stage>,
    pub paint_matrix: Option<dx::Matrix>,
    pub pick_matrix: Option<dx::Matrix>,
    pub pick_handler: u64,
    pub paint_handler: u64,
}

#[derive(Clone, Debug)]
pub struct FloatingWidget {
    props: RefCell<FloatingWidgetProps>,
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

impl Is<Actor> for FloatingWidget {}

impl AsRef<Actor> for FloatingWidget {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

impl fmt::Display for FloatingWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatingWidget")
    }
}
