#![allow(unused_imports)]
use std::time::Duration;

use crate::{
    elements::{Element, InkWellElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder, BorderRadius, BoxShape},
    services::MouseCursor,
    ui::{Brightness, Clip, VoidCallback},
    widgets::{FocusNode, NoneWidget, Widget},
};

use super::{ButtonTextTheme, MaterialTapTargetSize, VisualDensity, MaterialStateProperty, InteractiveInkFeatureFactory};

pub struct InkWell {
    // True if this widget will be selected as the initial focus when no other node in its scope is currently focused.
    pub autofocus: bool,
    
    // The clipping radius of the containing rect. This is effective only if customBorder is null.
    pub border_radius: Option<BorderRadius>,
    
    // If true, this widget may request the primary focus.
    pub can_request_focus: bool,
    
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    
    // Whether this ink response should be clipped its bounds.
    pub contained_ink_well: bool,
    
    // The custom clip border which overrides borderRadius.
    pub custom_border: Box<dyn ShapeBorder>,
    
    // Whether detected gestures should provide acoustic and/or haptic feedback.
    pub enable_feedback: bool,
    
    // Whether to exclude the gestures introduced by this widget from the semantics tree.
    pub exclude_from_semantics: bool,
    
    // The color of the ink response when the parent widget is focused. 
    // If this property is null then the focus color of the theme, ThemeData.focusColor, will be used.
    pub focus_color: Option<Color>,
    
    // An optional focus node to use as the focus node for this widget.
    pub focus_node: Option<FocusNode>,
    
    // The highlight color of the ink response when pressed. 
    // If this property is null then the highlight color of the theme, ThemeData.highlightColor, will be used.
    pub highlight_color: Option<Color>,
    
    // The shape (e.g., circle, rectangle) to use for the highlight drawn around this part of the material when pressed, hovered over, or focused.
    pub highlight_shape: Option<BoxShape>,
    
    // The color of the ink response when a pointer is hovering over it. 
    // If this property is null then the hover color of the theme, ThemeData.hoverColor, will be used.
    pub hover_color: Option<Color>,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // The cursor for a mouse pointer when it enters or is hovering over the widget.
    pub mouse_cursor: Option<MouseCursor>,
    
    // Called when the user double taps this part of the material.
    // pub on_double_tap: Option<GestureTapCallback>,
    
    // Handler called when the focus changes.
    // pub on_focus_change: Box<dyn ValueChanged<bool>>,
    
    // Called when this part of the material either becomes highlighted or stops being highlighted.
    // pub on_highlight_changed: Box<dyn ValueChanged<bool>>,
    
    // Called when a pointer enters or exits the ink response area.
    // pub on_hover: Box<dyn ValueChanged<bool>>,
    
    // Called when the user long-presses on this part of the material.
    // pub on_long_press: Option<GestureLongPressCallback>,
    
    // Called when the user taps this part of the material.
    // pub on_tap: Option<GestureTapCallback>,
    
    // Called when the user cancels a tap that was started on this part of the material.
    // pub on_tap_cancel: Option<GestureTapCallback>,
    
    // Called when the user taps down this part of the material.
    // pub on_tap_down: Option<GestureTapDownCallback>,
    
    // Defines the ink response focus, hover, and splash colors.
    pub overlay_color: Option<MaterialStateProperty<Option<Color>>>,
    
    // The radius of the ink splash.
    pub radius: Option<f32>,
    
    // The splash color of the ink response. If this property is null then the splash color of the theme, ThemeData.splashColor, will be used.
    pub splash_color: Color,
    
    // Defines the appearance of the splash.
    pub splash_factory: Option<InteractiveInkFeatureFactory>,
}

impl Default for InkWell {
    fn default() -> Self {
        Self {
            autofocus: Default::default(),
            border_radius: Default::default(),
            can_request_focus: Default::default(),
            child: box NoneWidget,
            contained_ink_well: Default::default(),
            custom_border: box NoneShapeBorder,
            enable_feedback: Default::default(),
            exclude_from_semantics: Default::default(),
            focus_color: Default::default(),
            focus_node: Default::default(),
            highlight_color: Default::default(),
            highlight_shape: Default::default(),
            hover_color: Default::default(),
            key: Default::default(),
            mouse_cursor: Default::default(),
            // on_double_tap: Default::default(),
            // on_focus_change: Default::default(),
            // on_highlight_changed: Default::default(),
            // on_hover: Default::default(),
            // on_long_press: Default::default(),
            // on_tap: Default::default(),
            // on_tap_cancel: Default::default(),
            // on_tap_down: Default::default(),
            overlay_color: Default::default(),
            radius: Default::default(),
            splash_color: Default::default(),
            splash_factory: Default::default(),
        }
    }
}

impl Widget for InkWell {
    fn create_element(&self) -> Box<dyn Element> {
        box InkWellElement::new(self)
    }
}

impl WidgetProperties for InkWell {
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
