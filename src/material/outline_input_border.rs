use crate::painting::{BorderSide, EdgeInsetsGeometry, BorderRadius, NoneEdgeInsetsGeometry};

// add(ShapeBorder other, {bool reversed = false}) → ShapeBorder?
// Attempts to create a new object that represents the amalgamation of this border and the other border. [...]
// @protected, inherited
// copyWith({BorderSide? borderSide, BorderRadius? borderRadius, double? gapPadding}) → OutlineInputBorder
// Creates a copy of this input border with the specified borderSide.
// override
// getInnerPath(Rect rect, {TextDirection? textDirection}) → Path
// Create a Path that describes the inner edge of the border. [...]
// override
// getOuterPath(Rect rect, {TextDirection? textDirection}) → Path
// Create a Path that describes the outer edge of the border. [...]
// override
// lerpFrom(ShapeBorder? a, double t) → ShapeBorder?
// Linearly interpolates from another ShapeBorder (possibly of another class) to this. [...]
// override
// lerpTo(ShapeBorder? b, double t) → ShapeBorder?
// Linearly interpolates from this to another ShapeBorder (possibly of another class). [...]
// override
// noSuchMethod(Invocation invocation) → dynamic
// Invoked when a non-existent method or property is accessed. [...]
// inherited
// paint(Canvas canvas, Rect rect, {double? gapStart, double gapExtent = 0.0, double gapPercentage = 0.0, TextDirection? textDirection}) → void
// Draw a rounded rectangle around rect using borderRadius. [...]
// override
// scale(double t) → OutlineInputBorder
// Creates a copy of this border, scaled by the factor t. [...]
// override


pub struct OutlineInputBorder {
    // The radii of the border's rounded rectangle corners.
    pub border_radius: BorderRadius,
    
    // Defines the border line's color and weight.
    pub border_side: BorderSide,
    
    // The widths of the sides of this border represented as an EdgeInsets.
    pub dimensions: Box<dyn EdgeInsetsGeometry>,
    
    // Horizontal padding on either side of the border's InputDecoration.labelText width gap.
    pub gap_padding: f32,
    
    // True if this border will enclose the InputDecorator's container.
    pub is_outline: bool,
}

impl OutlineInputBorder {
    // pub const NONE: OutlineInputBorder = OutlineInputBorder {
    //     border_side: BorderSide::NONE,
    // };
}

impl Default for OutlineInputBorder {
    fn default() -> Self {
        Self {
            border_radius: Default::default(),
            border_side: Default::default(),
            dimensions: box NoneEdgeInsetsGeometry,
            gap_padding: Default::default(),
            is_outline: Default::default(),
        }
    }
}
