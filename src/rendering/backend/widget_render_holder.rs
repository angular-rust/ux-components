use std::fmt::{Debug, Formatter, Result};

use super::WidgetRenderer;

pub struct WidgetRenderHolder<T>(pub Box<dyn WidgetRenderer<T>>);

impl<T> WidgetRenderer<T> for WidgetRenderHolder<T> {
    fn render(&self, widget: &T) {
        self.0.render(widget);
    }

    fn post_render(&self, widget: &T) {
        self.0.post_render(widget);
    }
}

impl<T> Debug for WidgetRenderHolder<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("WidgetRenderHolder").field(&self.0).finish()
    }
}
