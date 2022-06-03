use crate::{
    elements::{Element, ToggleButtonsElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::{BorderRadius, TextStyle, VerticalDirection},
    rendering::BoxConstraints,
    services::MouseCursor,
    widgets::{FocusNode, Widget},
};

pub struct ToggleButtons {
    pub key: Key,
    pub children: Vec<Box<dyn Widget>>,
    pub is_selected: Vec<bool>,
    // pub on_pressed: Fn(i32), // Fn(index: i32)
    pub mouse_cursor: MouseCursor,
    pub text_style: TextStyle,
    pub constraints: BoxConstraints,
    pub color: Color,
    pub selected_color: Color,
    pub disabled_color: Color,
    pub fill_color: Color,
    pub focus_color: Color,
    pub highlight_color: Color,
    pub hover_color: Color,
    pub splash_color: Color,
    pub focus_nodes: Vec<FocusNode>,
    pub render_border: bool,
    pub border_color: Color,
    pub selected_border_color: Color,
    pub disabled_border_color: Color,
    pub border_radius: BorderRadius,
    pub border_width: f32,
    // pub direction: Axis,
    pub vertical_direction: VerticalDirection,
}

impl Default for ToggleButtons {
    fn default() -> Self {
        Self {
            key: Default::default(),
            children: Default::default(),
            is_selected: Default::default(),
            // on_pressed: Default::default(),
            mouse_cursor: Default::default(),
            text_style: Default::default(),
            constraints: Default::default(),
            color: Default::default(),
            selected_color: Default::default(),
            disabled_color: Default::default(),
            fill_color: Default::default(),
            focus_color: Default::default(),
            highlight_color: Default::default(),
            hover_color: Default::default(),
            splash_color: Default::default(),
            focus_nodes: Default::default(),
            render_border: Default::default(),
            border_color: Default::default(),
            selected_border_color: Default::default(),
            disabled_border_color: Default::default(),
            border_radius: Default::default(),
            border_width: Default::default(),
            // direction: Default::default(),
            vertical_direction: Default::default(),
        }
    }
}

impl Widget for ToggleButtons {
    fn create_element(&self) -> Box<dyn Element> {
        box ToggleButtonsElement::new(self)
    }
}

impl WidgetProperties for ToggleButtons {
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
