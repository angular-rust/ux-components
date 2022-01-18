use crate::{elements::ScaffoldElement, rendering::backend::WidgetRenderer};

/// ScaffoldRender maybe used to fill background of application
#[derive(Debug)]
pub struct ScaffoldRender;

impl WidgetRenderer<ScaffoldElement> for ScaffoldRender {
    fn render(&self, widget: &ScaffoldElement) {}
}
