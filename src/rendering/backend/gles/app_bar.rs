use std::{fmt, rc::Rc};

use crate::{
    elements::AppBarElement, engine::d2::Painter, prelude::color,
    rendering::backend::WidgetRenderer,
};

pub struct AppBarRender {
    painter: Rc<Painter>,
}

impl AppBarRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<AppBarElement> for AppBarRender {
    fn render(&self, widget: &AppBarElement) {
        let comp = widget.as_ref().borrow();

        // println!("AppBarRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(color::WHITE);

        self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);
    }
}

impl fmt::Debug for AppBarRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppBarRender").finish()
    }
}
