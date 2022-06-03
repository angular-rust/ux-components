use crate::{
    elements::{Element, TextFieldElement},
    foundation::{colorspace::Color, Id, Key, ValueChanged, WidgetProperties},
    gestures::DragStartBehavior,
    painting::TextStyle,
    services::{MouseCursor, TextInputType},
    ui::{
        BoxHeightStyle, BoxWidthStyle, Brightness, Radius, TextAlign, TextDirection, VoidCallback,
    },
    widgets::{FocusNode, Widget, TextEditingController},
};

use super::InputDecoration;

pub struct TextField {
    pub key: Key,
    pub controller: TextEditingController,
    pub focus_node: FocusNode,
    pub decoration: InputDecoration,
    pub keyboard_type: TextInputType,
    // pub text_input_action: TextInputAction,
    // pub text_capitalization: TextCapitalization,
    pub style: TextStyle,
    // pub strut_style: StrutStyle,
    pub text_align: TextAlign,
    // pub text_align_vertical: TextAlignVertical,
    pub text_direction: TextDirection,
    pub read_only: bool,
    // pub toolbar_options: ToolbarOptions,
    pub show_cursor: bool,
    pub autofocus: bool,
    pub obscuring_character: String,
    pub obscure_text: bool,
    pub autocorrect: bool,
    // pub smart_dashes_type: SmartDashesType,
    // pub smart_quotes_type: SmartQuotesType,
    pub enable_suggestions: bool,
    pub max_lines: i32,
    pub min_lines: i32,
    pub expands: bool,
    pub max_length: i32,
    // @Deprecated("Use maxLengthEnforcement parameter which provides more specific ' 'behavior related to the maxLength limit. ")
    pub max_length_enforced: bool,
    // pub max_length_enforcement: MaxLengthEnforcement,
    pub on_changed: Option<ValueChanged<String>>,
    pub on_editing_complete: Option<VoidCallback>,
    pub on_submitted: Option<ValueChanged<String>>,
    // pub on_app_private_command: AppPrivateCommandCallback,
    // pub input_formatters: Vec<TextInputFormatter>,
    pub enabled: bool,
    pub cursor_width: f32,
    pub cursor_height: f32,
    pub cursor_radius: Radius,
    pub cursor_color: Color,
    pub selection_height_style: BoxHeightStyle,
    pub selection_width_style: BoxWidthStyle,
    pub keyboard_appearance: Brightness,
    // pub scroll_padding: EdgeInsets,
    pub drag_start_behavior: DragStartBehavior,
    pub enable_interactive_selection: bool,
    // pub selection_controls: TextSelectionControls,
    // pub on_tap: GestureTapCallback,
    pub mouse_cursor: MouseCursor,
    // pub build_counter: InputCounterWidgetBuilder,
    // pub scroll_controller: ScrollController,
    // pub scroll_physics: ScrollPhysics,
    pub autofill_hints: Vec<String>,
    pub restoration_id: String,
    pub enable_ime_personalized_learning: bool,
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            key: Default::default(),
            controller: Default::default(),
            focus_node: Default::default(),
            decoration: Default::default(),
            keyboard_type: Default::default(),
            // text_input_action: Default::default(),
            // text_capitalization: Default::default(),
            style: Default::default(),
            // strut_style: Default::default(),
            text_align: Default::default(),
            // text_align_vertical: Default::default(),
            text_direction: Default::default(),
            read_only: Default::default(),
            // toolbar_options: Default::default(),
            show_cursor: Default::default(),
            autofocus: Default::default(),
            obscuring_character: Default::default(),
            obscure_text: Default::default(),
            autocorrect: Default::default(),
            // smart_dashes_type: Default::default(),
            // smart_quotes_type: Default::default(),
            enable_suggestions: Default::default(),
            max_lines: Default::default(),
            min_lines: Default::default(),
            expands: Default::default(),
            max_length: Default::default(),
            max_length_enforced: Default::default(),
            // max_length_enforcement: Default::default(),
            on_changed: Default::default(),
            on_editing_complete: Default::default(),
            on_submitted: Default::default(),
            // on_app_private_command: Default::default(),
            // input_formatters: Default::default(),
            enabled: Default::default(),
            cursor_width: Default::default(),
            cursor_height: Default::default(),
            cursor_radius: Default::default(),
            cursor_color: Default::default(),
            selection_height_style: Default::default(),
            selection_width_style: Default::default(),
            keyboard_appearance: Default::default(),
            // scroll_padding: Default::default(),
            drag_start_behavior: Default::default(),
            enable_interactive_selection: Default::default(),
            // selection_controls: Default::default(),
            // on_tap: Default::default(),
            mouse_cursor: Default::default(),
            // build_counter: Default::default(),
            // scroll_controller: Default::default(),
            // scroll_physics: Default::default(),
            autofill_hints: Default::default(),
            restoration_id: Default::default(),
            enable_ime_personalized_learning: Default::default(),
        }
    }
}

impl Widget for TextField {
    fn create_element(&self) -> Box<dyn Element> {
        box TextFieldElement::new(self)
    }
}

impl WidgetProperties for TextField {
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
