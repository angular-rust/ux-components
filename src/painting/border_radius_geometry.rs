use crate::ui::TextDirection;

use super::BorderRadius;

pub trait BorderRadiusGeometry {
    // Returns the sum of two BorderRadiusGeometry objects.
    fn add(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry>;
    
    // Convert this instance into a BorderRadius, so that the radii are expressed for specific 
    // physical corners (top-left, top-right, etc) rather than in a direction-dependent manner.
    fn resolve(&self, direction: Option<TextDirection>) -> BorderRadius;

    // Returns the difference between two BorderRadiusGeometry objects.
    fn subtract(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry>;
}

#[derive(Default)]
pub struct NoneBorderRadiusGeometry;

impl BorderRadiusGeometry for NoneBorderRadiusGeometry {
    fn add(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry> {
        todo!()
    }

    fn resolve(&self, direction: Option<TextDirection>) -> BorderRadius {
        todo!()
    }

    fn subtract(&self, other: Box<dyn BorderRadiusGeometry>) -> Box<dyn BorderRadiusGeometry> {
        todo!()
    }
}