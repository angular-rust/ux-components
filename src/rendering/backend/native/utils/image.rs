#![allow(unused_variables)]

use crate::prelude::*;
use crate::{platform::core::{Handle, Material}, Actor, HandlerId, ImageScaleMode, Timeline, Widget};
use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct ImageAsyncData {
    // pub parent: Image,
    // pub mutex: GMutex,
    pub complete: bool,
    pub cancelled: bool,
    pub upscale: bool,
    pub idle_handler: i32,

    pub filename: String,
    pub buffer: Vec<u8>,
    pub count: usize,
    // pub free_func: GDestroyNotify,
    pub width: i32,
    pub height: i32,
    pub width_threshold: u32,
    pub height_threshold: u32,
    // pub pixbuf: GdkPixbuf,
    // pub error: GError,
}

#[derive(Debug)]
pub struct ImageProps {
    pub mode: ImageScaleMode,
    pub previous_mode: ImageScaleMode,
    pub load_async: bool,
    pub upscale: bool,
    pub width_threshold: u32,
    pub height_threshold: u32,
    pub texture: Handle,
    pub old_texture: Handle,
    pub blank_texture: Handle,
    pub rotation: f32,
    pub old_rotation: f32,
    pub old_mode: ImageScaleMode,
    pub template_material: dx::core::Material,
    pub material: dx::core::Material,
    pub timeline: Timeline,
    pub redraw_timeline: Timeline,
    pub transition_duration: u32,
    pub async_load_data: Option<ImageAsyncData>,
}

#[derive(Debug)]
pub struct Image {
    props: RefCell<ImageProps>,
}

