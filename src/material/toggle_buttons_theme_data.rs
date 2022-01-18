use crate::{
    foundation::colorspace::Color,
    painting::{BorderRadius, TextStyle},
    rendering::BoxConstraints,
};

/*
copyWith({TextStyle? textStyle, BoxConstraints? constraints, Color? color, Color? selectedColor, Color? disabledColor, Color? fillColor, Color? focusColor, Color? highlightColor, Color? hoverColor, Color? splashColor, Color? borderColor, Color? selectedBorderColor, Color? disabledBorderColor, BorderRadius? borderRadius, double? borderWidth}) -> ToggleButtonsThemeData
Creates a copy of this object but with the given fields replaced with the new values.

debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
Add additional properties associated with the node.
*/

pub struct ToggleButtonsThemeData {
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
    pub border_color: Color,
    pub selected_border_color: Color,
    pub disabled_border_color: Color,
    pub border_radius: BorderRadius,
    pub border_width: f32,
}

impl Default for ToggleButtonsThemeData {
    fn default() -> Self {
        Self {
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
            border_color: Default::default(),
            selected_border_color: Default::default(),
            disabled_border_color: Default::default(),
            border_radius: Default::default(),
            border_width: Default::default(),
        }
    }
}
