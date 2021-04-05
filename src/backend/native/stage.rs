#![allow(unused_variables)]

use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use super::{Toolbar, WindowRotation};
use crate::prelude::*;

use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Stage {
    inner: clutter::Stage,
}

impl Stage {
    pub fn new() -> Stage {
        Self {
            inner: clutter::Stage::new(),
        }
    }

    pub fn with_clutter_stage(stage: &clutter::Stage) -> Stage {
        //    unsafe { TODO: call ffi:window_new_with_clutter_stage() }
        unimplemented!()
    }

    pub fn get_for_stage(stage: &clutter::Stage) -> Option<Stage> {
        //    unsafe { TODO: call ffi:window_get_for_stage() }
        unimplemented!()
    }

    pub fn test_check(&self) -> String {
        "HERE".into()
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            inner: clutter::Stage::new(),
        }
    }
}

impl Object for Stage {}
impl Is<Stage> for Stage {}

impl AsRef<Stage> for Stage {
    fn as_ref(&self) -> &Stage {
        self
    }
}

pub const NONE_WINDOW: Option<&Stage> = None;

pub trait WindowExt: 'static {
    fn get_child(&self) -> Option<clutter::Actor>;

    fn get_clutter_stage(&self) -> Option<clutter::Stage>;

    fn get_fullscreen(&self) -> bool;

    fn get_has_toolbar(&self) -> bool;

    fn get_icon_name(&self) -> Option<String>;

    fn get_resisable(&self) -> bool;

    fn get_small_screen(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    fn get_toolbar(&self) -> Option<Toolbar>;

    fn get_window_position(&self) -> (i32, i32);

    fn get_window_rotation(&self) -> WindowRotation;

    fn get_window_size(&self) -> (i32, i32);

    fn hide(&self) -> &Self;

    fn present(&self);

    fn set_child<P: Is<clutter::Actor>>(&self, actor: &P);

    fn set_fullscreen(&self, fullscreen: bool) -> &Self;

    fn set_has_toolbar(&self, toolbar: bool);

    //fn set_icon_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle);

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_resizable(&self, resizable: bool) -> &Self;

    fn set_small_screen(&self, small_screen: bool);

    fn set_title(&self, title: &str) -> &Self;

    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P);

    fn set_window_position(&self, x: i32, y: i32);

    fn set_window_rotation(&self, rotation: WindowRotation);

    fn set_window_size(&self, width: i32, height: i32) -> &Self;

    fn show(&self) -> &Self;

    fn get_property_icon_cogl_texture(&self) -> Option<String>;

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>);

    fn get_property_window_rotation_angle(&self) -> f32;

    fn get_property_window_rotation_timeline(&self) -> Option<clutter::Timeline>;

    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<Stage>> WindowExt for O {
    fn get_child(&self) -> Option<clutter::Actor> {
        // unsafe { from_glib_none(ffi::window_get_child(self.as_ref().to_glib_none().0)) }
        let win = self.as_ref();
        unimplemented!()
    }

    fn get_clutter_stage(&self) -> Option<clutter::Stage> {
        //    unsafe { TODO: call ffi:window_get_clutter_stage() }
        unimplemented!()
    }

    fn get_fullscreen(&self) -> bool {
        let inner = &self.as_ref().inner;
        inner.get_fullscreen()
    }

    fn get_has_toolbar(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::window_get_has_toolbar(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_icon_name(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::window_get_icon_name(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_resisable(&self) -> bool {
        let inner = &self.as_ref().inner;
        inner.get_user_resizable()
    }

    fn get_small_screen(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::window_get_small_screen(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_title(&self) -> Option<String> {
        let inner = &self.as_ref().inner;
        match inner.get_title() {
            Some(title) => Some(title.as_str().into()),
            None => None,
        }
    }

    fn get_toolbar(&self) -> Option<Toolbar> {
        // unsafe {
        //     from_glib_none(ffi::window_get_toolbar(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_window_position(&self) -> (i32, i32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     ffi::window_get_window_position(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     (x, y)
        // }
        unimplemented!()
    }

    fn get_window_rotation(&self) -> WindowRotation {
        //    unsafe { TODO: call ffi:window_get_window_rotation() }
        unimplemented!()
    }

    fn get_window_size(&self) -> (i32, i32) {
        let inner = &self.as_ref().inner;
        let (width, height) = inner.get_size();
        (width as i32, height as i32)
    }

    fn hide(&self) -> &Self {
        let inner = &self.as_ref().inner;
        inner.hide();
        self
    }

    fn present(&self) {
        // unsafe {
        //     ffi::window_present(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_child<P: Is<clutter::Actor>>(&self, actor: &P) {
        // unsafe {
        //     ffi::window_set_child(
        //         self.as_ref().to_glib_none().0,
        //         actor.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_fullscreen(&self, fullscreen: bool) -> &Self {
        let inner = &self.as_ref().inner;
        inner.set_fullscreen(fullscreen);
        self
    }

    fn set_has_toolbar(&self, toolbar: bool) {
        // unsafe {
        //     ffi::window_set_has_toolbar(self.as_ref().to_glib_none().0, toolbar.to_glib());
        // }
        unimplemented!()
    }

    //fn set_icon_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:window_set_icon_from_cogl_texture() }
    //}

    fn set_icon_name(&self, icon_name: Option<&str>) {
        // unsafe {
        //     ffi::window_set_icon_name(
        //         self.as_ref().to_glib_none().0,
        //         icon_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_resizable(&self, resizable: bool) -> &Self {
        let inner = &self.as_ref().inner;
        inner.set_user_resizable(resizable);
        self
    }

    fn set_small_screen(&self, small_screen: bool) {
        // unsafe {
        //     ffi::window_set_small_screen(
        //         self.as_ref().to_glib_none().0,
        //         small_screen.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_title(&self, title: &str) -> &Self {
        let inner = &self.as_ref().inner;
        inner.set_title(title);
        self
    }

    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P) {
        // unsafe {
        //     ffi::window_set_toolbar(
        //         self.as_ref().to_glib_none().0,
        //         toolbar.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_window_position(&self, x: i32, y: i32) {
        // unsafe {
        //     ffi::window_set_window_position(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn set_window_rotation(&self, rotation: WindowRotation) {
        //    unsafe { TODO: call ffi:window_set_window_rotation() }
        unimplemented!()
    }

    fn set_window_size(&self, width: i32, height: i32) -> &Self {
        let inner = &self.as_ref().inner;
        inner.set_size(width as f32, height as f32);
        self
    }

    fn show(&self) -> &Self {
        let inner = &self.as_ref().inner;
        inner.show();
        self
    }

    fn get_property_icon_cogl_texture(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `icon-cogl-texture` getter")
        // }
        unimplemented!()
    }

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         Value::from(icon_cogl_texture).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_angle(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-angle\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-angle` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_timeline(&self) -> Option<clutter::Timeline> {
        // unsafe {
        //     let mut value = Value::from_type(<clutter::Timeline as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-timeline\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-timeline` getter")
        // }
        unimplemented!()
    }

    // unsafe fn unsafe_cast_ref<T: ObjectType>(&self) -> &T {
    //     debug_assert!(self.is::<T>());
    //     // This cast is safe because all our wrapper types have the
    //     // same representation except for the name and the phantom data
    //     // type. IsA<> is an unsafe trait that must only be implemented
    //     // if this is a valid wrapper type
    //     &*(self as *const Self as *const T)
    // }

    // TODO: &Self
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        let win = self.as_ref();
        let this = unsafe { &*(win as *const Stage as *const Self) };

        win.inner.connect_destroy(move |_| {
            f(this);
        })
    }

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::child\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_child_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_fullscreen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fullscreen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fullscreen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_has_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::has-toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_has_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_cogl_texture_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-cogl-texture\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_cogl_texture_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_small_screen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::small-screen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_small_screen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::title\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_title_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_angle_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-angle\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_angle_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_timeline_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-timeline\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_timeline_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window")
    }
}