impl Image {
    pub fn new() -> Image {
        // assert_initialized_main_thread!();
        // unsafe { Actor::from_glib_none(ffi::image_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for Image {}
impl Is<Image> for Image {}

impl AsRef<Image> for Image {
    fn as_ref(&self) -> &Image {
        self
    }
}

impl Is<Widget> for Image {}

impl AsRef<Widget> for Image {
    fn as_ref(&self) -> &Widget {
        // &self.widget
        unimplemented!()
    }
}

impl Is<Actor> for Image {}

impl AsRef<Actor> for Image {
    fn as_ref(&self) -> &Actor {
        // &self.widget
        unimplemented!()
    }
}

pub trait ImageExt: 'static {
    /// animate_scale_mode:
    /// @image: An #Image
    /// @mode: a #AnimationMode
    /// @duration: duration of the animation in milliseconds
    /// @scale_mode: The #ImageScaleMode to set
    ///
    /// Sets the value of #Image:scale-mode to @scale_mode and animates the
    /// scale factor of the image between the previous value and the new value.
    ///
    fn animate_scale_mode(&self, mode: u64, duration: u32, scale_mode: ImageScaleMode);

    /// clear:
    /// @image: A #Image
    ///
    /// Clear the current image and set a blank, transparent image.
    ///
    /// Returns: static void
    ///
    fn clear(&self);

    /// get_allow_upscale:
    /// @image: A #Image
    ///
    /// Determines whether image up-scaling is allowed.
    ///
    /// Returns: %true if upscaling is allowed, %false otherwise
    ///
    fn get_allow_upscale(&self) -> bool;

    /// get_image_rotation:
    /// @image: A #Image
    ///
    /// Get the value of the Image:image-rotation property.
    ///
    /// Returns: The value of the image-rotation property.
    ///
    fn get_image_rotation(&self) -> f32;

    /// get_load_async:
    /// @image: A #Image
    ///
    /// Determines whether asynchronous image loading is in use.
    ///
    /// Returns: %true if images are set to load asynchronously, %false otherwise
    ///
    fn get_load_async(&self) -> bool;

    /// get_scale_height_threshold:
    /// @image: A #Image
    ///
    /// Retrieves the height scaling threshold.
    ///
    /// Returns: The height scaling threshold, in pixels
    ///
    fn get_scale_height_threshold(&self) -> u32;

    /// get_scale_mode:
    /// @image: An #Image
    ///
    /// Get the current scale mode of @Image.
    ///
    /// Returns: The current ImageScaleMode
    ///
    fn get_scale_mode(&self) -> ImageScaleMode;

    /// get_scale_width_threshold:
    /// @image: A #Image
    ///
    /// Retrieves the width scaling threshold.
    ///
    /// Returns: The width scaling threshold, in pixels
    ///
    fn get_scale_width_threshold(&self) -> u32;

    /// get_transition_duration:
    /// @image: A #Image
    ///
    /// Get the value of the Image:transition-duration property.
    ///
    /// Returns: The value of the transition-duration property.
    ///
    fn get_transition_duration(&self) -> u32;

    /// set_allow_upscale:
    /// @image: A #Image
    /// @allow: %true to allow upscaling, %false otherwise
    ///
    /// Sets whether up-scaling of images is allowed. If set to %true and a size
    /// larger than the image is requested, the image will be up-scaled in
    /// software.
    ///
    /// The advantage of this is that software up-scaling is potentially higher
    /// quality, but it comes at the expense of video memory.
    ///
    fn set_allow_upscale(&self, allow: bool);

    /// set_from_buffer:
    /// @image: An #Image
    /// @buffer: (array length=buffer_size) (transfer full): A buffer
    ///   pointing to encoded image data
    /// @buffer_size: The size of @buffer, in bytes
    /// @buffer_free_func: (allow-none): A function to free @buffer, or %None
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from unencoded image data, stored in memory. In case of
    /// failure, #false is returned and @error is set. It is expected that @buffer
    /// will remain accessible for the duration of the load. Once it is finished
    /// with, @buffer_free_func will be called.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_buffer(&self, buffer: &[u8]) -> Result<(), glib::Error>;

    /// set_from_buffer_at_size:
    /// @image: An #Image
    /// @buffer: (array length=buffer_size) (transfer full): A buffer
    ///   pointing to encoded image data
    /// @buffer_size: The size of @buffer, in bytes
    /// @buffer_free_func: (allow-none): A function to free @buffer, or %None
    /// @width: Width to scale the image to, or -1
    /// @height: Height to scale the image to, or -1
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from unencoded image data, stored in memory, and scales
    /// it while loading. In case of failure, #false is returned and @error is set.
    /// It is expected that @buffer will remain accessible for the duration of the
    /// load. Once it is finished with, @buffer_free_func will be called. The aspect
    /// ratio will always be maintained.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_buffer_at_size(
    //     &self,
    //     buffer: &[u8],
    //     width: i32,
    //     height: i32,
    // ) -> Result<(), glib::Error>;

    /// set_from_cogl_texture:
    /// @image: A #Image
    /// @texture: A #CoglHandle to a texture
    ///
    /// Sets the contents of the image from the given Cogl texture.
    ///
    /// Returns: %true on success, %false on failure
    ///
    fn set_from_cogl_texture(&self, texture: Handle) -> bool;

    /// set_from_data:
    /// @image: An #Image
    /// @data: (array): Image data
    /// @pixel_format: The #CoglPixelFormat of the buffer
    /// @width: Width in pixels of image data.
    /// @height: Height in pixels of image data
    /// @rowstride: Distance in bytes between row starts.
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from a buffer. In case of failure, #false is returned
    /// and @error is set.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_data(
    //     &self,
    //     data: &[u8],
    //     pixel_format: dx::PixelFormat,
    //     width: i32,
    //     height: i32,
    //     rowstride: i32,
    // ) -> Result<(), glib::Error>;

    /// set_from_file:
    /// @image: An #Image
    /// @filename: Filename to read the file from
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from an image file. In case of failure, #false is returned
    /// and @error is set.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_file(&self, filename: &str) -> Result<(), glib::Error>;

    /// set_from_file_at_size:
    /// @image: An #Image
    /// @filename: Filename to read the file from
    /// @width: Width to scale the image to, or -1
    /// @height: Height to scale the image to, or -1
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from an image file, and scale the image during loading.
    /// In case of failure, #false is returned and @error is set. The aspect ratio
    /// will always be maintained.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_file_at_size(
    //     &self,
    //     filename: &str,
    //     width: i32,
    //     height: i32,
    // ) -> Result<(), glib::Error>;

    /// set_image_rotation:
    /// @image: A #Image
    /// @rotation: Rotation angle in degrees
    ///
    /// Set the Image:image-rotation property.
    ///
    fn set_image_rotation(&self, rotation: f32);

    /// set_load_async:
    /// @image: A #Image
    /// @load_async: %true to load images asynchronously
    ///
    /// Sets whether to load images asynchronously. Asynchronous image loading
    /// requires thread support (see g_thread_init()).
    ///
    /// When using asynchronous image loading, all image-loading functions will
    /// return immediately as successful. The #Image::image-loaded and
    /// #Image::image-load-error signals are used to signal success or failure
    /// of asynchronous image loading.
    ///
    fn set_load_async(&self, load_async: bool);

    /// set_scale_height_threshold:
    /// @image: A #Image
    /// @pixels: Number of pixels
    ///
    /// Sets the threshold used to determine whether to scale the height of the
    /// image. If a specific height is requested, the image height is allowed to
    /// differ by this amount before scaling is employed.
    ///
    /// This can be useful to avoid excessive CPU usage when the image differs
    /// only slightly to the desired size.
    ///
    fn set_scale_height_threshold(&self, pixels: u32);

    /// set_scale_mode:
    /// @image: An #Image
    /// @mode: The #ImageScaleMode to set
    ///
    /// Set the scale mode on @Image
    ///
    fn set_scale_mode(&self, mode: ImageScaleMode);

    /// set_scale_width_threshold:
    /// @image: A #Image
    /// @pixels: Number of pixels
    ///
    /// Sets the threshold used to determine whether to scale the width of the
    /// image. If a specific width is requested, the image width is allowed to
    /// differ by this amount before scaling is employed.
    ///
    /// This can be useful to avoid excessive CPU usage when the image differs
    /// only slightly to the desired size.
    ///
    fn set_scale_width_threshold(&self, pixels: u32);

    /// set_transition_duration:
    /// @image: A #Image
    /// @duration: Transition duration in milliseconds
    ///
    /// Set the Image:transition-duration property.
    ///
    fn set_transition_duration(&self, duration: u32);

    fn set_property_filename(&self, filename: Option<&str>);

    // fn connect_image_load_error<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> HandlerId;

    fn connect_image_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_allow_upscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_image_rotation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_load_async_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_scale_height_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_scale_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_scale_width_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;
}

impl<O: Is<Image>> ImageExt for O {
    /// animate_scale_mode:
    /// @image: An #Image
    /// @mode: a #AnimationMode
    /// @duration: duration of the animation in milliseconds
    /// @scale_mode: The #ImageScaleMode to set
    ///
    /// Sets the value of #Image:scale-mode to @scale_mode and animates the
    /// scale factor of the image between the previous value and the new value.
    ///
    fn animate_scale_mode(&self, mode: u64, duration: u32, scale_mode: ImageScaleMode) {
        let image = self.as_ref();

        // if image.mode != mode {
        //     image.previous_mode = image.mode;
        //     image.mode = scale_mode;

        //     timeline_stop(image.redraw_timeline);
        //     timeline_set_duration(image.redraw_timeline, duration);
        //     timeline_set_progress_mode(image.redraw_timeline, mode);
        //     timeline_start(image.redraw_timeline);

        //     g_object_notify(G_OBJECT(image), "scale-mode");
        // }
    }

