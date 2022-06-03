use std::{fmt, rc::Rc};

use crate::prelude::color;

use crate::{
    elements::NavigationRailElement,
    engine::d2::Painter,
    platform::gles::{core30::gl, enums::*},
    rendering::backend::WidgetRenderer,
};

pub struct NavigationRailRender {
    painter: Rc<Painter>,
}

impl NavigationRailRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<NavigationRailElement> for NavigationRailRender {
    fn render(&self, widget: &NavigationRailElement) {
        let comp = widget.as_ref().borrow();

        // println!("Node {:?}", widget.node);
        // println!("NavigationRailRender {}x{} {}x{}", comp.x, comp.y, comp.w, comp.h);

        self.painter.set_color(color::GRAY_7);

        self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);

        self.painter.flush();
    }
}

impl fmt::Debug for NavigationRailRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationRailRender").finish()
    }
}
