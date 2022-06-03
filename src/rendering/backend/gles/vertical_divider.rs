use std::{fmt, rc::Rc};

use crate::{
    elements::VerticalDividerElement, engine::d2::Painter, prelude::color, rendering::backend::WidgetRenderer,
};

pub struct VerticalDividerRender {
    painter: Rc<Painter>,
}

impl VerticalDividerRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<VerticalDividerElement> for VerticalDividerRender {
    fn render(&self, widget: &VerticalDividerElement) {
        let comp = widget.as_ref().borrow();

        // println!("Node {:?}", widget.node);
        // println!("VerticalDividerRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(widget.color);
       
        let delta = if widget.width - widget.thickness == 0.0 {
            0.0
        } else {
            (widget.width - widget.thickness) / 2.0
        };

        let cy = comp.y + widget.indent;
        let ch = comp.h - widget.indent - widget.end_indent;

        self.painter.fill_rect(comp.x + delta, cy, widget.thickness, ch);
    }
}

impl fmt::Debug for VerticalDividerRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerticalDividerRender").finish()
    }
}