    /// clear:
    /// @image: A #Image
    ///
    /// Clear the current image and set a blank, transparent image.
    ///
    /// Returns: static void
    ///
    fn clear(&self) {
        let image = self.as_ref();

        // image_cancel_in_progress (image);

        // if image.texture {
        //     cogl_object_unref(image.texture);
        // }

        // image.texture = cogl_object_ref(image.blank_texture);

        // if image.old_texture {
        //     cogl_object_unref(image.old_texture);
        // }

        // image.old_texture = cogl_object_ref(image.blank_texture);
        // image.old_rotation = image.rotation;
        // image.old_mode = image.mode;

        // if image.material {
        //     cogl_object_unref(image.material);
        // }

        // image.material = cogl_object_ref(image.template_material);

        // // the image has changed size, so update the preferred width/height
        // actor_queue_relayout(CLUTTER_ACTOR(image));
    }

    /// get_allow_upscale:
    /// @image: A #Image
    ///
    /// Determines whether image up-scaling is allowed.
    ///
    /// Returns: %true if upscaling is allowed, %false otherwise
    ///
    fn get_allow_upscale(&self) -> bool {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.upscale
    }

    /// get_image_rotation:
    /// @image: A #Image
    ///
    /// Get the value of the Image:image-rotation property.
    ///
    /// Returns: The value of the image-rotation property.
    ///
    fn get_image_rotation(&self) -> f32 {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.rotation
    }

    /// get_load_async:
    /// @image: A #Image
    ///
    /// Determines whether asynchronous image loading is in use.
    ///
    /// Returns: %true if images are set to load asynchronously, %false otherwise
    ///
    fn get_load_async(&self) -> bool {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.load_async
    }

