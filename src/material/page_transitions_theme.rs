use std::collections::HashMap;

use crate::foundation::TargetPlatform;

use super::PageTransitionsBuilder;

pub struct PageTransitionsTheme {
    pub builders: HashMap<TargetPlatform, PageTransitionsBuilder>, // = _defaultBuilders
}

impl Default for PageTransitionsTheme {
    fn default() -> Self {
        Self {
            builders: Default::default(),
        }
    }
}
