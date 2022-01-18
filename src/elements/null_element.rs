use std::{cell::RefCell, rc::Rc};

use super::{Element, WidgetComponent};

#[derive(Default)]
pub struct NullElement {
    pub component: Rc<RefCell<WidgetComponent>>,
}

impl Element for NullElement {
    // Empty impl to exclude default behavior
    fn render(&self) {}
}

impl AsRef<RefCell<WidgetComponent>> for NullElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}