    /// get_scale_height_threshold:
    /// @image: A #Image
    ///
    /// Retrieves the height scaling threshold.
    ///
    /// Returns: The height scaling threshold, in pixels
    ///
    fn get_scale_height_threshold(&self) -> u32 {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.height_threshold
    }

    /// get_scale_mode:
    /// @image: An #Image
    ///
    /// Get the current scale mode of @Image.
    ///
    /// Returns: The current ImageScaleMode
    ///
    fn get_scale_mode(&self) -> ImageScaleMode {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.mode
    }

    /// get_scale_width_threshold:
    /// @image: A #Image
    ///
    /// Retrieves the width scaling threshold.
    ///
    /// Returns: The width scaling threshold, in pixels
    ///
    fn get_scale_width_threshold(&self) -> u32 {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.width_threshold
    }

    /// get_transition_duration:
    /// @image: A #Image
    ///
    /// Get the value of the Image:transition-duration property.
    ///
    /// Returns: The value of the transition-duration property.
    ///
    fn get_transition_duration(&self) -> u32 {
        let image = self.as_ref();
        let props = image.props.borrow();

        props.transition_duration
    }

    /// set_allow_upscale:
    /// @image: A #Image
    /// @allow: %true to allow upscaling, %false otherwise
    ///
    /// Sets whether up-scaling of images is allowed. If set to %true and a size
    /// larger than the image is requested, the image will be up-scaled in
    /// software.
    ///
    /// The advantage of this is that software up-scaling is potentially higher
    /// quality, but it comes at the expense of video memory.
    ///
    fn set_allow_upscale(&self, allow: bool) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.upscale != allow {
            props.upscale = allow;
            // g_object_notify(G_OBJECT(image), "allow-upscale");
        }
    }

    /// set_from_buffer:
    /// @image: An #Image
    /// @buffer: (array length=buffer_size) (transfer full): A buffer
    ///   pointing to encoded image data
    /// @buffer_size: The size of @buffer, in bytes
    /// @buffer_free_func: (allow-none): A function to free @buffer, or %None
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from unencoded image data, stored in memory. In case of
    /// failure, #false is returned and @error is set. It is expected that @buffer
    /// will remain accessible for the duration of the load. Once it is finished
    /// with, @buffer_free_func will be called.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_buffer(&self, buffer: &[u8]) -> Result<(), glib::Error> {
    //     self.set_from_buffer_at_size(buffer, -1, -1)
    // }

    /// set_from_buffer_at_size:
    /// @image: An #Image
    /// @buffer: (array length=buffer_size) (transfer full): A buffer
    ///   pointing to encoded image data
    /// @buffer_size: The size of @buffer, in bytes
    /// @buffer_free_func: (allow-none): A function to free @buffer, or %None
    /// @width: Width to scale the image to, or -1
    /// @height: Height to scale the image to, or -1
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from unencoded image data, stored in memory, and scales
    /// it while loading. In case of failure, #false is returned and @error is set.
    /// It is expected that @buffer will remain accessible for the duration of the
    /// load. Once it is finished with, @buffer_free_func will be called. The aspect
    /// ratio will always be maintained.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_buffer_at_size(
    //     &self,
    //     buffer: &[u8],
    //     width: i32,
    //     height: i32,
    // ) -> Result<(), glib::Error> {
    //     let image = self.as_ref();
    //     let props = image.props.borrow();

    //     if props.load_async {
    //         //     return image.set_async(None, buffer, buffer_size,
    //         //                             buffer_free_func, width, height, error);
    //     }

    //     // let pixbuf: GdkPixbuf = Image::pixbuf_new(None, buffer, buffer_size, width, height,
    //     //     image.width_threshold, image.height_threshold,
    //     //     image.upscale, None, error);
    //     // if !pixbuf {
    //     //     return false;
    //     // }

    //     // let retval = image.set_from_pixbuf(pixbuf, None, error);

    //     // g_object_unref(pixbuf);

    //     // if buffer_free_func {
    //     //     buffer_free_func((gpointer)buffer);
    //     // }

    //     // retval
    //     unimplemented!()
    // }

