use crate::{elements::ListElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct ListRender;

impl WidgetRenderer<ListElement> for ListRender {
    fn render(&self, widget: &ListElement) {
        println!("ListRender");
    }
}
