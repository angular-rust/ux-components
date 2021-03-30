// use std::boxed::Box as Box_;
// use std::mem::transmute;
// use std::ptr;

use super::Widget;
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends Widget, clutter::Actor;
#[derive(Clone, Debug)]
pub struct Image {}

impl Image {
    pub fn new() -> Image {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::image_new()).unsafe_cast() }
        unimplemented!()
    }

    // pub fn new() -> Image {
    //     unimplemented!() // TODO: complete it

    //     // assert_initialized_main_thread!();
    //     // unsafe { from_glib_full(ffi::image_new()) }
    // }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_IMAGE: Option<&Image> = None;

pub trait ImageExt: 'static {
    //fn animate_scale_mode(&self, mode: libc::c_ulong, duration: u32, scale_mode: /*Ignored*/ImageScaleMode);

    fn clear(&self);

    fn get_allow_upscale(&self) -> bool;

    fn get_image_rotation(&self) -> f32;

    fn get_load_async(&self) -> bool;

    fn get_scale_height_threshold(&self) -> u32;

    //fn get_scale_mode(&self) -> /*Ignored*/ImageScaleMode;

    fn get_scale_width_threshold(&self) -> u32;

    fn get_transition_duration(&self) -> u32;

    fn set_allow_upscale(&self, allow: bool);

    //fn set_from_buffer(&self, buffer: &[u8]) -> Result<(), glib::Error>;

    //fn set_from_buffer_at_size(&self, buffer: &[u8], width: i32, height: i32) -> Result<(), glib::Error>;

    //fn set_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle) -> bool;

    //fn set_from_data(&self, data: &[u8], pixel_format: /*Ignored*/cogl::PixelFormat, width: i32, height: i32, rowstride: i32) -> Result<(), glib::Error>;

    fn set_from_file(&self, filename: &str) -> Result<(), glib::Error>;

    fn set_from_file_at_size(
        &self,
        filename: &str,
        width: i32,
        height: i32,
    ) -> Result<(), glib::Error>;

    fn set_image_rotation(&self, rotation: f32);

    fn set_load_async(&self, load_async: bool);

    fn set_scale_height_threshold(&self, pixels: u32);

    //fn set_scale_mode(&self, mode: /*Ignored*/ImageScaleMode);

    fn set_scale_width_threshold(&self, pixels: u32);

    fn set_transition_duration(&self, duration: u32);

    fn set_property_filename(&self, filename: Option<&str>);

    fn connect_image_load_error<F: Fn(&Self, &glib::Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_image_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_allow_upscale_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_image_rotation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_load_async_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_height_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_scale_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_width_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

// impl<O: Is<Image>> ImageExt for O {
//     //fn animate_scale_mode(&self, mode: libc::c_ulong, duration: u32, scale_mode: /*Ignored*/ImageScaleMode) {
//     //    unsafe { TODO: call ffi:image_animate_scale_mode() }
//     //}

//     fn clear(&self) {
//         unsafe {
//             ffi::image_clear(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn get_allow_upscale(&self) -> bool {
//         unsafe {
//             from_glib(ffi::image_get_allow_upscale(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_image_rotation(&self) -> f32 {
//         unsafe { ffi::image_get_image_rotation(self.as_ref().to_glib_none().0) }
//     }

//     fn get_load_async(&self) -> bool {
//         unsafe {
//             from_glib(ffi::image_get_load_async(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_scale_height_threshold(&self) -> u32 {
//         unsafe { ffi::image_get_scale_height_threshold(self.as_ref().to_glib_none().0) }
//     }

//     //fn get_scale_mode(&self) -> /*Ignored*/ImageScaleMode {
//     //    unsafe { TODO: call ffi:image_get_scale_mode() }
//     //}

//     fn get_scale_width_threshold(&self) -> u32 {
//         unsafe { ffi::image_get_scale_width_threshold(self.as_ref().to_glib_none().0) }
//     }

//     fn get_transition_duration(&self) -> u32 {
//         unsafe { ffi::image_get_transition_duration(self.as_ref().to_glib_none().0) }
//     }

//     fn set_allow_upscale(&self, allow: bool) {
//         unsafe {
//             ffi::image_set_allow_upscale(self.as_ref().to_glib_none().0, allow.to_glib());
//         }
//     }

//     //fn set_from_buffer(&self, buffer: &[u8]) -> Result<(), glib::Error> {
//     //    unsafe { TODO: call ffi:image_set_from_buffer() }
//     //}

//     //fn set_from_buffer_at_size(&self, buffer: &[u8], width: i32, height: i32) -> Result<(), glib::Error> {
//     //    unsafe { TODO: call ffi:image_set_from_buffer_at_size() }
//     //}

//     //fn set_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle) -> bool {
//     //    unsafe { TODO: call ffi:image_set_from_cogl_texture() }
//     //}

//     //fn set_from_data(&self, data: &[u8], pixel_format: /*Ignored*/cogl::PixelFormat, width: i32, height: i32, rowstride: i32) -> Result<(), glib::Error> {
//     //    unsafe { TODO: call ffi:image_set_from_data() }
//     //}

//     fn set_from_file(&self, filename: &str) -> Result<(), glib::Error> {
//         unsafe {
//             let mut error = ptr::null_mut();
//             let _ = ffi::image_set_from_file(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//                 &mut error,
//             );
//             if error.is_null() {
//                 Ok(())
//             } else {
//                 Err(from_glib_full(error))
//             }
//         }
//     }

//     fn set_from_file_at_size(
//         &self,
//         filename: &str,
//         width: i32,
//         height: i32,
//     ) -> Result<(), glib::Error> {
//         unsafe {
//             let mut error = ptr::null_mut();
//             let _ = ffi::image_set_from_file_at_size(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//                 width,
//                 height,
//                 &mut error,
//             );
//             if error.is_null() {
//                 Ok(())
//             } else {
//                 Err(from_glib_full(error))
//             }
//         }
//     }

//     fn set_image_rotation(&self, rotation: f32) {
//         unsafe {
//             ffi::image_set_image_rotation(self.as_ref().to_glib_none().0, rotation);
//         }
//     }

//     fn set_load_async(&self, load_async: bool) {
//         unsafe {
//             ffi::image_set_load_async(self.as_ref().to_glib_none().0, load_async.to_glib());
//         }
//     }

//     fn set_scale_height_threshold(&self, pixels: u32) {
//         unsafe {
//             ffi::image_set_scale_height_threshold(self.as_ref().to_glib_none().0, pixels);
//         }
//     }

//     //fn set_scale_mode(&self, mode: /*Ignored*/ImageScaleMode) {
//     //    unsafe { TODO: call ffi:image_set_scale_mode() }
//     //}

//     fn set_scale_width_threshold(&self, pixels: u32) {
//         unsafe {
//             ffi::image_set_scale_width_threshold(self.as_ref().to_glib_none().0, pixels);
//         }
//     }

//     fn set_transition_duration(&self, duration: u32) {
//         unsafe {
//             ffi::image_set_transition_duration(self.as_ref().to_glib_none().0, duration);
//         }
//     }

//     fn set_property_filename(&self, filename: Option<&str>) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"filename\0".as_ptr() as *const _,
//                 Value::from(filename).to_glib_none().0,
//             );
//         }
//     }

//     fn connect_image_load_error<F: Fn(&Self, &glib::Error) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn image_load_error_trampoline<P, F: Fn(&P, &glib::Error) + 'static>(
//             this: *mut ffi::Image,
//             object: *mut glib_sys::GError,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(
//                 &Image::from_glib_borrow(this).unsafe_cast_ref(),
//                 &from_glib_borrow(object),
//             )
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"image-load-error\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     image_load_error_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_image_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn image_loaded_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"image-loaded\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     image_loaded_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_allow_upscale_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_allow_upscale_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::allow-upscale\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_allow_upscale_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_filename_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::filename\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_filename_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_image_rotation_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_image_rotation_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::image-rotation\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_image_rotation_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_load_async_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_load_async_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::load-async\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_load_async_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_scale_height_threshold_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_scale_height_threshold_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::scale-height-threshold\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_scale_height_threshold_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_scale_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_scale_mode_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::scale-mode\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_scale_mode_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_scale_width_threshold_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_scale_width_threshold_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::scale-width-threshold\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_scale_width_threshold_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_transition_duration_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::Image,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Image>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Image::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::transition-duration\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_transition_duration_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
