use crate::{elements::ButtonElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct ButtonRender;

impl WidgetRenderer<ButtonElement> for ButtonRender {
    fn render(&self, widget: &ButtonElement) {}
}
