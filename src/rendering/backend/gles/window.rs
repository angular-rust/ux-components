use crate::{elements::WindowElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct WindowRender;

impl WidgetRenderer<WindowElement> for WindowRender {
    fn render(&self, widget: &WindowElement) {
        println!("WindowRender");
    }
}
