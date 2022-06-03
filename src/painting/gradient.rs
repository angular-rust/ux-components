use crate::ui::{Rect, Shader, TextDirection};

pub trait Gradient {
    // The colors the gradient should obtain at each of the stops.
    // pub colors: Vec<Color>,

    // A list of values from 0.0 to 1.0 that denote fractions along the gradient.
    // pub stops: Vec<f32>,

    // The transform, if any, to apply to the gradient.
    // pub transform: Option<Box<dyn GradientTransform>>,

    // Creates a Shader for this gradient to fill the given rect.
    fn create_shader(&self, rect: Rect, text_direction: Option<TextDirection>) -> Shader {
        todo!()
    }

    // Linearly interpolates from another Gradient to this.
    fn lerp_from(&self, a: Box<dyn Gradient>, t: f32) -> Box<dyn Gradient> {
        todo!()
    }

    // Linearly interpolates from this to another Gradient.
    fn lerp_to(&self, b: Box<dyn Gradient>, t: f32) -> Box<dyn Gradient> {
        todo!()
    }

    // Returns a new gradient with its properties scaled by the given factor.
    fn scale(&self, factor: f32) -> Box<dyn Gradient> {
        todo!()
    }
}
