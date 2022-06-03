use cgmath::Matrix4;

use crate::ui::{TextDirection, Rect};

pub trait GradientTransform {
    // When a Gradient creates its Shader, it will call this method to determine what transform 
    // to apply to the shader for the given Rect and TextDirection. 
    fn transform(&self, bounds: Rect, text_direction: Option<TextDirection>) -> Matrix4<f32>;
}

#[derive(Default, Debug, Copy, Clone)]
pub struct NullGradientTransform;

impl GradientTransform for NullGradientTransform {
    fn transform(&self, bounds: Rect, text_direction: Option<TextDirection>) -> Matrix4<f32> {
        todo!()
    }
}