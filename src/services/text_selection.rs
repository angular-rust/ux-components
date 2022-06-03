use crate::ui::{TextAffinity, TextPosition};

#[derive(Default)]
pub struct TextSelection {
    // If the text range is collapsed and has more than one visual location 
    // (e.g., occurs at a line break), which of the two locations to use when pai32ing the caret.
    pub affinity: TextAffinity,
    
    // The position at which the selection originates.
    pub base: TextPosition,
    
    // The offset at which the selection originates.
    pub base_offset: i32,
    
    // The next index after the characters in this range.
    pub end: i32,
    
    // The position at which the selection terminates.
    pub extent: TextPosition,
    
    // The offset at which the selection terminates.
    pub extent_offset: i32,
    
    // Whether this range is empty (but still potentially placed inside the text).
    pub is_collapsed: bool,
    
    // Whether this selection has disambiguated its base and extent.
    pub is_directional: bool,
    
    // Whether the start of this range precedes the end.
    pub is_normalized: bool,
    
    // Whether this range represents a valid position in the text.
    pub is_valid: bool,
    
    // The index of the first character in the range.
    pub start: i32,
}

impl TextSelection {
    // copyWith({i32? baseOffset, i32? extentOffset, TextAffinity? affinity, bool? isDirectional}) → TextSelection
    // Creates a new TextSelection based on the current selection, with the provided parameters overridden. 
    // expandTo(TextPosition position, [bool extentAtIndex = false]) → TextSelection
    // Returns the smallest TextSelection that this could expand to in order to include the given TextPosition. 
    // extendTo(TextPosition position) → TextSelection
    // Keeping the selection's TextSelection.baseOffset fixed, pivot the TextSelection.extentOffset to the given TextPosition. 
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // textAfter(String text) → String
    // The text after this range.
    // inherited
    // textBefore(String text) → String
    // The text before this range.
    // inherited
    // textInside(String text) → String
    // The text inside this range.
    // inherited
    // toString() → String
    // A string representation of this object.
    // override
}
