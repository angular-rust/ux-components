#![allow(unused_variables)]

use crate::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct FinalizedClosure {
    pub uri: String,
    pub cache: TextureCache,
}

// Convention: posX with a value of -1 indicates whole texture
#[derive(Clone, Debug)]
pub struct TextureCacheItem {
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    // pub ptr: cogl::Handle,
    // pub meta: GHashTable,
}

#[derive(Clone, Debug)]
pub struct TextureCacheMetaEntry {
    // pub ident: gpointer,
    // pub texture: cogl::Handle,
    // pub destroy_func: GDestroyNotify,
}

#[derive(Clone, Debug)]
pub struct TextureCache {
    // pub parent: GObject,
    // pub cache: GHashTable,
    // pub is_uri: GRegex,
}

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

    /// get_size:
    /// @self: A #TextureCache
    ///
    /// Returns the number of items in the texture cache
    ///
    /// Returns: the current size of the cache
    ///
    fn get_size(&self) -> usize;

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

    /// get_size:
    /// @self: A #TextureCache
    ///
    /// Returns the number of items in the texture cache
    ///
    /// Returns: the current size of the cache
    ///
    fn get_size(&self) -> usize {
        let cache = self.as_ref();
        // cache.cache.len()
        unimplemented!()
    }

    //fn insert(&self, uri: &str, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:texture_cache_insert() }
    //}

    //fn insert_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:texture_cache_insert_meta() }
    //}

    fn load_cache(&self, filename: &str) {
        let cache = self.as_ref();

        // FILE *file;
        // TextureCacheItem *element;
        // TextureCacheItem head;
        // CoglHandle full_texture;


        // let file = fopen(filename, "rm");
        // if !file {
        //     return;
        // }

        // let ret = fread(&head, sizeof(TextureCacheItem), 1, file);
        // if ret < 0 {
        //     fclose (file);
        //     return;
        // }

        // // check if we already if this texture in the cache
        // if g_hash_table_lookup(cache.cache, head.filename) {
        //     /* skip it, we're done */
        //     fclose (file);
        //     return;
        // }

        // let full_texture = texture_cache_get_cogl_texture(self, head.filename);

        // if full_texture == COGL_INVALID_HANDLE {
        //     g_critical (G_STRLOC ": Error opening cache image file");
        //     fclose (file);
        //     return;
        // }

        // while !feof(file) {
        //     gchar *uri;

        //     element = texture_cache_item_new();
        //     ret = fread(element, sizeof(TextureCacheItem), 1, file);

        //     if ret < 1 {
        //         // end of file 
        //         texture_cache_item_free(element);
        //         break;
        //     }

        //     uri = texture_cache_filename_to_uri(element.filename);
        //     if (!uri) {
        //         // Couldn't resolve path 
        //         texture_cache_item_free(element);
        //         continue;
        //     }

        //     if g_hash_table_lookup (cache.cache, uri) {
        //         // URI is already in the cache....
        //         texture_cache_item_free(element);
        //         g_free(uri);
        //     } else {
        //         element.ptr = cogl_texture_new_from_sub_texture (full_texture,
        //                                                             element.posX,
        //                                                             element.posY,
        //                                                             element.width,
        //                                                             element.height);
        //         g_hash_table_insert(cache.cache, uri, element);
        //     }
        // }

        // fclose(file);
    }
}

impl fmt::Display for TextureCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureCache")
    }
}