    /// set_from_cogl_texture:
    /// @image: A #Image
    /// @texture: A #CoglHandle to a texture
    ///
    /// Sets the contents of the image from the given Cogl texture.
    ///
    /// Returns: %true on success, %false on failure
    ///
    fn set_from_cogl_texture(&self, texture: Handle) -> bool {
        //     let image = self.as_ref();

        //     gint width, height;

        //     g_return_val_if_fail(IS_IMAGE (image), false);
        //     g_return_val_if_fail(cogl_is_texture (texture), false);

        //     image_cancel_in_progress(image);

        //     width = cogl_texture_get_width(texture);
        //     height = cogl_texture_get_height(texture);

        //     // If we have offscreen buffers, use those to add the 1-pixel border
        //     // around the image on the GPU - if not, fallback to copying the image
        //     // data into memory and use set_from_data.
        //     if feature_available(CLUTTER_FEATURE_OFFSCREEN) {
        //         CoglColor transparent;
        //         CoglMaterial *clear_material;

        //         CoglHandle new_texture =
        //           cogl_texture_new_with_size(width + 2, height + 2,
        //                                       COGL_TEXTURE_NO_ATLAS,
        //                                       COGL_PIXEL_FORMAT_RGBA_8888);
        //         CoglHandle fbo = cogl_offscreen_new_to_texture(new_texture);
        //         CoglMaterial *tex_material = cogl_material_new();

        //         /* Set the blending equation to directly copy the bits of the old
        //          * texture without blending the destination pixels.
        //          */
        //         cogl_material_set_blend(tex_material, "RGBA=ADD(SRC_COLOR, 0)", None);
        //         clear_material = cogl_material_copy(tex_material);

        //         cogl_color_set_from_4ub(&transparent, 0, 0, 0, 0);
        //         cogl_material_set_layer(tex_material, 0, texture);

        //         /* Push the off-screen buffer and setup an orthographic projection */
        //         cogl_push_framebuffer(fbo);
        //         cogl_ortho(0, width + 2, height +2, 0, -1, 1);

        //         /* Draw the texture into the middle */
        //         cogl_push_source(tex_material);
        //         cogl_rectangle(1, 1, width +1, height + 1);

        //         /* Clear the 1-pixel border around the texture */
        //         cogl_set_source(clear_material);
        //         cogl_rectangle(0, 0, width + 2, 1);
        //         cogl_rectangle(0, height + 1, width + 2, height + 2);
        //         cogl_rectangle(0, 1, 1, height + 1);
        //         cogl_rectangle(width + 1, 1, width + 2, height + 1);

        //         cogl_pop_source();
        //         cogl_pop_framebuffer();

        //         /* Free unneeded data */
        //         cogl_object_unref(clear_material);
        //         cogl_object_unref(tex_material);
        //         cogl_handle_unref(fbo);

        //         /* Replace the old texture */
        //         if (priv->old_texture)
        //           cogl_object_unref(priv->old_texture);

        //         priv->old_texture = priv->texture;
        //         priv->old_rotation = priv->rotation;
        //         priv->old_mode = priv->mode;

        //         priv->texture = new_texture;

        //         image_prepare_texture(image);

        //         return true;
        //       } else {
        //         guint8 *data;
        //         gint rowstride;
        //         CoglPixelFormat format;

        //         rowstride = cogl_texture_get_rowstride(texture);
        //         format = cogl_texture_get_format(texture);

        //         data = g_malloc(height * rowstride);
        //         cogl_texture_get_data(texture, format, rowstride, data);
        //         return image_set_from_data(image, data, format,
        //                                        width, height, rowstride, None);
        //       }
        unimplemented!()
    }

    /// set_from_data:
    /// @image: An #Image
    /// @data: (array): Image data
    /// @pixel_format: The #CoglPixelFormat of the buffer
    /// @width: Width in pixels of image data.
    /// @height: Height in pixels of image data
    /// @rowstride: Distance in bytes between row starts.
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from a buffer. In case of failure, #false is returned
    /// and @error is set.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_data(
    //     &self,
    //     data: &[u8],
    //     pixel_format: dx::PixelFormat,
    //     width: i32,
    //     height: i32,
    //     rowstride: i32,
    // ) -> Result<(), glib::Error> {
    //     let image = self.as_ref();

    //     // image.set_from_data_internal(image, data, None, false,
    //     //                                       pixel_format, width, height,
    //     //                                       rowstride, error);
    //     unimplemented!()
    // }

