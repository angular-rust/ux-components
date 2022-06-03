use super::TextAffinity;

#[derive(Default)]
pub struct TextPosition {
    // Disambiguates cases where the position in the string given by offset 
    // could represent two different visual positions in the rendered text. 
    // For example, this can happen when text is forced to wrap, or when 
    // one string of text is rendered with multiple text directions.
    pub affinity: TextAffinity,
    
    // The index of the character that immediately follows the position 
    // in the string representation of the text.
    pub offset: i32,
}