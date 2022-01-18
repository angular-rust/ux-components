use crate::painting::BorderSide;

// paint(Canvas canvas, Rect rect, {double? gapStart, double gapExtent = 0.0, double gapPercentage = 0.0, TextDirection? textDirection}) -> void
// Paint this input border on canvas.

pub struct InputBorder {
    pub border_side: BorderSide,
}

impl Default for InputBorder {
    fn default() -> Self {
        Self {
            border_side: Default::default(),
        }
    }
}
