pub trait AlignmentGeometry {
    // Returns the sum of two AlignmentGeometry objects.
    // add(AlignmentGeometry other) -> AlignmentGeometry
    
    // Convert this instance into an Alignment, which uses literal coordinates (the x coordinate being explicitly a distance from the left).//
    // resolve(TextDirection? direction) -> Alignment
}

pub struct NoneAlignmentGeometry;

impl AlignmentGeometry for NoneAlignmentGeometry {
    
}
