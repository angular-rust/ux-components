pub struct TextInputType {
    // The number is decimal, allowing a decimal point to provide fractional.
    pub decimal: bool,

    // Enum value index, corresponds to one of the values.
    pub index: usize,

    // The number is signed, allowing a positive or negative sign at the start.
    pub signed: bool,
}

impl TextInputType {
    // Optimize for textual information. [...]
    pub const TEXT: TextInputType = TextInputType {
        index: 0,
        decimal: false,
        signed: false,
    };

    // Optimize for multiline textual information. [...]
    pub const MULTILINE: TextInputType = TextInputType {
        index: 1,
        decimal: false,
        signed: false,
    };

    // Optimize for unsigned numerical information without a decimal point. [...]
    pub const NUMBER: TextInputType = TextInputType {
        index: 2,
        decimal: false,
        signed: false,
    };

    // Optimize for telephone numbers. [...]
    pub const PHONE: TextInputType = TextInputType {
        index: 3,
        decimal: false,
        signed: false,
    };

    // Optimize for date and time information. [...]
    pub const DATATIME: TextInputType = TextInputType {
        index: 4,
        decimal: false,
        signed: false,
    };

    // Optimize for email addresses. [...]
    pub const EMAIL_ADDRESS: TextInputType = TextInputType {
        index: 5,
        decimal: false,
        signed: false,
    };

    // Optimize for URLs. [...]
    pub const URL: TextInputType = TextInputType {
        index: 6,
        decimal: false,
        signed: false,
    };

    // Optimize for passwords that are visible to the user. [...]
    pub const VISIBLE_PASSWORD: TextInputType = TextInputType {
        index: 7,
        decimal: false,
        signed: false,
    };

    // Optimized for a person's name. [...]
    pub const NAME: TextInputType = TextInputType {
        index: 8,
        decimal: false,
        signed: false,
    };

    // Optimized for postal mailing addresses. [...]
    pub const STREET_ADDRESS: TextInputType = TextInputType {
        index: 9,
        decimal: false,
        signed: false,
    };

    // Prevent the OS from showing the on-screen virtual keyboard.
    pub const NONE: TextInputType = TextInputType {
        index: 10,
        decimal: false,
        signed: false,
    };
    // toJson() → Map<String, dynamic>
    // Returns a representation of this object as a JSON object.
}

impl Default for TextInputType {
    fn default() -> Self {
        Self {
            decimal: Default::default(),
            index: Default::default(),
            signed: Default::default(),
        }
    }
}
