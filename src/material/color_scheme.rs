use crate::{foundation::colorspace::Color, ui::Brightness};

/*
copyWith({Color? primary, Color? primaryVariant, Color? secondary, Color? secondaryVariant, Color? surface, Color? background, Color? error, Color? onPrimary, Color? onSecondary, Color? onSurface, Color? onBackground, Color? onError, Brightness? brightness}) -> ColorScheme
Creates a copy of this color scheme with the given fields replaced by the non-null parameter values.

debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
Add additional properties associated with the node.
*/

pub struct ColorScheme {
    pub primary: Color,
    pub primary_variant: Color,
    pub secondary: Color,
    pub secondary_variant: Color,
    pub surface: Color,
    pub background: Color,
    pub error: Color,
    pub on_primary: Color,
    pub on_secondary: Color,
    pub on_surface: Color,
    pub on_background: Color,
    pub on_error: Color,
    pub brightness: Brightness,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: Default::default(),
            primary_variant: Default::default(),
            secondary: Default::default(),
            secondary_variant: Default::default(),
            surface: Default::default(),
            background: Default::default(),
            error: Default::default(),
            on_primary: Default::default(),
            on_secondary: Default::default(),
            on_surface: Default::default(),
            on_background: Default::default(),
            on_error: Default::default(),
            brightness: Default::default(),
        }
    }
}
