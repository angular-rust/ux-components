use crate::{foundation::colorspace::Color, painting::TextStyle};

use super::{
    RangeSliderThumbShape, RangeSliderTickMarkShape, RangeSliderTrackShape,
    RangeSliderValueIndicatorShape, RangeThumbSelector, ShowValueIndicator, SliderComponentShape,
    SliderTickMarkShape, SliderTrackShape,
};

pub struct SliderThemeData {
    pub track_height: f32,
    pub active_track_color: Color,
    pub inactive_track_color: Color,
    pub disabled_active_track_color: Color,
    pub disabled_inactive_track_color: Color,
    pub active_tick_mark_color: Color,
    pub inactive_tick_mark_color: Color,
    pub disabled_active_tick_mark_color: Color,
    pub disabled_inactive_tick_mark_color: Color,
    pub thumb_color: Color,
    pub overlapping_shape_stroke_color: Color,
    pub disabled_thumb_color: Color,
    pub overlay_color: Color,
    pub value_indicator_color: Color,
    pub overlay_shape: SliderComponentShape,
    pub tick_mark_shape: SliderTickMarkShape,
    pub thumb_shape: SliderComponentShape,
    pub track_shape: SliderTrackShape,
    pub value_indicator_shape: SliderComponentShape,
    pub range_tick_mark_shape: RangeSliderTickMarkShape,
    pub range_thumb_shape: RangeSliderThumbShape,
    pub range_track_shape: RangeSliderTrackShape,
    pub range_value_indicator_shape: RangeSliderValueIndicatorShape,
    pub show_value_indicator: ShowValueIndicator,
    pub value_indicator_text_style: TextStyle,
    pub min_thumb_separation: f32,
    pub thumb_selector: Option<Box<dyn RangeThumbSelector>>,
}

impl Default for SliderThemeData {
    fn default() -> Self {
        Self {
            track_height: Default::default(),
            active_track_color: Default::default(),
            inactive_track_color: Default::default(),
            disabled_active_track_color: Default::default(),
            disabled_inactive_track_color: Default::default(),
            active_tick_mark_color: Default::default(),
            inactive_tick_mark_color: Default::default(),
            disabled_active_tick_mark_color: Default::default(),
            disabled_inactive_tick_mark_color: Default::default(),
            thumb_color: Default::default(),
            overlapping_shape_stroke_color: Default::default(),
            disabled_thumb_color: Default::default(),
            overlay_color: Default::default(),
            value_indicator_color: Default::default(),
            overlay_shape: Default::default(),
            tick_mark_shape: Default::default(),
            thumb_shape: Default::default(),
            track_shape: Default::default(),
            value_indicator_shape: Default::default(),
            range_tick_mark_shape: Default::default(),
            range_thumb_shape: Default::default(),
            range_track_shape: Default::default(),
            range_value_indicator_shape: Default::default(),
            show_value_indicator: Default::default(),
            value_indicator_text_style: Default::default(),
            min_thumb_separation: Default::default(),
            thumb_selector: Default::default(),
        }
    }
}
