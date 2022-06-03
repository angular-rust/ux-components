use crate::{services::{MouseCursor, PointerEnterEventListener, PointerExitEventListener}, ui::Locale, gestures::GestureRecognizer};

use super::{TextStyle, InlineSpan};

pub struct TextSpan {
    // Additional spans to include as children.
    pub children: Vec<Box<dyn InlineSpan>>,

    // Returns the value of mouseCursor.
    pub cursor: MouseCursor,

    // The language of the text in this span and its span children.
    pub locale: Option<Locale>,

    // Mouse cursor when the mouse hovers over this span.
    pub mouse_cursor: MouseCursor,

    // Triggered when a mouse pointer, with or without buttons pressed, has entered the region and validForMouseTracker is true.
    pub on_enter: PointerEnterEventListener,

    // Triggered when a mouse pointer, with or without buttons pressed, has exited the region and validForMouseTracker is true.
    pub on_exit: PointerExitEventListener,

    // A gesture recognizer that will receive events that hit this span.
    pub recognizer: GestureRecognizer,

    // An alternative semantics label for this TextSpan.
    pub semantics_label: String,

    // Whether the assistive technologies should spell out this text character by character.
    pub spell_out: bool,

    // The TextStyle to apply to this span.
    pub style: TextStyle,

    // The text contained in this span.
    pub text: String,

    // Whether this is included when MouseTracker collects the list of annotations.
    pub valid_for_mouse_tracker: bool,
}

impl Default for TextSpan {
    fn default() -> Self {
        Self {
            children: Default::default(),
            cursor: Default::default(),
            locale: Default::default(),
            mouse_cursor: Default::default(),
            on_enter: box |_| {},
            on_exit: box |_| {},
            recognizer: Default::default(),
            semantics_label: Default::default(),
            spell_out: Default::default(),
            style: Default::default(),
            text: Default::default(),
            valid_for_mouse_tracker: Default::default(),
        }
    }
}
