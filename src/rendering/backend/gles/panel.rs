use crate::{elements::PanelElement, rendering::backend::WidgetRenderer};

#[derive(Debug)]
pub struct PanelRender;

impl WidgetRenderer<PanelElement> for PanelRender {
    fn render(&self, widget: &PanelElement) {}
}
