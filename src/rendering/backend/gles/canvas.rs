use crate::{elements::CanvasElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct CanvasRender;

impl WidgetRenderer<CanvasElement> for CanvasRender {
    fn render(&self, widget: &CanvasElement) {}
}
