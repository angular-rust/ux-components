pub struct FocusNode {
    pub debug_label: String,
    // pub on_key: FocusOnKeyCallback,            // TODO:
    // pub on_key_event: FocusOnKeyEventCallback, // TODO:
    pub skip_traversal: bool,            // = false,
    pub can_request_focus: bool,         // = true,
    pub descendants_are_focusable: bool, // = true
}

impl Default for FocusNode {
    fn default() -> Self {
        Self {
            debug_label: Default::default(),
            // on_key: Default::default(),
            // on_key_event: Default::default(),
            skip_traversal: Default::default(),
            can_request_focus: Default::default(),
            descendants_are_focusable: Default::default(),
        }
    }
}
