#![allow(unused_variables)]

use crate::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TextureCache {}

impl TextureCache {
    pub fn get_default() -> Option<TextureCache> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::texture_cache_get_default()) }
        unimplemented!()
    }
}

impl Object for TextureCache {}
impl Is<TextureCache> for TextureCache {}

impl AsRef<TextureCache> for TextureCache {
    fn as_ref(&self) -> &TextureCache {
        self
    }
}

pub const NONE_TEXTURE_CACHE: Option<&TextureCache> = None;

pub trait TextureCacheExt: 'static {
    fn contains(&self, uri: &str) -> bool;

    //fn contains_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> bool;

    //fn get_cogl_texture(&self, uri: &str) -> /*Ignored*/Option<cogl::Handle>;

    //fn get_meta_cogl_texture(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> /*Ignored*/Option<cogl::Handle>;

    fn get_size(&self) -> i32;

    //fn insert(&self, uri: &str, texture: /*Ignored*/cogl::Handle);

    //fn insert_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>, texture: /*Ignored*/cogl::Handle);

    fn load_cache(&self, filename: &str);
}

impl<O: Is<TextureCache>> TextureCacheExt for O {
    fn contains(&self, uri: &str) -> bool {
        // unsafe {
        //     from_glib(ffi::texture_cache_contains(
        //         self.as_ref().to_glib_none().0,
        //         uri.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    //fn contains_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:texture_cache_contains_meta() }
    //}

    //fn get_cogl_texture(&self, uri: &str) -> /*Ignored*/Option<cogl::Handle> {
    //    unsafe { TODO: call ffi:texture_cache_get_cogl_texture() }
    //}

    //fn get_meta_cogl_texture(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> /*Ignored*/Option<cogl::Handle> {
    //    unsafe { TODO: call ffi:texture_cache_get_meta_cogl_texture() }
    //}

    fn get_size(&self) -> i32 {
        // unsafe { ffi::texture_cache_get_size(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    //fn insert(&self, uri: &str, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:texture_cache_insert() }
    //}

    //fn insert_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:texture_cache_insert_meta() }
    //}

    fn load_cache(&self, filename: &str) {
        // unsafe {
        //     ffi::texture_cache_load_cache(
        //         self.as_ref().to_glib_none().0,
        //         filename.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }
}

impl fmt::Display for TextureCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureCache")
    }
}
