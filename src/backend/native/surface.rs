#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, ActorCanvas, Pattern, ScalingFilter, Widget};
use glib::signal::SignalHandlerId;
use primitives::CanvasContext;
use std::{cell::RefCell, fmt, mem};

#[derive(Clone, Debug)]
pub struct SurfaceProps {
    pub texture: Option<dx::Handle>,
    pub material: Option<dx::Handle>,
    pub frames: u32,
    pub anim_duration: u32,
    pub current_frame: u32,
    pub update_id: u32,
    pub animating: bool,
}

#[derive(Clone, Debug)]
pub struct Surface {
    props: RefCell<SurfaceProps>,
    canvas: ActorCanvas,
    widget: Widget,
}

impl Surface {
    pub fn new() -> Surface {
        let props = SurfaceProps {
            texture: None,
            material: None,
            frames: 1,
            anim_duration: 500,
            current_frame: 0,
            update_id: 0,
            animating: true,
        };

        println!("create surface");

        let widget = Widget::new();
        let content = ActorCanvas::new().unwrap();
        let canvas: ActorCanvas = unsafe { mem::transmute(content) };
        canvas.set_size(400, 400);

        {
            let actor: &Actor = widget.as_ref();

            actor.set_background_color(Some(color::RED_9));
            actor.set_size(100_f32, 100_f32);
            actor.set_position(50_f32, 50_f32);

            actor.set_content(Some(&canvas));
            actor.set_content_scaling_filters(ScalingFilter::Trilinear, ScalingFilter::Linear);
        }

        let surface = Self {
            props: RefCell::new(props),
            canvas,
            widget,
        };

        surface
    }
}

impl Default for Surface {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Surface {}
impl Is<Surface> for Surface {}

impl AsRef<Surface> for Surface {
    fn as_ref(&self) -> &Surface {
        self
    }
}

impl Is<Widget> for Surface {}

impl AsRef<Widget> for Surface {
    fn as_ref(&self) -> &Widget {
        &self.widget
    }
}

impl Is<ActorCanvas> for Surface {}

impl AsRef<ActorCanvas> for Surface {
    fn as_ref(&self) -> &ActorCanvas {
        &self.canvas
    }
}

impl Is<Actor> for Surface {}

impl AsRef<Actor> for Surface {
    fn as_ref(&self) -> &Actor {
        let actor: &Actor = self.widget.as_ref();
        actor
    }
}

pub trait SurfaceExt: 'static {
    /// get_animating:
    /// @spinner: A #Surface widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %true if the spinner is animating, %false otherwise
    ///
    fn get_animating(&self) -> bool;

    /// set_animating:
    /// @spinner: A #Surface widget
    /// @animating: %true to enable animation, %false to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool);

    /// The `Canvas::draw` signal is emitted each time a canvas is
    /// invalidated.
    ///
    /// It is safe to connect multiple handlers to this signal: each
    /// handler invocation will be automatically protected by `cairo_save`
    /// and `cairo_restore` pairs.
    /// ## `cr`
    /// the Cairo context used to draw
    /// ## `width`
    /// the width of the `canvas`
    /// ## `height`
    /// the height of the `canvas`
    ///
    /// # Returns
    ///
    /// `true` if the signal emission should stop, and
    ///  `false` otherwise
    fn connect_draw<F: Fn(&Self, &dyn CanvasContext<Pattern>, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Surface>> SurfaceExt for O {
    /// get_animating:
    /// @spinner: A #Surface widget
    ///
    /// Determines whether the spinner is animating.
    ///
    /// Returns: %true if the spinner is animating, %false otherwise
    ///
    fn get_animating(&self) -> bool {
        let spinner = self.as_ref();
        let props = spinner.props.borrow();
        props.animating
    }

    /// set_animating:
    /// @spinner: A #Surface widget
    /// @animating: %true to enable animation, %false to disable
    ///
    /// Sets whether the spinner is animating. A spinner can be stopped if
    /// the task it represents has finished, or to save energy.
    ///
    fn set_animating(&self, animating: bool) {
        let spinner = self.as_ref();
        let mut props = spinner.props.borrow_mut();

        if props.animating != animating {
            props.animating = animating;
            // update_timeout(spinner);
            // g_object_notify(G_OBJECT(spinner), "animating");
        }
    }

    /// The `Canvas::draw` signal is emitted each time a canvas is
    /// invalidated.
    ///
    /// It is safe to connect multiple handlers to this signal: each
    /// handler invocation will be automatically protected by `cairo_save`
    /// and `cairo_restore` pairs.
    /// ## `cr`
    /// the Cairo context used to draw
    /// ## `width`
    /// the width of the `canvas`
    /// ## `height`
    /// the height of the `canvas`
    ///
    /// # Returns
    ///
    /// `true` if the signal emission should stop, and
    ///  `false` otherwise
    fn connect_draw<F: Fn(&Self, &dyn CanvasContext<Pattern>, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        let surface = self.as_ref();

        let this = unsafe { &*(surface as *const Surface as *const Self) };

        let result = surface
            .canvas
            .connect_draw(move |widget, cr, width, height| {
                let ctx = animate::Canvas::new(cr);
                f(this, &ctx, width, height);
                false
            });

        // First redraw
        surface.canvas.invalidate();

        result
    }

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn looped_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Surface,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Surface>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Surface::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"looped\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             looped_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_animating_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Surface,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Surface>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Surface::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::animating\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_animating_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Surface")
    }
}
