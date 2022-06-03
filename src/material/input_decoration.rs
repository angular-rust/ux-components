use crate::painting::NoneEdgeInsetsGeometry;
use crate::prelude::Color;

use crate::ui::TextDirection;
use crate::widgets::NoneWidget;
use crate::{
    painting::{EdgeInsetsGeometry, TextStyle},
    rendering::BoxConstraints,
    widgets::Widget,
};

use super::{FloatingLabelAlignment, FloatingLabelBehavior, InputBorder};

pub struct InputDecoration {
    // Typically set to true when the InputDecorator contains a multiline TextField
    // (TextField.maxLines is null or > 1) to override the default behavior of aligning
    // the label with the center of the TextField.
    pub align_label_with_hint: Option<bool>,

    // The shape of the border to draw around the decoration's container.
    pub border: InputBorder,

    // Defines minimum and maximum sizes for the InputDecorator.
    pub constraints: Option<BoxConstraints>,

    // The padding for the input decoration's container.
    pub content_padding: Box<dyn EdgeInsetsGeometry>,

    // Optional custom counter widget to go in the place otherwise occupied by counterText.
    // If this property is non null, then counterText is ignored.
    pub counter: Box<dyn Widget>,

    // The style to use for the counterText.
    pub counter_style: Option<TextStyle>,

    // Optional text to place below the line as a character count.
    pub counter_text: Option<String>,

    // The border to display when the InputDecorator is disabled and is not showing an error.
    pub disabled_border: Option<InputBorder>,

    // If false helperText,errorText, and counterText are not displayed,
    // and the opacity of the remaining visual elements is reduced.
    pub enabled: bool,

    // The border to display when the InputDecorator is enabled and is not showing an error.
    pub enabled_border: Option<InputBorder>,

    // The border to display when the InputDecorator does not have the focus and is showing an error.
    pub error_border: Option<InputBorder>,

    // The maximum number of lines the errorText can occupy.
    pub error_max_lines: Option<i32>,

    // The style to use for the InputDecoration.errorText.
    pub error_style: Option<TextStyle>,

    // Text that appears below the InputDecorator.child and the border.
    pub error_text: Option<String>,

    // The base fill color of the decoration's container color.
    pub fill_color: Option<Color>,

    // If true the decoration's container is filled with fillColor.
    pub filled: Option<bool>,

    // Defines where the floating label should be displayed.
    pub floating_label_alignment: Option<FloatingLabelAlignment>,

    // Defines how the floating label should behave.
    pub floating_label_behavior: Option<FloatingLabelBehavior>,

    // The style to use for InputDecoration.labelText when the label is above
    // (i.e., vertically adjacent to) the input field.
    pub floating_label_style: Option<TextStyle>,

    // By default the focusColor is based on the current Theme.
    pub focus_color: Option<Color>,

    // The border to display when the InputDecorator has the focus and is not showing an error.
    pub focused_border: Option<InputBorder>,

    // The border to display when the InputDecorator has the focus and is showing an error.
    pub focused_error_border: Option<InputBorder>,

    // The maximum number of lines the helperText can occupy.
    pub helper_max_lines: Option<i32>,

    // The style to use for the helperText.
    pub helper_style: Option<TextStyle>,

    // Text that provides context about the InputDecorator.child's value, such as how the value will be used.
    pub helper_text: Option<String>,

    // The maximum number of lines the hintText can occupy.
    pub hint_max_lines: Option<i32>,

    // The style to use for the hintText.
    pub hint_style: TextStyle,

    // Text that suggests what sort of input the field accepts.
    pub hint_text: Option<String>,

    // The direction to use for the hintText.
    pub hint_text_direction: Option<TextDirection>,

    // The color of the focus highlight for the decoration shown if the
    // container is being hovered over by a mouse.
    pub hover_color: Option<Color>,

    // An icon to show before the input field and outside of the decoration's container.
    pub icon: Box<dyn Widget>,

    // The color of the icon.
    pub icon_color: Option<Color>,

    // Whether the decoration is the same size as the input field.
    pub is_collapsed: bool,

    // Whether the InputDecorator.child is part of a dense form
    // (i.e., uses less vertical space).
    pub is_dense: Option<bool>,

    // Optional widget that describes the input field.
    pub label: Box<dyn Widget>,

    // The style to use for InputDecoration.labelText when the label
    // is on top of the input field.
    pub label_style: TextStyle,

    // Optional text that describes the input field.
    pub label_text: Option<String>,

    // Optional widget to place on the line before the input.
    pub prefix: Box<dyn Widget>,

    // An icon that appears before the prefix or prefixText and before the
    // editable part of the text field, within the decoration's container.
    pub prefix_icon: Box<dyn Widget>,

