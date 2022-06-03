#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextLeadingDistribution {
    Proportional = 0,
    Even = 1,
}

impl Default for TextLeadingDistribution {
    fn default() -> Self {
        Self::Proportional
    }
}
