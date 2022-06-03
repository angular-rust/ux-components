use std::{fmt, rc::Rc};

use crate::{
    elements::MaterialButtonElement, engine::d2::Painter, prelude::color, rendering::backend::WidgetRenderer,
};

pub struct MaterialButtonRender {
    painter: Rc<Painter>,
}

impl MaterialButtonRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<MaterialButtonElement> for MaterialButtonRender {
    fn render(&self, widget: &MaterialButtonElement) {
        let comp = widget.as_ref().borrow();

        // println!("Node {:?}", widget.node);
        // println!("MaterialButtonRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(color::ORANGE_5);

        self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);
    }
}

impl fmt::Debug for MaterialButtonRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialButtonRender").finish()
    }
}
