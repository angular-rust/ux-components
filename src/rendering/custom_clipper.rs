use crate::ui::{Rect, Size, VoidCallback, Path};

pub trait CustomClipper<T> {
    // Register a closure to be notified when it is time to reclip.
    fn add_listener(&self, listener: VoidCallback) {
        todo!()
    }

    // Returns an approximation of the clip returned by getClip, as an axis-aligned Rect.
    // This is used by the semantics layer to determine whether widgets should be excluded.
    fn get_approximate_clip_rect(&self, size: Size) -> Rect {
        todo!()
    }

    // Returns a description of the clip given that the render object being clipped is of the given size.
    fn get_clip(&self, size: Size) -> T;

    // Remove a previously registered closure from the list of closures that the object notifies when it is time to reclip.
    fn remove_listener(&self, listener: VoidCallback) {}

    // Called whenever a new instance of the custom clipper delegate class is provided to the clip object,
    // or any time that a new clip object is created with a new instance of the custom clipper
    // delegate class (which amounts to the same thing, because the latter is implemented in terms of the former).
    fn should_reclip(&self, old_clipper: Box<dyn CustomClipper<T>>) -> bool {
        todo!()
    }
}

#[derive(Default)]
pub struct NoneCustomPathClipper;

impl CustomClipper<Path> for NoneCustomPathClipper {
    fn get_clip(&self, size: Size) -> Path {
        todo!()
    }
}

#[derive(Default)]
pub struct NoneCustomRectClipper;

impl CustomClipper<Rect> for NoneCustomRectClipper {
    fn get_clip(&self, size: Size) -> Rect {
        todo!()
    }
}