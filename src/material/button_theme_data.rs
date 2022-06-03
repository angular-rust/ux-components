use crate::{
    foundation::colorspace::Color,
    painting::{EdgeInsetsGeometry, NoneEdgeInsetsGeometry, NoneShapeBorder, ShapeBorder},
};

use super::{ButtonBarLayoutBehavior, ButtonTextTheme, ColorScheme, MaterialTapTargetSize};

/*
copyWith({ButtonTextTheme? textTheme, ButtonBarLayoutBehavior? layoutBehavior, double? minWidth, double? height, EdgeInsetsGeometry? padding, ShapeBorder? shape, bool? alignedDropdown, Color? buttonColor, Color? disabledColor, Color? focusColor, Color? hoverColor, Color? highlightColor, Color? splashColor, ColorScheme? colorScheme, MaterialTapTargetSize? materialTapTargetSize}) -> ButtonThemeData
Creates a copy of this button theme data object with the matching fields replaced with the non-null parameter values.

debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
Add additional properties associated with the node.
override

getAnimationDuration(MaterialButton button) -> Duration
The duration of the button's highlight animation.

getBrightness(MaterialButton button) -> Brightness
The button's overall brightness.

getConstraints(MaterialButton button) -> BoxConstraints
The BoxConstraints that the define the button's size.

getDisabledElevation(MaterialButton button) -> double
The button's elevation when MaterialButton.onPressed is null (when MaterialButton.enabled is false).

getDisabledFillColor(MaterialButton button) -> Color
The button's background color when MaterialButton.onPressed is null (when MaterialButton.enabled is false).

getDisabledTextColor(MaterialButton button) -> Color
The foreground color of the button's text and icon when MaterialButton.onPressed is null (when MaterialButton.enabled is false).

getElevation(MaterialButton button) -> double
The button's elevation when it is enabled and has not been pressed.

getFillColor(MaterialButton button) -> Color?
The button's background fill color or null for buttons that don't have a background color.

getFocusColor(MaterialButton button) -> Color
The fill color of the button when it has input focus.

getFocusElevation(MaterialButton button) -> double
The button's elevation when it is enabled and has focus.

getHighlightColor(MaterialButton button) -> Color
The color of the overlay that appears when the button is pressed.

getHighlightElevation(MaterialButton button) -> double
The button's elevation when it is enabled and has been pressed.

getHoverColor(MaterialButton button) -> Color
The fill color of the button when it has input focus.

getHoverElevation(MaterialButton button) -> double
The button's elevation when it is enabled and has focus.

getMaterialTapTargetSize(MaterialButton button) -> MaterialTapTargetSize
The minimum size of the button's tap target.

getPadding(MaterialButton button) -> EdgeInsetsGeometry
Padding for the button's child (typically the button's label).

getShape(MaterialButton button) -> ShapeBorder
The shape of the button's Material.

getSplashColor(MaterialButton button) -> Color
The color of the ink "splash" overlay that appears when the (enabled) button is tapped.

getTextColor(MaterialButton button) -> Color
The foreground color of the button's text and icon.

getTextTheme(MaterialButton button) -> ButtonTextTheme
Defines the button's base colors, and the defaults for the button's minimum size, internal padding, and shape.
*/

pub struct ButtonThemeData {
    pub text_theme: ButtonTextTheme,
    pub min_width: f32,
    pub height: f32,
    pub padding: Box<dyn EdgeInsetsGeometry>,
    pub shape: Box<dyn ShapeBorder>,
    pub layout_behavior: ButtonBarLayoutBehavior,
    pub aligned_dropdown: bool,
    pub button_color: Color,
    pub disabled_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub highlight_color: Color,
    pub splash_color: Color,
    pub color_scheme: ColorScheme,
    pub material_tap_target_size: MaterialTapTargetSize,
}

impl Default for ButtonThemeData {
    fn default() -> Self {
        Self {
            text_theme: Default::default(),
            min_width: Default::default(),
            height: Default::default(),
            padding: box NoneEdgeInsetsGeometry,
            shape: box NoneShapeBorder,
            layout_behavior: Default::default(),
            aligned_dropdown: Default::default(),
            button_color: Default::default(),
            disabled_color: Default::default(),
            focus_color: Default::default(),
            hover_color: Default::default(),
            highlight_color: Default::default(),
            splash_color: Default::default(),
            color_scheme: Default::default(),
            material_tap_target_size: Default::default(),
        }
    }
}
