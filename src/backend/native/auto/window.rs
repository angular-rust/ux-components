use crate::prelude::*;
// use glib::object::Cast;
// use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib::StaticType;
// use glib::Value;



// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem;
// use std::mem::transmute;
use crate::native::Toolbar;

// glib_wrapper! {
//     pub struct Window(Object<ffi::Window, ffi::WindowClass, WindowClass>);

//     match fn {
//         get_type => || ffi::window_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct Window {}

impl Window {
    pub fn new() -> Window {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_full(ffi::window_new()) }
        unimplemented!()
    }

    pub fn with_clutter_stage(stage: /*Ignored*/&clutter::Stage) -> Window {
    //    unsafe { TODO: call ffi:mx_window_new_with_clutter_stage() }
       unimplemented!()
    }

    pub fn get_for_stage(stage: &clutter::Stage) -> Option<Window> {
    //    unsafe { TODO: call ffi:mx_window_get_for_stage() }
        unimplemented!()
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_WINDOW: Option<&Window> = None;

pub trait WindowExt: 'static {
    fn get_child(&self) -> Option<clutter::Actor>;

    fn get_clutter_stage(&self) -> Option<clutter::Stage>;

    fn get_fullscreen(&self) -> bool;

    fn get_has_toolbar(&self) -> bool;

    fn get_icon_name(&self) -> Option<String>;

    fn get_small_screen(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    fn get_toolbar(&self) -> Option<Toolbar>;

    fn get_window_position(&self) -> (i32, i32);

    //fn get_window_rotation(&self) -> /*Ignored*/WindowRotation;

    fn get_window_size(&self) -> (i32, i32);

    fn hide(&self);

    fn present(&self);

    fn set_child<P: Is<clutter::Actor>>(&self, actor: &P);

    fn set_fullscreen(&self, fullscreen: bool);

    fn set_has_toolbar(&self, toolbar: bool);

    //fn set_icon_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle);

    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_small_screen(&self, small_screen: bool);

    fn set_title(&self, title: &str);

    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P);

    fn set_window_position(&self, x: i32, y: i32);

    //fn set_window_rotation(&self, rotation: /*Ignored*/WindowRotation);

    fn set_window_size(&self, width: i32, height: i32);

    fn show(&self);

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

// impl<O: IsA<Window>> WindowExt for O {
//     fn get_child(&self) -> Option<clutter::Actor> {
//         unsafe { from_glib_none(ffi::window_get_child(self.as_ref().to_glib_none().0)) }
//     }

//     //fn get_clutter_stage(&self) -> /*Ignored*/Option<clutter::Stage> {
//     //    unsafe { TODO: call ffi:mx_window_get_clutter_stage() }
//     //}

//     fn get_fullscreen(&self) -> bool {
//         unsafe {
//             from_glib(ffi::window_get_fullscreen(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_has_toolbar(&self) -> bool {
//         unsafe {
//             from_glib(ffi::window_get_has_toolbar(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_icon_name(&self) -> Option<String> {
//         unsafe {
//             from_glib_none(ffi::window_get_icon_name(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_small_screen(&self) -> bool {
//         unsafe {
//             from_glib(ffi::window_get_small_screen(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_title(&self) -> Option<String> {
//         unsafe { from_glib_none(ffi::window_get_title(self.as_ref().to_glib_none().0)) }
//     }

//     fn get_toolbar(&self) -> Option<Toolbar> {
//         unsafe {
//             from_glib_none(ffi::window_get_toolbar(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_window_position(&self) -> (i32, i32) {
//         unsafe {
//             let mut x = mem::MaybeUninit::uninit();
//             let mut y = mem::MaybeUninit::uninit();
//             ffi::window_get_window_position(
//                 self.as_ref().to_glib_none().0,
//                 x.as_mut_ptr(),
//                 y.as_mut_ptr(),
//             );
//             let x = x.assume_init();
//             let y = y.assume_init();
//             (x, y)
//         }
//     }

//     //fn get_window_rotation(&self) -> /*Ignored*/WindowRotation {
//     //    unsafe { TODO: call ffi:mx_window_get_window_rotation() }
//     //}

//     fn get_window_size(&self) -> (i32, i32) {
//         unsafe {
//             let mut width = mem::MaybeUninit::uninit();
//             let mut height = mem::MaybeUninit::uninit();
//             ffi::window_get_window_size(
//                 self.as_ref().to_glib_none().0,
//                 width.as_mut_ptr(),
//                 height.as_mut_ptr(),
//             );
//             let width = width.assume_init();
//             let height = height.assume_init();
//             (width, height)
//         }
//     }

//     fn hide(&self) {
//         unsafe {
//             ffi::window_hide(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn present(&self) {
//         unsafe {
//             ffi::window_present(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn set_child<P: IsA<clutter::Actor>>(&self, actor: &P) {
//         unsafe {
//             ffi::window_set_child(
//                 self.as_ref().to_glib_none().0,
//                 actor.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn set_fullscreen(&self, fullscreen: bool) {
//         unsafe {
//             ffi::window_set_fullscreen(self.as_ref().to_glib_none().0, fullscreen.to_glib());
//         }
//     }

//     fn set_has_toolbar(&self, toolbar: bool) {
//         unsafe {
//             ffi::window_set_has_toolbar(self.as_ref().to_glib_none().0, toolbar.to_glib());
//         }
//     }

//     //fn set_icon_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle) {
//     //    unsafe { TODO: call ffi:mx_window_set_icon_from_cogl_texture() }
//     //}

//     fn set_icon_name(&self, icon_name: Option<&str>) {
//         unsafe {
//             ffi::window_set_icon_name(
//                 self.as_ref().to_glib_none().0,
//                 icon_name.to_glib_none().0,
//             );
//         }
//     }

//     fn set_small_screen(&self, small_screen: bool) {
//         unsafe {
//             ffi::window_set_small_screen(
//                 self.as_ref().to_glib_none().0,
//                 small_screen.to_glib(),
//             );
//         }
//     }

//     fn set_title(&self, title: &str) {
//         unsafe {
//             ffi::window_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
//         }
//     }

//     fn set_toolbar<P: IsA<Toolbar>>(&self, toolbar: &P) {
//         unsafe {
//             ffi::window_set_toolbar(
//                 self.as_ref().to_glib_none().0,
//                 toolbar.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn set_window_position(&self, x: i32, y: i32) {
//         unsafe {
//             ffi::window_set_window_position(self.as_ref().to_glib_none().0, x, y);
//         }
//     }

//     //fn set_window_rotation(&self, rotation: /*Ignored*/WindowRotation) {
//     //    unsafe { TODO: call ffi:mx_window_set_window_rotation() }
//     //}

//     fn set_window_size(&self, width: i32, height: i32) {
//         unsafe {
//             ffi::window_set_window_size(self.as_ref().to_glib_none().0, width, height);
//         }
//     }

//     fn show(&self) {
//         unsafe {
//             ffi::window_show(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn get_property_icon_cogl_texture(&self) -> Option<String> {
//         unsafe {
//             let mut value = Value::from_type(<String as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"icon-cogl-texture\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `icon-cogl-texture` getter")
//         }
//     }

//     fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"icon-cogl-texture\0".as_ptr() as *const _,
//                 Value::from(icon_cogl_texture).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_window_rotation_angle(&self) -> f32 {
//         unsafe {
//             let mut value = Value::from_type(<f32 as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"window-rotation-angle\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `window-rotation-angle` getter")
//                 .unwrap()
//         }
//     }

//     fn get_property_window_rotation_timeline(&self) -> Option<clutter::Timeline> {
//         unsafe {
//             let mut value = Value::from_type(<clutter::Timeline as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"window-rotation-timeline\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `window-rotation-timeline` getter")
//         }
//     }

//     fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn destroy_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"destroy\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     destroy_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::child\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_child_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_fullscreen_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::fullscreen\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_fullscreen_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_has_toolbar_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::has-toolbar\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_has_toolbar_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_cogl_texture_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-cogl-texture\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_cogl_texture_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-name\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_name_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_small_screen_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::small-screen\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_small_screen_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::title\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_title_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_toolbar_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::toolbar\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_toolbar_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_window_rotation_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::window-rotation\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_window_rotation_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_window_rotation_angle_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::window-rotation-angle\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_window_rotation_angle_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_window_rotation_timeline_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Window,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Window>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Window::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::window-rotation-timeline\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_window_rotation_timeline_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window")
    }
}
