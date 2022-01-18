// getPreferredSize(bool isEnabled, bool isDiscrete) -> Size
// Returns the preferred size of the shape, based on the given conditions.
//
// paint(PaintingContext context, Offset center, {required Animation<double> activationAnimation, required Animation<double> enableAnimation, required bool isDiscrete, required TextPainter labelPainter, required RenderBox parentBox, required SliderThemeData sliderTheme, required TextDirection textDirection, required double value, required double textScaleFactor, required Size sizeWithOverflow}) -> void
// Paints the shape, taking into account the state passed to it.

pub struct RangeSliderValueIndicatorShape;

impl Default for RangeSliderValueIndicatorShape {
    fn default() -> Self {
        Self {}
    }
}
