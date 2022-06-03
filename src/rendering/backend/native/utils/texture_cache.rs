#![allow(unused_variables)]

use crate::prelude::*;

use crate::platform::core::Handle;

use std::{cell::RefCell, fmt};

#[derive(Clone, Debug)]
pub struct FinalizedClosure {
    pub uri: String,
    pub cache: TextureCache,
}

// Convention: posX with a value of -1 indicates whole texture
#[derive(Debug)]
pub struct TextureCacheItem {
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub ptr: Handle,
    // pub meta: GHashTable,
}

#[derive(Debug)]
pub struct TextureCacheMetaEntry {
    // pub ident: gpointer,
    pub texture: Handle,
    // pub destroy_func: GDestroyNotify,
}

#[derive(Clone, Debug)]
pub struct TextureCacheProps {
    // pub parent: GObject,
// pub cache: GHashTable,
// pub is_uri: GRegex,
}
#[derive(Clone, Debug)]
pub struct TextureCache {
    props: RefCell<TextureCacheProps>,
}

impl TextureCache {
    /// get_default:
    ///
    /// Returns the default texture cache. This is owned by Mx and should not be
    /// unreferenced or freed.
    ///
    /// Returns: (transfer none): a TextureCache
    ///
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

pub trait TextureCacheExt: 'static {
    /// contains:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    ///
    /// Checks whether the given URI/path is contained within the texture
    /// cache.
    ///
    /// Returns: %true if the image exists, %false otherwise
    ///
    fn contains(&self, uri: &str) -> bool;

    /// contains_meta:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    /// @ident: A unique identifier
    ///
    /// Checks whether there are any textures associated with the given URI by
    /// the given identifier.
    ///
    /// Returns: %true if the data exists, %false otherwise
    ///
    //fn contains_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> bool;

    /// get_cogl_texture:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    ///
    /// Create a #CoglHandle representing a texture of the specified image. Adds
    /// the image to the cache if the image had not been previously loaded.
    /// Subsequent calls with the same image URI/path will return the #CoglHandle of
    /// the previously loaded image with an increased reference count.
    ///
    /// Returns: (transfer none): a #CoglHandle to the cached texture
    ///
    fn get_cogl_texture(&self, uri: &str) -> Option<Handle>;

    /// get_meta_cogl_texture:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    /// @ident: A unique identifier
    ///
    /// Retrieves the #CoglHandle of the previously added image associated
    /// with the given unique identifier.
    ///
    /// See insert_meta()
    ///
    /// Returns: (transfer full): A #CoglHandle to a texture, with an added
    ///   reference. %None if no image was found.
    ///
    // fn get_meta_cogl_texture(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> Option<Handle>;

    /// get_size:
    /// @self: A #TextureCache
    ///
    /// Returns the number of items in the texture cache
    ///
    /// Returns: the current size of the cache
    ///
    fn get_size(&self) -> usize;

    /// insert:
    /// @self: A #TextureCache
    /// @uri: A URI or local file path
    /// @texture: A #CoglHandle to a texture
    ///
    /// Inserts a texture into the texture cache. This can be useful if you
    /// want to cache a texture from a custom or unhandled URI type, or you
    /// want to override a particular texture.
    ///
    /// If the image is already in the cache, this texture will replace it. A
    /// reference will be taken on the given texture.
    ///
    fn insert(&self, uri: &str, texture: Handle);

    /// insert_meta:
    /// @self: A #TextureCache
    /// @uri: A URI or local file path
    /// @ident: A unique identifier
    /// @texture: A #CoglHandle to a texture
    /// @destroy_func: An optional destruction function for @ident
    ///
    /// Inserts a texture that's associated with a URI into the cache.
    /// If the metadata already exists for this URI, it will be replaced.
    ///
    /// This is useful if you have a widely used modification of an image,
    /// for example, an image with a border composited around it.
    ///
    // fn insert_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>, texture: Handle);

    fn load_cache(&self, filename: &str);
}

impl<O: Is<TextureCache>> TextureCacheExt for O {
    /// contains:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    ///
    /// Checks whether the given URI/path is contained within the texture
    /// cache.
    ///
    /// Returns: %true if the image exists, %false otherwise
    ///
    fn contains(&self, uri: &str) -> bool {
        let cache = self.as_ref();
        // cache.get_item (uri, false) ? true : false;
        unimplemented!()
    }

    /// contains_meta:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    /// @ident: A unique identifier
    ///
    /// Checks whether there are any textures associated with the given URI by
    /// the given identifier.
    ///
    /// Returns: %true if the data exists, %false otherwise
    ///
    //fn contains_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:texture_cache_contains_meta() }
    //}

    /// get_cogl_texture:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    ///
    /// Create a #CoglHandle representing a texture of the specified image. Adds
    /// the image to the cache if the image had not been previously loaded.
    /// Subsequent calls with the same image URI/path will return the #CoglHandle of
    /// the previously loaded image with an increased reference count.
    ///
    /// Returns: (transfer none): a #CoglHandle to the cached texture
    ///
    fn get_cogl_texture(&self, uri: &str) -> Option<Handle> {
        // unsafe { TODO: call ffi:texture_cache_get_cogl_texture() }
        unimplemented!()
    }

    /// get_meta_cogl_texture:
    /// @self: A #TextureCache
    /// @uri: A URI or path to an image file
    /// @ident: A unique identifier
    ///
    /// Retrieves the #CoglHandle of the previously added image associated
    /// with the given unique identifier.
    ///
    /// See insert_meta()
    ///
    /// Returns: (transfer full): A #CoglHandle to a texture, with an added
    ///   reference. %None if no image was found.
    ///
    // fn get_meta_cogl_texture(&self, uri: &str, ident: Option<Fundamental: Pointer>) -> Option<Handle> {
    // unsafe { TODO: call ffi:texture_cache_get_meta_cogl_texture() }
    // unimplemented!()
    // }

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

    /// insert:
    /// @self: A #TextureCache
    /// @uri: A URI or local file path
    /// @texture: A #CoglHandle to a texture
    ///
    /// Inserts a texture into the texture cache. This can be useful if you
    /// want to cache a texture from a custom or unhandled URI type, or you
    /// want to override a particular texture.
    ///
    /// If the image is already in the cache, this texture will replace it. A
    /// reference will be taken on the given texture.
    ///
    fn insert(&self, uri: &str, texture: Handle) {
        // unsafe { TODO: call ffi:texture_cache_insert() }
    }

    /// insert_meta:
    /// @self: A #TextureCache
    /// @uri: A URI or local file path
    /// @ident: A unique identifier
    /// @texture: A #CoglHandle to a texture
    /// @destroy_func: An optional destruction function for @ident
    ///
    /// Inserts a texture that's associated with a URI into the cache.
    /// If the metadata already exists for this URI, it will be replaced.
    ///
    /// This is useful if you have a widely used modification of an image,
    /// for example, an image with a border composited around it.
    ///
    //fn insert_meta(&self, uri: &str, ident: Option<Fundamental: Pointer>, texture: Handle) {
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
