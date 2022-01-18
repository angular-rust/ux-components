use crate::{elements::CheckboxElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct CheckboxRender;

impl WidgetRenderer<CheckboxElement> for CheckboxRender {
    fn render(&self, widget: &CheckboxElement) {
        println!("CheckboxRender");
    }
}
