pub struct FontFeature {
    pub feature: String,
    pub value: i32, // = 1
}

impl Default for FontFeature {
    fn default() -> Self {
        Self {
            feature: Default::default(),
            value: Default::default(),
        }
    }
}
