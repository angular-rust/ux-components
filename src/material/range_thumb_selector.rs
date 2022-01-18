use crate::ui::{Size, TextDirection};

use super::{RangeValues, Thumb};

pub trait RangeThumbSelector {
    fn thumb_selector(
        &self,
        text_direction: TextDirection,
        values: RangeValues,
        tap_value: f32,
        thumb_size: Size,
        track_size: Size,
        dx: f32,
    ) -> Thumb;
}
