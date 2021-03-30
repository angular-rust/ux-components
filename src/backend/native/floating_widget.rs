use super::Widget;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct FloatingWidget {}

impl FloatingWidget {}

pub const NONE_FLOATING_WIDGET: Option<&FloatingWidget> = None;

impl fmt::Display for FloatingWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FloatingWidget")
    }
}
