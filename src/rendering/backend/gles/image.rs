use std::{fmt, rc::Rc};

use crate::prelude::color;

use crate::{elements::ImageElement, engine::d2::Painter, rendering::backend::WidgetRenderer};

pub struct ImageRender {
    painter: Rc<Painter>,
}

impl ImageRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<ImageElement> for ImageRender {
    fn render(&self, widget: &ImageElement) {
        let comp = widget.as_ref().borrow();

        if let Some(ref image) = widget.image {
            let dx = (comp.w - image.width as f32) / 2.0;
            let dy = (comp.h - image.height as f32) / 2.0;
            
            self.painter.set_color(color::WHITE); // clear tint
            self.painter.draw_image(image, comp.x + dx, comp.y + dy);
        } else {
            self.painter.set_color(color::YELLOW_7);
            self.painter.fill_rect(comp.x, comp.y, comp.w, comp.h);
        }
    }
}

impl fmt::Debug for ImageRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageRender").finish()
    }
}