// constrain(Size size) → Size
// Returns the size that both satisfies the constraints and is as close as possible to the given size.
//
// constrainDimensions(double width, double height) → Size
// Returns the size that both satisfies the constraints and is as close as possible to the given width and height.
//
// constrainHeight([double height = double.infinity]) → double
// Returns the height that both satisfies the constraints and is as close as possible to the given height.
//
// constrainSizeAndAttemptToPreserveAspectRatio(Size size) → Size
// Returns a size that attempts to meet the following conditions, in order:
//
// constrainWidth([double width = double.infinity]) → double
// Returns the width that both satisfies the constraints and is as close as possible to the given width.
//
// copyWith({double? minWidth, double? maxWidth, double? minHeight, double? maxHeight}) → BoxConstraints
// Creates a copy of this box constraints but with the given fields replaced with the new values.
//
// debugAssertIsValid({bool isAppliedConstraint = false, InformationCollector? informationCollector}) → bool
// Asserts that the constraints are valid.
// override
//
// deflate(EdgeInsets edges) → BoxConstraints
// Returns new box constraints that are smaller by the given edge dimensions.
//
// enforce(BoxConstraints constraints) → BoxConstraints
// Returns new box constraints that respect the given constraints while being as close as possible to the original constraints.
//
// heightConstraints() → BoxConstraints
// Returns box constraints with the same height constraints but with unconstrained width.
//
// isSatisfiedBy(Size size) → bool
// Whether the given size satisfies the constraints.
//
// loosen() → BoxConstraints
// Returns new box constraints that remove the minimum width and height requirements.
//
// normalize() → BoxConstraints
// Returns a box constraints that isNormalized.
//
// tighten({double? width, double? height}) → BoxConstraints
// Returns new box constraints with a tight width and/or height as close to the given width and height as possible while still respecting the original box constraints.
//
// toString() → String
// A string representation of this object.
// override
//
// widthConstraints() → BoxConstraints
// Returns box constraints with the same width constraints but with unconstrained height.
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for BoxConstraints {
    fn default() -> Self {
        Self {
            min_width: Default::default(),
            max_width: Default::default(),
            min_height: Default::default(),
            max_height: Default::default(),
        }
    }
}
