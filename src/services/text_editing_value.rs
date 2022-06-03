use crate::ui::TextRange;

use super::TextSelection;

#[derive(Default)]
pub struct TextEditingValue {
    // The range of text that is still being composed.
    pub composing: TextRange,
    
    // Whether the composing range is a valid range within text.
    pub is_composing_range_valid: bool,
    
    // The range of text that is currently selected.
    pub selection: TextSelection,
    
    // The current text being edited.
    pub text: String,
}

impl TextEditingValue {
    // copyWith({String? text, TextSelection? selection, TextRange? composing}) → TextEditingValue
    // Creates a copy of this value but with the given fields replaced with the new values. 
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // replaced(TextRange replacementRange, String replacementString) → TextEditingValue
    // Returns a new TextEditingValue, which is this TextEditingValue with its text partially replaced by the replacementString. 
    // toJSON() → Map<String, dynamic>
    // Returns a representation of this object as a JSON object. 
    // toString() → String
    // A string representation of this object.
    // override
}