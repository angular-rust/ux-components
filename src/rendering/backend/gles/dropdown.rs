use crate::{elements::DropdownButtonElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct DropdownRender;

impl WidgetRenderer<DropdownButtonElement> for DropdownRender {
    fn render(&self, widget: &DropdownButtonElement) {}
}
