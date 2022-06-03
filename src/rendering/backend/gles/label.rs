use crate::{elements::LabelElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct LabelRender;

impl WidgetRenderer<LabelElement> for LabelRender {
    fn render(&self, widget: &LabelElement) {}
}
