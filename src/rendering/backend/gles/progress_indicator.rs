use crate::{elements::ProgressIndicatorElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct ProgressIndicatorRender;

impl WidgetRenderer<ProgressIndicatorElement> for ProgressIndicatorRender {
    fn render(&self, widget: &ProgressIndicatorElement) {}
}
