use super::{CssValue, CssProperty};
use intmap::IntMap;

#[derive(Clone)]
pub struct CssRule {
    inner: IntMap<CssValue>,
}

impl Default for CssRule {
    fn default() -> Self {
        CssRule::new()
    }
}

impl CssRule {
    fn new() -> Self {
        Self{
            inner: IntMap::new()
        }
    }

    pub fn get(&self, key: CssProperty) -> Option<&CssValue> {
        self.inner.get(key.into())
    }
}
