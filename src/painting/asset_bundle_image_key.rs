use crate::services::AssetBundle;

pub struct AssetBundleImageKey {
    // The bundle from which the image will be obtained.
    pub bundle: Box<dyn AssetBundle>,

    // The key to use to obtain the resource from the bundle. This is the argument passed to AssetBundle.load.
    pub name: String,

    // The scale to place in the ImageInfo object of the image.
    pub scale: f64,
}

// impl<T: Default> Default for ImageProvider<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }
