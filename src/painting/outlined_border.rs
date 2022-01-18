use super::BorderSide;

pub struct OutlinedBorder {
    pub side: BorderSide,
}

impl Default for OutlinedBorder {
    fn default() -> Self {
        Self {
            side: Default::default(),
        }
    }
}
