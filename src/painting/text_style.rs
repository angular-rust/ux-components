use crate::{
    foundation::colorspace::Color,
    ui::{
        FontFeature, FontStyle, FontWeight, Locale, Paint, Shadow, TextBaseline,
        TextDecorationStyle, TextLeadingDistribution,
    },
};

use super::TextOverflow;

// apply({Color? color, Color? backgroundColor, TextDecoration? decoration, Color? decorationColor, TextDecorationStyle? decorationStyle, double decorationThicknessFactor = 1.0, double decorationThicknessDelta = 0.0, String? fontFamily, List<String>? fontFamilyFallback, double fontSizeFactor = 1.0, double fontSizeDelta = 0.0, int fontWeightDelta = 0, FontStyle? fontStyle, double letterSpacingFactor = 1.0, double letterSpacingDelta = 0.0, double wordSpacingFactor = 1.0, double wordSpacingDelta = 0.0, double heightFactor = 1.0, double heightDelta = 0.0, TextBaseline? textBaseline, TextLeadingDistribution? leadingDistribution, Locale? locale, List<Shadow>? shadows, List<FontFeature>? fontFeatures, TextOverflow? overflow}) -> TextStyle
// Creates a copy of this text style replacing or altering the specified properties.
//
// compareTo(TextStyle other) -> RenderComparison
// Describe the difference between this style and another, in terms of how much damage it will make to the rendering.
//
// copyWith({bool? inherit, Color? color, Color? backgroundColor, String? fontFamily, List<String>? fontFamilyFallback, double? fontSize, FontWeight? fontWeight, FontStyle? fontStyle, double? letterSpacing, double? wordSpacing, TextBaseline? textBaseline, double? height, TextLeadingDistribution? leadingDistribution, Locale? locale, Paint? foreground, Paint? background, List<Shadow>? shadows, List<FontFeature>? fontFeatures, TextDecoration? decoration, Color? decorationColor, TextDecorationStyle? decorationStyle, double? decorationThickness, String? debugLabel, TextOverflow? overflow}) -> TextStyle
// Creates a copy of this text style but with the given fields replaced with the new values.
//
// debugFillProperties(DiagnosticPropertiesBuilder properties, {String prefix = ''}) -> void
// Adds all properties prefixing property names with the optional prefix.
// override
//
// getParagraphStyle({TextAlign? textAlign, TextDirection? textDirection, double textScaleFactor = 1.0, String? ellipsis, int? maxLines, TextHeightBehavior? textHeightBehavior, Locale? locale, String? fontFamily, double? fontSize, FontWeight? fontWeight, FontStyle? fontStyle, double? height, StrutStyle? strutStyle}) -> ParagraphStyle
// The style information for paragraphs, encoded for use by dart:ui.
//
// getTextStyle({double textScaleFactor = 1.0}) -> TextStyle
// The style information for text runs, encoded for use by dart:ui.
//
// merge(TextStyle? other) -> TextStyle
// Returns a new text style that is a combination of this style and the given other style.
//
// toStringShort() -> String
// A brief description of this object, usually just the runtimeType and the hashCode.

pub struct TextStyle {
    pub inherit: bool,
    pub color: Color,
    pub background_color: Color,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub letter_spacing: f32,
    pub word_spacing: f32,
    pub text_baseline: TextBaseline,
    pub height: f32,
    pub leading_distribution: TextLeadingDistribution,
    pub locale: Locale,
    pub foreground: Paint,
    pub background: Paint,
    pub shadows: Vec<Shadow>,
    pub font_features: Vec<FontFeature>,
    pub decoration: TextDecorationStyle,
    pub decoration_colo: Color,
    pub decoration_style: TextDecorationStyle,
    pub decoration_thickness: f32,
    pub debug_label: String,
    pub font_family: String,
    pub font_family_fallback: Vec<String>,
    pub package: String,
    pub overflow: TextOverflow,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            inherit: Default::default(),
            color: Default::default(),
            background_color: Default::default(),
            font_size: Default::default(),
            font_weight: Default::default(),
            font_style: Default::default(),
            letter_spacing: Default::default(),
            word_spacing: Default::default(),
            text_baseline: Default::default(),
            height: Default::default(),
            leading_distribution: Default::default(),
            locale: Default::default(),
            foreground: Default::default(),
            background: Default::default(),
            shadows: Default::default(),
            font_features: Default::default(),
            decoration: Default::default(),
            decoration_colo: Default::default(),
            decoration_style: Default::default(),
            decoration_thickness: Default::default(),
            debug_label: Default::default(),
            font_family: Default::default(),
            font_family_fallback: Default::default(),
            package: Default::default(),
            overflow: Default::default(),
        }
    }
}
