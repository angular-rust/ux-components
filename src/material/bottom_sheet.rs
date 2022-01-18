use crate::{
    elements::{BottomSheetElement, Element},
    foundation::{colorspace::Color, Id, Key, WidgetProperties},
    painting::ShapeBorder,
    rendering::BoxConstraints,
    ui::{Clip, VoidCallback},
    widgets::{Widget, WidgetBuilder},
};

pub struct BottomSheet {
    pub key: Key,
    // pub animation_controller: AnimationController,
    pub enable_drag: bool,
    // pub on_drag_start: BottomSheetDragStartHandler,
    // pub on_drag_end: BottomSheetDragEndHandler,
    pub background_color: Color,
    pub elevation: f32,
    pub shape: ShapeBorder,
    pub clip_behavior: Clip,
    pub constraints: BoxConstraints,
    pub on_closing: Option<Box<dyn VoidCallback>>,
    pub builder: Option<Box<dyn WidgetBuilder>>,
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self {
            key: Default::default(),
            // animation_controller: Default::default(),
            enable_drag: Default::default(),
            // on_drag_start: Default::default(),
            // on_drag_end: Default::default(),
            background_color: Default::default(),
            elevation: Default::default(),
            shape: Default::default(),
            clip_behavior: Default::default(),
            constraints: Default::default(),
            on_closing: Default::default(),
            builder: Default::default(),
        }
    }
}

impl Widget for BottomSheet {
    fn create_element(&self) -> Box<dyn Element> {
        log::info!("Create BottomSheetElement");
        box BottomSheetElement::new(self)
    }
}

impl WidgetProperties for BottomSheet {
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
