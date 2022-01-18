use crate::{elements::TextEditElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct TextEditRender;

impl WidgetRenderer<TextEditElement> for TextEditRender {
    fn render(&self, widget: &TextEditElement) {
        println!("TextEditRender");
    }
}