    /// set_from_file:
    /// @image: An #Image
    /// @filename: Filename to read the file from
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from an image file. In case of failure, #false is returned
    /// and @error is set.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_file(&self, filename: &str) -> Result<(), glib::Error> {
    //     let image = self.as_ref();
    //     image.set_from_file_at_size(filename, -1, -1)
    // }

    /// set_from_file_at_size:
    /// @image: An #Image
    /// @filename: Filename to read the file from
    /// @width: Width to scale the image to, or -1
    /// @height: Height to scale the image to, or -1
    /// @error: Return location for a #GError, or #None
    ///
    /// Set the image data from an image file, and scale the image during loading.
    /// In case of failure, #false is returned and @error is set. The aspect ratio
    /// will always be maintained.
    ///
    /// Returns: #true if the image was successfully updated
    ///
    // fn set_from_file_at_size(
    //     &self,
    //     filename: &str,
    //     width: i32,
    //     height: i32,
    // ) -> Result<(), glib::Error> {
    //     let image = self.as_ref();

    //     // GdkPixbuf *pixbuf;
    //     // ImagePrivate *priv;
    //     // TextureCache *cache;
    //     // gboolean retval, use_cache;

    //     // pixbuf = None;

    //     // // Check if the processed image is in the cache - we don't use the cache
    //     // // if we're loading at a particular size.
    //     // cache = texture_cache_get_default();
    //     // use_cache = true;

    //     // if (width != -1) || (height != -1) ||
    //     //     !texture_cache_contains_meta(cache, filename,
    //     //                                     GINT_TO_POINTER(image_cache_quark)) {
    //     //     // Check if the unprocessed image is in the cache, and if so, skip
    //     //     // loading it and set it from the Cogl texture handle.
    //     //     if (width == -1) && (height == -1) &&
    //     //         texture_cache_contains(cache, filename) {
    //     //         if image_set_from_cogl_texture(image,
    //     //             texture_cache_get_cogl_texture(cache, filename)) {
    //     //             // Add the processed image to the cache
    //     //             texture_cache_insert_meta (cache, filename,
    //     //                                         GINT_TO_POINTER (image_cache_quark),
    //     //                                         image.texture, None);
    //     //             return true;
    //     //         } else {
    //     //             g_set_error (error, IMAGE_ERROR, IMAGE_ERROR_INTERNAL,
    //     //                         "Setting image '%s' from CoglTexture failed",
    //     //                         filename);
    //     //             return false;
    //     //         }
    //     //     }

    //     //     // Load the pixbuf in a thread, then later on upload it to the GPU
    //     //     if image.load_async {
    //     //         return image.set_async(filename, None, 0, None,
    //     //                                 width, height, error);
    //     //     }

    //     //     // Synchronously load the pixbuf and set it
    //     //     pixbuf = image_pixbuf_new(filename, None, 0, width, height,
    //     //         image.width_threshold,
    //     //         image.height_threshold,
    //     //         image.upscale, &use_cache, error);

    //     //     if !pixbuf {
    //     //         return false;
    //     //     }
    //     // }

    //     // retval = image.set_from_pixbuf(pixbuf, use_cache ? filename : None, error);

    //     // if pixbuf {
    //     //     g_object_unref (pixbuf);
    //     // }

    //     // return retval;

    //     unimplemented!()
    // }

