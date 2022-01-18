use std::{fmt, rc::Rc};

use crate::{
    elements::TextElement, engine::d2::Painter, prelude::color, rendering::backend::WidgetRenderer,
};

pub struct TextRender {
    painter: Rc<Painter>,
}

impl TextRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<TextElement> for TextRender {
    fn render(&self, widget: &TextElement) {
        let comp = widget.as_ref().borrow();

        // println!("Node {:?}", widget.node);
        println!("TextRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(color::TEAL_5);

        self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);
    }
}

impl fmt::Debug for TextRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextRender").finish()
    }
}
