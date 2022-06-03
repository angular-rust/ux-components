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
        // println!("TextRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        // self.painter.set_color(color::RED_5);

        // self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);

        let font_size = 18.0;
        self.painter.set_color(color::WHITE);
        self.painter.set_font_size(font_size);

        // if let Some((w, h)) = self.painter.measure(widget.text().as_str()) {
        //     println!("Text {}x{}", w, h);
        // }
        let dy = (comp.h - font_size) / 2.0;

        self.painter.draw_string(widget.text().as_str(), comp.x, comp.y + dy);
    }
}

impl fmt::Debug for TextRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextRender").finish()
    }
}
