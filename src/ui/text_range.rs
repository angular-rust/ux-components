
#[derive(Default)]
pub struct TextRange {
    // The next index after the characters in this range.
    pub end: i32,
    
    // Whether this range is empty (but still potentially placed inside the text).
    pub is_collapsed: bool,
    
    // Whether the start of this range precedes the end.
    pub is_normalized: bool,
    
    // Whether this range represents a valid position in the text.
    pub is_valid: bool,
    
    // The index of the first character in the range.
    pub start: i32,
}

impl TextRange {
    // textAfter(String text) → String
    // The text after this range. 
    // textBefore(String text) → String
    // The text before this range. 
    // textInside(String text) → String
    // The text inside this range. 
    // toString() → String
    // A string representation of this object.
}