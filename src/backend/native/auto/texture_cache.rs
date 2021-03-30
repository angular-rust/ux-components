use crate::prelude::*;
// use glib::translate::*;

use std::fmt;

// glib_wrapper! {
//     pub struct TextureCache(Object<ffi::TextureCache, ffi::TextureCacheClass, TextureCacheClass>);

//     match fn {
//         get_type => || ffi::texture_cache_get_type(),
//     }
// }

#[derive(Clone, Debug)]
pub struct TextureCache {

}

impl TextureCache {
    pub fn get_default() -> Option<TextureCache> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::texture_cache_get_default()) }
        unimplemented!()
    }
}

pub const NONE_TEXTURE_CACHE: Option<&TextureCache> = None;

// pub trait TextureCacheExt: 'static {
//     fn contains(&self, uri: &str) -> bool;

//     //fn contains_meta(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

//     //fn get_cogl_texture(&self, uri: &str) -> /*Ignored*/Option<cogl::Handle>;

//     //fn get_meta_cogl_texture(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<cogl::Handle>;

//     fn get_size(&self) -> i32;

//     //fn insert(&self, uri: &str, texture: /*Ignored*/cogl::Handle);

//     //fn insert_meta(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>, texture: /*Ignored*/cogl::Handle);

//     fn load_cache(&self, filename: &str);
// }

// impl<O: IsA<TextureCache>> TextureCacheExt for O {
//     fn contains(&self, uri: &str) -> bool {
//         unsafe {
//             from_glib(ffi::texture_cache_contains(
//                 self.as_ref().to_glib_none().0,
//                 uri.to_glib_none().0,
//             ))
//         }
//     }

//     //fn contains_meta(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//     //    unsafe { TODO: call ffi:mx_texture_cache_contains_meta() }
//     //}

//     //fn get_cogl_texture(&self, uri: &str) -> /*Ignored*/Option<cogl::Handle> {
//     //    unsafe { TODO: call ffi:mx_texture_cache_get_cogl_texture() }
//     //}

//     //fn get_meta_cogl_texture(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<cogl::Handle> {
//     //    unsafe { TODO: call ffi:mx_texture_cache_get_meta_cogl_texture() }
//     //}

//     fn get_size(&self) -> i32 {
//         unsafe { ffi::texture_cache_get_size(self.as_ref().to_glib_none().0) }
//     }

//     //fn insert(&self, uri: &str, texture: /*Ignored*/cogl::Handle) {
//     //    unsafe { TODO: call ffi:mx_texture_cache_insert() }
//     //}

//     //fn insert_meta(&self, uri: &str, ident: /*Unimplemented*/Option<Fundamental: Pointer>, texture: /*Ignored*/cogl::Handle) {
//     //    unsafe { TODO: call ffi:mx_texture_cache_insert_meta() }
//     //}

//     fn load_cache(&self, filename: &str) {
//         unsafe {
//             ffi::texture_cache_load_cache(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//             );
//         }
//     }
// }

impl fmt::Display for TextureCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureCache")
    }
}