    /// set_image_rotation:
    /// @image: A #Image
    /// @rotation: Rotation angle in degrees
    ///
    /// Set the Image:image-rotation property.
    ///
    fn set_image_rotation(&self, rotation: f32) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.rotation != rotation {
            props.rotation = rotation;

            // actor_queue_redraw(CLUTTER_ACTOR(image));

            // g_object_notify(G_OBJECT(image), "image-rotation");
        }
    }

    /// set_load_async:
    /// @image: A #Image
    /// @load_async: %true to load images asynchronously
    ///
    /// Sets whether to load images asynchronously. Asynchronous image loading
    /// requires thread support (see g_thread_init()).
    ///
    /// When using asynchronous image loading, all image-loading functions will
    /// return immediately as successful. The #Image::image-loaded and
    /// #Image::image-load-error signals are used to signal success or failure
    /// of asynchronous image loading.
    ///
    fn set_load_async(&self, load_async: bool) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.load_async != load_async {
            props.load_async = load_async;
            // g_object_notify(G_OBJECT(image), "load-async");

            // Cancel the old transfer if we're turning async off
            if !load_async && props.async_load_data.is_some() {
                // props.async_load_data.cancelled = true;
                props.async_load_data = None;
            }
        }
    }

    /// set_scale_height_threshold:
    /// @image: A #Image
    /// @pixels: Number of pixels
    ///
    /// Sets the threshold used to determine whether to scale the height of the
    /// image. If a specific height is requested, the image height is allowed to
    /// differ by this amount before scaling is employed.
    ///
    /// This can be useful to avoid excessive CPU usage when the image differs
    /// only slightly to the desired size.
    ///
    fn set_scale_height_threshold(&self, pixels: u32) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.height_threshold != pixels {
            props.height_threshold = pixels;
            // g_object_notify(G_OBJECT (image), "scale-height-threshold");
        }
    }

    /// set_scale_mode:
    /// @image: An #Image
    /// @mode: The #ImageScaleMode to set
    ///
    /// Set the scale mode on @Image
    ///
    fn set_scale_mode(&self, mode: ImageScaleMode) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.mode != mode {
            props.previous_mode = mode;
            props.mode = mode;

            // g_object_notify(G_OBJECT(image), "scale-mode");
        }

        // actor_queue_redraw(CLUTTER_ACTOR (image));
    }

    /// set_scale_width_threshold:
    /// @image: A #Image
    /// @pixels: Number of pixels
    ///
    /// Sets the threshold used to determine whether to scale the width of the
    /// image. If a specific width is requested, the image width is allowed to
    /// differ by this amount before scaling is employed.
    ///
    /// This can be useful to avoid excessive CPU usage when the image differs
    /// only slightly to the desired size.
    ///
    fn set_scale_width_threshold(&self, pixels: u32) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.width_threshold != pixels {
            props.width_threshold = pixels;
            // g_object_notify(G_OBJECT(image), "scale-width-threshold");
        }
    }

    /// set_transition_duration:
    /// @image: A #Image
    /// @duration: Transition duration in milliseconds
    ///
    /// Set the Image:transition-duration property.
    ///
    fn set_transition_duration(&self, duration: u32) {
        let image = self.as_ref();
        let mut props = image.props.borrow_mut();

        if props.transition_duration != duration {
            props.transition_duration = duration;

            if duration != 0 {
                // timeline_set_duration(image.timeline, duration);
            }
            // g_object_notify(G_OBJECT (image), "transition-duration");
        }
    }

    fn set_property_filename(&self, filename: Option<&str>) {
        let image = self.as_ref();
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"filename\0".as_ptr() as *const _,
        //         Value::from(filename).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    // fn connect_image_load_error<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> HandlerId {
    //     // unsafe extern "C" fn image_load_error_trampoline<P, F: Fn(&P, &glib::Error) + 'static>(
    //     //     this: *mut ffi::Image,
    //     //     object: *mut glib_sys::GError,
    //     //     f: glib_sys::gpointer,
    //     // ) where
    //     //     P: Is<Image>,
    //     // {
    //     //     let f: &F = &*(f as *const F);
    //     //     f(
    //     //         &Image::from_glib_borrow(this).unsafe_cast_ref(),
    //     //         &from_glib_borrow(object),
    //     //     )
    //     // }
    //     // unsafe {
    //     //     let f: Box<F> = Box::new(f);
    //     //     connect_raw(
    //     //         self.as_ptr() as *mut _,
    //     //         b"image-load-error\0".as_ptr() as *const _,
    //     //         Some(transmute::<_, unsafe extern "C" fn()>(
    //     //             image_load_error_trampoline::<Self, F> as *const (),
    //     //         )),
    //     //         Box::into_raw(f),
    //     //     )
    //     // }
    //     unimplemented!()
    // }

    fn connect_image_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn image_loaded_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"image-loaded\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             image_loaded_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_allow_upscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_allow_upscale_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::allow-upscale\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_allow_upscale_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_filename_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::filename\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_filename_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_image_rotation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_image_rotation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::image-rotation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_image_rotation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_load_async_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_load_async_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::load-async\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_load_async_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_scale_height_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_scale_height_threshold_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scale-height-threshold\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scale_height_threshold_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_scale_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_scale_mode_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scale-mode\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scale_mode_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_scale_width_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_scale_width_threshold_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::scale-width-threshold\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_scale_width_threshold_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_transition_duration_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Image,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Image>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box<F> = Box::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::transition-duration\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_transition_duration_trampoline::<Self, F> as *const (),
        //         )),
        //         Box::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
