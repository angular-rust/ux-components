use std::{fmt, rc::Rc};

use crate::{
    elements::NavigationRailDestinationElement, engine::d2::Painter, prelude::color,
    rendering::backend::WidgetRenderer,
};

pub struct NavigationRailDestinationRender {
    painter: Rc<Painter>,
}

impl NavigationRailDestinationRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<NavigationRailDestinationElement> for NavigationRailDestinationRender {
    fn render(&self, widget: &NavigationRailDestinationElement) {
        // let comp = widget.as_ref().borrow();

        // self.painter.set_color(color::GREEN_7);

        // self.painter
        //     .fill_rect(comp.x + 1.0, comp.y + 1.0, comp.w - 2.0, comp.h - 2.0);

        // self.painter.flush();
    }
}

impl fmt::Debug for NavigationRailDestinationRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationRailDestinationRender").finish()
    }
}
