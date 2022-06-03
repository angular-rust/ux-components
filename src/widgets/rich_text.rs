use crate::{
    elements::{Element, RichTextElement},
    foundation::{Id, Key, WidgetProperties},
    painting::{InlineSpan, TextOverflow, NoneInlineSpan},
    ui::{Locale, TextAlign, TextDirection},
    widgets::Widget,
};

pub struct RichText {
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,

    // The text to display in this widget.
    pub text: Box<dyn InlineSpan>,

    // How the text should be aligned horizontally.
    pub text_align: TextAlign,

    // The directionality of the text.
    //
    // This decides how [textAlign] values like [TextAlign.start] and
    // [TextAlign.end] are interpreted.
    //
    // This is also used to disambiguate how to render bidirectional text. For
    // example, if the [text] is an English phrase followed by a Hebrew phrase,
    // in a [TextDirection.ltr] context the English phrase will be on the left
    // and the Hebrew phrase to its right, while in a [TextDirection.rtl]
    // context, the English phrase will be on the right and the Hebrew phrase on
    // its left.
    //
    // Defaults to the ambient [Directionality], if any. If there is no ambient
    // [Directionality], then this must not be null.
    pub text_direction: TextDirection,

    // Whether the text should break at soft line breaks.
    //
    // If false, the glyphs in the text will be positioned as if there was unlimited horizontal space.
    pub soft_wrap: bool,

    // How visual overflow should be handled.
    pub overflow: TextOverflow,

    // The number of font pixels for each logical pixel.
    //
    // For example, if the text scale factor is 1.5, text will be 50% larger than
    // the specified font size.
    pub text_scale_factor: f32,

    // An optional maximum number of lines for the text to span, wrapping if necessary.
    // If the text exceeds the given number of lines, it will be truncated according
    // to [overflow].
    //
    // If this is 1, text will not wrap. Otherwise, text will be wrapped at the
    // edge of the box.
    pub max_lines: i32,

    // Used to select a font when the same Unicode character can
    // be rendered differently, depending on the locale.
    //
    // It's rarely necessary to set this property. By default its value
    // is inherited from the enclosing app with `Localizations.localeOf(context)`.
    //
    // See [RenderParagraph.locale] for more information.
    pub locale: Locale,

    // // {@macro flutter.painting.textPainter.strutStyle}
    // pub strut_style: StrutStyle,

    // // {@macro flutter.painting.textPainter.textWidthBasis}
    // pub text_width_basis: TextWidthBasis,

    // //{@macro ui.TextHeightBehavior}
    // pub text_height_behavior: TextHeightBehavior,
}

impl Default for RichText {
    fn default() -> Self {
        Self {
            key: Default::default(),
            text: box NoneInlineSpan,
            text_align: Default::default(),
            text_direction: Default::default(),
            soft_wrap: Default::default(),
            overflow: Default::default(),
            text_scale_factor: Default::default(),
            max_lines: Default::default(),
            locale: Default::default(),
            // strut_style: Default::default(),
            // text_width_basis: Default::default(),
            // text_height_behavior: Default::default(),
        }
    }
}

impl Widget for RichText {
    fn create_element(&self) -> Box<dyn Element> {
        box RichTextElement::new(self)
    }
}

impl WidgetProperties for RichText {
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
