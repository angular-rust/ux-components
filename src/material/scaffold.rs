use crate::{
    elements::{Element, ScaffoldElement},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    gestures::DragStartBehavior,
    widgets::{NullWidget, PreferredSizeWidget, Widget},
};

use super::{DrawerCallback, FloatingActionButtonAnimator, FloatingActionButtonLocation};

pub struct Scaffold {
    pub key: Key,
    pub app_bar: Box<dyn PreferredSizeWidget>,
    pub body: Box<dyn Widget>,
    pub floating_action_button: Box<dyn Widget>,
    pub floating_action_button_location: FloatingActionButtonLocation,
    pub floating_action_button_animator: FloatingActionButtonAnimator,
    pub persistent_footer_buttons: Vec<Box<dyn Widget>>,
    pub drawer: Box<dyn Widget>,
    pub on_drawer_changed: Option<Box<dyn DrawerCallback>>,
    pub end_drawer: Box<dyn Widget>,
    pub on_end_drawer_changed: Option<Box<dyn DrawerCallback>>,
    pub bottom_navigation_bar: Box<dyn Widget>,
    pub bottom_sheet: Box<dyn Widget>,
    pub background_color: Color,
    pub resize_to_avoid_bottom_inset: bool,
    pub primary: bool,
    pub drawer_drag_start_behavior: DragStartBehavior,
    pub extend_body: bool,
    pub extend_body_behind_app_bar: bool,
    pub drawer_scrim_color: Color,
    pub drawer_edge_drag_width: f32,
    pub drawer_enable_open_drag_gesture: bool,
    pub end_drawer_enable_open_drag_gesture: bool,
    pub restoration_id: String,
}

impl Default for Scaffold {
    fn default() -> Self {
        Self {
            key: Default::default(),
            app_bar: box NullWidget,
            body: box NullWidget,
            floating_action_button: box NullWidget,
            floating_action_button_location: Default::default(),
            floating_action_button_animator: Default::default(),
            persistent_footer_buttons: Default::default(),
            drawer: box NullWidget,
            on_drawer_changed: Default::default(),
            end_drawer: box NullWidget,
            on_end_drawer_changed: Default::default(),
            bottom_navigation_bar: box NullWidget,
            bottom_sheet: box NullWidget,
            background_color: Default::default(),
            resize_to_avoid_bottom_inset: Default::default(),
            primary: Default::default(),
            drawer_drag_start_behavior: Default::default(),
            extend_body: Default::default(),
            extend_body_behind_app_bar: Default::default(),
            drawer_scrim_color: Default::default(),
            drawer_edge_drag_width: Default::default(),
            drawer_enable_open_drag_gesture: Default::default(),
            end_drawer_enable_open_drag_gesture: Default::default(),
            restoration_id: Default::default(),
        }
    }
}

impl Widget for Scaffold {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create ScaffoldElement");
        box ScaffoldElement::new(self)
    }
}

impl WidgetProperties for Scaffold {
    fn key(&self) -> &Key {
        &self.key
    }

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

    fn parent(&self) -> Option<Id> {
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
