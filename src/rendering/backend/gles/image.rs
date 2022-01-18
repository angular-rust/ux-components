use crate::{elements::ImageElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct ImageRender;

impl WidgetRenderer<ImageElement> for ImageRender {
    fn render(&self, widget: &ImageElement) {
        println!("CanvasRender");
    }
}
