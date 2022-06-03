// toLanguageTag() → String
// Returns a syntactically valid Unicode BCP47 Locale Identifier.
//
// toString() → String
// Returns a string representing the locale.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Locale {
    pub language_code: String, // = "und",
    pub script_code: String,
    pub country_code: String,
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            language_code: Default::default(),
            script_code: Default::default(),
            country_code: Default::default(),
        }
    }
}
