use std::{fmt, rc::Rc};

use crate::prelude::color;

use crate::{elements::ScaffoldElement, engine::d2::Painter, rendering::backend::WidgetRenderer};

/// ScaffoldRender maybe used to fill background of application
pub struct ScaffoldRender {
    painter: Rc<Painter>,
}

impl ScaffoldRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<ScaffoldElement> for ScaffoldRender {
    fn render(&self, widget: &ScaffoldElement) {
        let comp = widget.as_ref().borrow();

        log::info!("Render {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);
        self.painter.set_color(color::GRAY_8);

        self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);
    }
}

impl fmt::Debug for ScaffoldRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScaffoldRender").finish()
    }
}