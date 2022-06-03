use crate::ui::{TextDirection, Shader, Rect};
use crate::{prelude::Color, ui::TileMode};

use crate::material::AlignmentGeometry;

use super::{Gradient, GradientTransform};

pub struct LinearGradient {
    // The offset at which stop 0.0 of the gradient is placed.
    pub begin: Box<dyn AlignmentGeometry>,
    
    // The colors the gradient should obtain at each of the stops.
    pub colors: Vec<Color>,
    
    // The offset at which stop 1.0 of the gradient is placed.
    pub end: Box<dyn AlignmentGeometry>,
    
    
    // A list of values from 0.0 to 1.0 that denote fractions along the gradient.
    pub stops: Vec<f32>,
    
    // How this gradient should tile the plane beyond in the region before begin and after end.
    pub tile_mode: TileMode,
    
    
    // The transform, if any, to apply to the gradient.
    pub transform: Box<dyn GradientTransform>,
}

impl Gradient for LinearGradient {
    fn create_shader(&self, rect: Rect, text_direction: Option<TextDirection>) -> Shader {
        todo!()
    }

    fn lerp_from(&self, a: Box<dyn Gradient>, t: f32) -> Box<dyn Gradient> {
        todo!()
    }

    fn lerp_to(&self, b: Box<dyn Gradient>, t: f32) -> Box<dyn Gradient> {
        todo!()
    }

    fn scale(&self, factor: f32) -> Box<dyn Gradient> {
        todo!()
    }
}