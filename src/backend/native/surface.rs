#![allow(unused_variables)]

use crate::prelude::*;
use crate::{Actor, ActorCanvas, Canvas, ScalingFilter, Widget};
use glib::signal::SignalHandlerId;
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
    inner: Widget,
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

        let inner = Widget::new();
        let content = ActorCanvas::new().unwrap();
        let canvas: ActorCanvas = unsafe { mem::transmute(content) };

        // set the default surface size similar HTML5 Canvas
        canvas.set_size(300, 150);

        {
            let actor: &Actor = inner.as_ref();

            // set the default surface Actor size similar HTML5 Canvas
            actor.set_size(300_f32, 150_f32);
            actor.set_content(Some(&canvas));
            actor.set_content_scaling_filters(ScalingFilter::Trilinear, ScalingFilter::Linear);
        }

        let component = Self {
            props: RefCell::new(props),
            canvas,
            inner,
        };

        component
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
        &self.inner
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
        let actor: &Actor = self.inner.as_ref();
        actor
    }
}

pub trait SurfaceExt: 'static {
    // /// get_animating:
    // /// @spinner: A #Surface widget
    // ///
    // /// Determines whether the spinner is animating.
    // ///
    // /// Returns: %true if the spinner is animating, %false otherwise
    // ///
    // fn get_animating(&self) -> bool;

    // /// set_animating:
    // /// @spinner: A #Surface widget
    // /// @animating: %true to enable animation, %false to disable
    // ///
    // /// Sets whether the spinner is animating. A spinner can be stopped if
    // /// the task it represents has finished, or to save energy.
    // ///
    // fn set_animating(&self, animating: bool);

    /// Invalidates a `Content`.
    ///
    /// This function should be called by `Content` implementations when
    /// they change the way a the content should be painted regardless of the
    /// actor state.
    fn invalidate(&self);

    /// Sets the size of the surface context, and invalidates the content.
    ///
    /// This function will cause the `self` to be invalidated only
    /// if the size of the canvas surface has changed.
    ///
    /// If you want to invalidate the contents of the `self` when setting
    /// the size, you can use the return value of the function to conditionally
    /// call `Content::invalidate`:
    ///
    ///
    /// ```text
    ///   if (!canvas_set_size (canvas, width, height))
    ///     content_invalidate (CONTENT (canvas));
    /// ```
    /// ## `width`
    /// the width of the canvas, in pixels
    /// ## `height`
    /// the height of the canvas, in pixels
    ///
    /// # Returns
    ///
    /// this function returns `true` if the size change
    ///  caused a content invalidation, and `false` otherwise
    fn set_content_size(&self, width: f64, height: f64) -> bool;

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
    fn connect_draw<F: Fn(&Self, &Canvas, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_looped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_animating_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Surface>> SurfaceExt for O {
    // /// get_animating:
    // /// @spinner: A #Surface widget
    // ///
    // /// Determines whether the spinner is animating.
    // ///
    // /// Returns: %true if the spinner is animating, %false otherwise
    // ///
    // fn get_animating(&self) -> bool {
    //     let spinner = self.as_ref();
    //     let props = spinner.props.borrow();
    //     props.animating
    // }

    // /// set_animating:
    // /// @spinner: A #Surface widget
    // /// @animating: %true to enable animation, %false to disable
    // ///
    // /// Sets whether the spinner is animating. A spinner can be stopped if
    // /// the task it represents has finished, or to save energy.
    // ///
    // fn set_animating(&self, animating: bool) {
    //     let spinner = self.as_ref();
    //     let mut props = spinner.props.borrow_mut();

    //     if props.animating != animating {
    //         props.animating = animating;
    //         // update_timeout(spinner);
    //         // g_object_notify(G_OBJECT(spinner), "animating");
    //     }
    // }

    /// Invalidates a `Content`.
    ///
    /// This function should be called by `Content` implementations when
    /// they change the way a the content should be painted regardless of the
    /// actor state.
    fn invalidate(&self) {
        let surface = self.as_ref();
        surface.canvas.invalidate()
    }
    /// Sets the size of the surface context, and invalidates the content.
    ///
    /// This function will cause the `self` to be invalidated only
    /// if the size of the canvas surface has changed.
    ///
    /// If you want to invalidate the contents of the `self` when setting
    /// the size, you can use the return value of the function to conditionally
    /// call `Content::invalidate`:
    ///
    ///
    /// ```text
    ///   if (!canvas_set_size (canvas, width, height))
    ///     content_invalidate (CONTENT (canvas));
    /// ```
    /// ## `width`
    /// the width of the canvas, in pixels
    /// ## `height`
    /// the height of the canvas, in pixels
    ///
    /// # Returns
    ///
    /// this function returns `true` if the size change
    ///  caused a content invalidation, and `false` otherwise
    fn set_content_size(&self, width: f64, height: f64) -> bool {
        let surface = self.as_ref();
        surface.canvas.set_size(width as i32, height as i32)
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
    fn connect_draw<F: Fn(&Self, &Canvas, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        let surface = self.as_ref();

        let this = unsafe { &*(surface as *const Surface as *const Self) };

        let result = surface
            .canvas
            .connect_draw(move |widget, cr, width, height| {
                let ctx = animate::Canvas::new(cr);
                f(this, &ctx, width, height)
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
