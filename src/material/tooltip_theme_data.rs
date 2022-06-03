use std::time::Duration;

use crate::painting::{
    Decoration, EdgeInsetsGeometry, NoneDecoration, NoneEdgeInsetsGeometry, TextStyle,
};

use super::TooltipTriggerMode;

pub struct TooltipThemeData {
    pub height: f32,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub margin: Box<dyn EdgeInsetsGeometry>,
    pub vertical_offset: f32,
    pub prefer_below: bool,
    pub exclude_from_semantics: bool,
    pub decoration: Box<dyn Decoration>,
    pub text_style: TextStyle,
    pub wait_duration: Duration,
    pub show_duration: Duration,
    pub trigger_mode: TooltipTriggerMode,
    pub enable_feedback: bool,
}

impl Default for TooltipThemeData {
    fn default() -> Self {
        Self {
            height: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            margin: box NoneEdgeInsetsGeometry,
            vertical_offset: Default::default(),
            prefer_below: Default::default(),
            exclude_from_semantics: Default::default(),
            decoration: box NoneDecoration,
            text_style: Default::default(),
            wait_duration: Default::default(),
            show_duration: Default::default(),
            trigger_mode: Default::default(),
            enable_feedback: Default::default(),
        }
    }
}
