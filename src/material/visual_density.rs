// copyWith({double? horizontal, double? vertical}) → VisualDensity
// Copy the current VisualDensity with the given values replacing the current values.
//
// debugFillProperties(DiagnosticPropertiesBuilder properties) → void
// Add additional properties associated with the node.
// override
//
// effectiveConstraints(BoxConstraints constraints) → BoxConstraints
// Return a copy of constraints whose minimum width and height have been updated with the baseSizeAdjustment.
//
// toStringShort() → String
// A brief description of this object, usually just the runtimeType and the hashCode.
// override

pub struct VisualDensity {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for VisualDensity {
    fn default() -> Self {
        Self {
            horizontal: Default::default(),
            vertical: Default::default(),
        }
    }
}
