pub struct DatePickerDialog {
    pub key: Key,
    pub initial_date: DateTime,
    pub first_date: DateTime,
    pub last_date: DateTime,
    pub current_date: DateTime,
    pub initial_entry_mode: DatePickerEntryMode,
    pub selectable_day_predicate: SelectableDayPredicate,
    pub cancel_text: String,
    pub confirm_text: String,
    pub help_text: String,
    pub initial_calendar_mode: DatePickerMode,
    pub error_format_text: String,
    pub error_invalid_text: String,
    pub field_hint_text: String,
    pub field_label_text: String,
    pub restoration_id: String,
}

impl Default for DatePickerDialog {
    fn default() -> Self {
        Self {
            key: Default::default(),
            initial_date: Default::default(),
            first_date: Default::default(),
            last_date: Default::default(),
            current_date: Default::default(),
            initial_entry_mode: Default::default(),
            selectable_day_predicate: Default::default(),
            cancel_text: Default::default(),
            confirm_text: Default::default(),
            help_text: Default::default(),
            initial_calendar_mode: Default::default(),
            error_format_text: Default::default(),
            error_invalid_text: Default::default(),
            field_hint_text: Default::default(),
            field_label_text: Default::default(),
            restoration_id: Default::default(),
        }
    }
}

impl Widget for DatePickerDialog {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create DatePickerDialogElement");
        box DatePickerDialogElement::new(self)
    }
}

impl WidgetProperties for DatePickerDialog {
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