    // Optional color of the prefixIcon
    pub prefix_icon_color: Option<Color>,

    // The constraints for the prefix icon.
    pub prefix_icon_constraints: Option<BoxConstraints>,

    // The style to use for the prefixText.
    pub prefix_style: Option<TextStyle>,

    // Optional text prefix to place on the line before the input.
    pub prefix_text: Option<String>,

    // A semantic label for the counterText.
    pub semantic_counter_text: Option<String>,

    // Optional widget to place on the line after the input.
    pub suffix: Box<dyn Widget>,

    // An icon that appears after the editable part of the text field and after
    // the suffix or suffixText, within the decoration's container.
    pub suffix_icon: Box<dyn Widget>,

    // Optional color of the suffixIcon
    pub suffix_icon_color: Option<Color>,

    // The constraints for the suffix icon.
    pub suffix_icon_constraints: Option<BoxConstraints>,

    // The style to use for the suffixText.
    pub suffix_style: Option<TextStyle>,

    // Optional text suffix to place on the line after the input.
    pub suffix_text: Option<String>,
}

// applyDefaults(InputDecorationTheme theme) → InputDecoration
// Used by widgets like TextField and InputDecorator to create a new InputDecoration with default values taken from the theme.
// copyWith({Widget? icon, Color? iconColor, Widget? label, String? labelText, TextStyle? labelStyle, TextStyle? floatingLabelStyle, String? helperText, TextStyle? helperStyle, int? helperMaxLines, String? hintText, TextStyle? hintStyle, TextDirection? hintTextDirection, int? hintMaxLines, String? errorText, TextStyle? errorStyle, int? errorMaxLines, FloatingLabelBehavior? floatingLabelBehavior, FloatingLabelAlignment? floatingLabelAlignment, bool? isCollapsed, bool? isDense, EdgeInsetsGeometry? contentPadding, Widget? prefixIcon, Widget? prefix, String? prefixText, BoxConstraints? prefixIconConstraints, TextStyle? prefixStyle, Color? prefixIconColor, Widget? suffixIcon, Widget? suffix, String? suffixText, TextStyle? suffixStyle, Color? suffixIconColor, BoxConstraints? suffixIconConstraints, Widget? counter, String? counterText, TextStyle? counterStyle, bool? filled, Color? fillColor, Color? focusColor, Color? hoverColor, InputBorder? errorBorder, InputBorder? focusedBorder, InputBorder? focusedErrorBorder, InputBorder? disabledBorder, InputBorder? enabledBorder, InputBorder? border, bool? enabled, String? semanticCounterText, bool? alignLabelWithHint, BoxConstraints? constraints}) → InputDecoration
// Creates a copy of this input decoration with the given fields replaced by the new values.

impl Default for InputDecoration {
    fn default() -> Self {
        Self {
            align_label_with_hint: Default::default(),
            border: Default::default(),
            constraints: Default::default(),
            content_padding: box NoneEdgeInsetsGeometry,
            counter: box NoneWidget,
            counter_style: Default::default(),
            counter_text: Default::default(),
            disabled_border: Default::default(),
            enabled: Default::default(),
            enabled_border: Default::default(),
            error_border: Default::default(),
            error_max_lines: Default::default(),
            error_style: Default::default(),
            error_text: Default::default(),
            fill_color: Default::default(),
            filled: Default::default(),
            floating_label_alignment: Default::default(),
            floating_label_behavior: Default::default(),
            floating_label_style: Default::default(),
            focus_color: Default::default(),
            focused_border: Default::default(),
            focused_error_border: Default::default(),
            helper_max_lines: Default::default(),
            helper_style: Default::default(),
            helper_text: Default::default(),
            hint_max_lines: Default::default(),
            hint_style: Default::default(),
            hint_text: Default::default(),
            hint_text_direction: Default::default(),
            hover_color: Default::default(),
            icon: box NoneWidget,
            icon_color: Default::default(),
            is_collapsed: Default::default(),
            is_dense: Default::default(),
            label: box NoneWidget,
            label_style: Default::default(),
            label_text: Default::default(),
            prefix: box NoneWidget,
            prefix_icon: box NoneWidget,
            prefix_icon_color: Default::default(),
            prefix_icon_constraints: Default::default(),
            prefix_style: Default::default(),
            prefix_text: Default::default(),
            semantic_counter_text: Default::default(),
            suffix: box NoneWidget,
            suffix_icon: box NoneWidget,
            suffix_icon_color: Default::default(),
            suffix_icon_constraints: Default::default(),
            suffix_style: Default::default(),
            suffix_text: Default::default(),
        }
    }
}
