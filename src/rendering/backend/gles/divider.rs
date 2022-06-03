use std::{fmt, rc::Rc};

use crate::{
    elements::DividerElement, engine::d2::Painter, prelude::color, rendering::backend::WidgetRenderer,
};

pub struct DividerRender {
    painter: Rc<Painter>,
}

impl DividerRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<DividerElement> for DividerRender {
    fn render(&self, widget: &DividerElement) {
        let comp = widget.as_ref().borrow();

        // println!("Node {:?}", widget.node);
        // println!("DividerRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(widget.color);

        let delta = if widget.height - widget.thickness == 0.0 {
            0.0
        } else {
            (widget.height - widget.thickness) / 2.0
        };

        let cx = comp.x + widget.indent;
        let cw = comp.w - widget.indent - widget.end_indent;

        self.painter.fill_rect(cx, comp.y + delta, cw, comp.h);
    }
}

impl fmt::Debug for DividerRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DividerRender").finish()
    }
}
