#![allow(unused_variables)]
use std::fmt::Debug;

use crate::elements::WidgetComponent;

/// A widget renderer interface.
pub trait WidgetRenderer<T>: Debug {
    // fn rendering(&self) -> Rendering;
    // fn control(&self) -> WidgetComponent;
    fn render(&self, widget: &T);

    fn post_render(&self, widget: &T) {}
}

/// The basic implementation for a control Renderer.
/// This includes convenience functions and implementation
/// of the base control events which are called
/// to simplify integration. Be aware of the timing of signals.
/// Implementation should contain:
///
/// rendering: Rendering,
/// control: WidgetComponent,
pub trait BaseRender
where
    Self: AsRef<WidgetComponent>,
{
    // fn new(render: Rendering, control: WidgetComponent) {
    //     control = control;
    //     rendering = render;

    //     control.oncreate.listen(internal_connect);
    // }

    /// Don"t need to call this from Render subclasses
    fn internal_connect(&self) {
        // let control = self.as_ref();
        // control.canvas.onscalechange.listen(onscale);
        // control.onvisible.listen(onvisible);
        // control.ondepth.listen(ondepth);
        // control.ondestroy.listen(ondestroy);
        // control.onclip.listen(onclip);
        // control.onchildadd.listen(onchildadd);
        // control.onchildremove.listen(onchildremove);
        // control.onbounds.listen(onbounds);
        // control.ondestroy.listen(internal_disconnect);
    }

    /// Don"t need to call this from Render subclasses
    fn internal_disconnect(&self) {
        // let control = self.as_ref();
        // control.canvas.onscalechange.remove(onscale);
        // control.onvisible.remove(onvisible);
        // control.ondepth.remove(ondepth);
        // control.ondestroy.remove(ondestroy);
        // control.onclip.remove(onclip);
        // control.onchildadd.remove(onchildadd);
        // control.onchildremove.remove(onchildremove);
        // control.onbounds.remove(onbounds);

        // control.oncreate.remove(internal_connect);
        // control.ondestroy.remove(internal_disconnect);
    }

    #[inline]
    fn scale(&self) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            canvas.scale()
        } else {
            1.0
        }
    }

    #[inline]
    fn sx(&self) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            control.x * canvas.scale()
        } else {
            control.x
        }
    }

    #[inline]
    fn sy(&self) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            control.y * canvas.scale()
        } else {
            control.y
        }
    }

    #[inline]
    fn sw(&self) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            control.w * canvas.scale()
        } else {
            control.w
        }
    }

    #[inline]
    fn sh(&self) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            control.h * canvas.scale()
        } else {
            control.h
        }
    }

    #[inline]
    fn cs(&self, value: f32) -> f32 {
        let control = self.as_ref();
        if let Some(canvas) = control.canvas.as_ref() {
            value * canvas.scale()
        } else {
            value
        }
    }

    fn onscale(&self, scale: f32, prev: f32) {}
    fn onvisible(&self, value: bool) {}
    fn ondepth(&self, depth: f32) {}
    fn ondestroy(&self) {}
    fn onbounds(&self) {}
    fn onclip(&self, disable: bool, x: f32, y: f32, w: f32, h: f32) {}
    fn onchildadd(&self, control: WidgetComponent) {}
    fn onchildremove(&self, control: WidgetComponent) {}
}
