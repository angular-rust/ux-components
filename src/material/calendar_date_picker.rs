// TODO: COMPLETE IT
use crate::{
    elements::Element,
    foundation::{Key, ValueChanged},
    properties::WidgetProperties,
    WidgetId,
};

pub struct CalendarDatePicker {
    pub key: Key,
    pub initial_date: DateTime,
    pub first_date: DateTime,
    pub last_date: DateTime,
    pub current_date: DateTime,
    pub on_date_changed: ValueChanged<DateTime>,
    pub on_displayed_month_changed: ValueChanged<DateTime>,
    pub initial_calendar_mode: DatePickerMode,
    pub selectable_day_predicate: SelectableDayPredicate,
}

impl Default for CalendarDatePicker {
    fn default() -> Self {
        Self {
            key: Default::default(),
            initial_date: Default::default(),
            first_date: Default::default(),
            last_date: Default::default(),
            current_date: Default::default(),
            on_date_changed: Default::default(),
            on_displayed_month_changed: Default::default(),
            initial_calendar_mode: Default::default(),
            selectable_day_predicate: Default::default(),
        }
    }
}

impl Widget for CalendarDatePicker {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create CalendarDatePickerElement");
        box CalendarDatePickerElement::new(self)
    }
}

impl WidgetProperties for CalendarDatePicker {
    fn x(&self) -> f32 {
        // self.x
        0.0
    }

    fn y(&self) -> f32 {
        // self.y
        0.0
    }

    fn w(&self) -> f32 {
        // self.w
        0.0
    }

    fn h(&self) -> f32 {
        // self.h
        0.0
    }

    fn w_min(&self) -> f32 {
        // self.w_min
        0.0
    }

    fn h_min(&self) -> f32 {
        // self.h_min
        0.0
    }

    fn w_max(&self) -> f32 {
        // self.w_max
        0.0
    }

    fn h_max(&self) -> f32 {
        // self.h_max
        0.0
    }

    fn parent(&self) -> Option<WidgetId> {
        // self.parent
        None
    }

    fn depth(&self) -> f32 {
        // self.depth
        0.0
    }

    fn visible(&self) -> bool {
        // self.visible
        true
    }

    fn mouse_input(&self) -> bool {
        // self.mouse_input
        true
    }

    fn key_input(&self) -> bool {
        // self.key_input
        true
    }

    fn renderable(&self) -> bool {
        // self.renderable
        true
    }

    fn internal_visible(&self) -> bool {
        // self.internal_visible
        true
    }
}
