pub struct FontWeight(pub usize);

impl FontWeight {

    // A commonly used font weight that is heavier than normal.
    pub const BOLD: FontWeight = Self::W700;
    
    // The default font weight.
    pub const NORMAL: FontWeight = Self::W400;
    
    // A list of all the font weights.
    pub const VALUES: [FontWeight; 9] = [Self::W100, Self::W200, Self::W300, Self::W400, Self::W500, Self::W600, Self::W700, Self::W800, Self::W900];
    
    // Thin, the least thick
    pub const W100: FontWeight = FontWeight(0);
    
    // Extra-light
    pub const W200: FontWeight = FontWeight(1);
    
    // Light
    pub const W300: FontWeight = FontWeight(2);
    
    // Normal / regular / plain
    pub const W400: FontWeight = FontWeight(3);
    
    // Medium
    pub const W500: FontWeight = FontWeight(4);
    
    // Semi-bold
    pub const W600: FontWeight = FontWeight(5);
    
    // Bold
    pub const W700: FontWeight = FontWeight(6);
    
    // Extra-bold
    pub const W800: FontWeight = FontWeight(7);
    
    // Black, the most thick
    pub const W900: FontWeight = FontWeight(8);
}

impl Default for FontWeight {
    fn default() -> Self {
        Self(0)
    }
}
