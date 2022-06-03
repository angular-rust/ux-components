use crate::{elements::SliderElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct SliderRender;

impl WidgetRenderer<SliderElement> for SliderRender {
    fn render(&self, widget: &SliderElement) {}
}
