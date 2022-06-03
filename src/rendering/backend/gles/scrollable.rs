use crate::{elements::ScrollableElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct ScrollableRender;

impl WidgetRenderer<ScrollableElement> for ScrollableRender {
    fn render(&self, widget: &ScrollableElement) {}
}
