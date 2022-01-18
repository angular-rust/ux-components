use crate::{
    elements::{Element, NullElement},
    ui::Size,
    widgets::Widget,
};

use super::PreferredSizeWidget;

/// NullWidget used only as default widget to simplify declarative sintax
pub struct NullWidget;

impl Default for NullWidget {
    fn default() -> Self {
        Self {}
    }
}

impl PreferredSizeWidget for NullWidget {
    fn preferred_size(&self) -> Size {
        Size(0.0, 0.0)
    }
}

impl Widget for NullWidget {
    fn create_element(&self) -> Box<dyn Element> {
        box NullElement::default()
    }
}
