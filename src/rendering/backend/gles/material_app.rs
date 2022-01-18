use cgmath::Matrix4;
use std::{fmt, rc::Rc};

use crate::{
    elements::MaterialAppElement,
    engine::d2::Painter,
    platform::gles::{core30::gl, enums::*},
    rendering::backend::WidgetRenderer,
};

/// MaterialAppRender primary used to clear surface,
/// set viewport and projection matrix in render pahase
/// and finally to flush paintings in post render phase
pub struct MaterialAppRender {
    painter: Rc<Painter>,
}

impl MaterialAppRender {
    pub fn new(painter: Rc<Painter>) -> Self {
        Self { painter }
    }
}

impl WidgetRenderer<MaterialAppElement> for MaterialAppRender {
    fn render(&self, widget: &MaterialAppElement) {
        let comp = widget.as_ref().borrow();
        gl::viewport(0, 0, comp.w as i32, comp.h as i32);

        gl::clear_color(0.3, 0.3, 0.5, 1.0);

        // Enable auto-conversion from/to sRGB
        // gl::enable(GL_FRAMEBUFFER_SRGB);

        // Enable alpha blending
        gl::enable(GL_BLEND);
        gl::blend_func(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        // Disable multisampling by default
        // gl::disable(GL_MULTISAMPLE);

        gl::clear(GL_COLOR_BUFFER_BIT);

        // log::debug!(
        //     "Render MaterialApp {}x{}",
        //     comp.w, comp.h
        // );

        // painter things
        let proj_mat: Matrix4<f32> = cgmath::ortho(0.0, comp.w, comp.h, 0.0, -0.1, 1000.0);
        self.painter.set_projection(proj_mat);
        self.painter.set_font_size(48.0);
    }

    fn post_render(&self, widget: &MaterialAppElement) {
        self.painter.end();
    }
}

impl fmt::Debug for MaterialAppRender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppBarRender").finish()
    }
}
