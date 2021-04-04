#![allow(unused_variables)]

use super::Actor;
use crate::prelude::*;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct FloatingWidget {}

impl FloatingWidget {}

impl Object for FloatingWidget {}
impl Is<FloatingWidget> for FloatingWidget {}

impl AsRef<FloatingWidget> for FloatingWidget {
    fn as_ref(&self) -> &FloatingWidget {
        self
    }
}

pub const NONE_FLOATING_WIDGET: Option<&FloatingWidget> = None;

impl fmt::Display for FloatingWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatingWidget")
    }
}
