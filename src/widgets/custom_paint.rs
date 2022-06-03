use crate::{
    elements::{CustomPaintElement, Element},
    foundation::{Id, Key, WidgetProperties},
    widgets::Widget, rendering::{CustomPainter, NoneCustomPainter}, ui::Size,
};

use super::NoneWidget;

pub struct CustomPaint {
    // The widget below this widget in the tree.
    pub child: Box<dyn Widget>,
    
    // The painter that paints after the children.
    pub foreground_painter: Box<dyn CustomPainter>,
    
    // Whether the painting is complex enough to benefit from caching.
    pub is_complex: bool,
    
    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // The painter that paints before the children.
    pub painter: Box<dyn CustomPainter>,
    
    // The size that this CustomPaint should aim for, given the layout constraints, if there is no child.
    pub size: Size,
    
    // Whether the raster cache should be told that this painting is likely to change in the next frame.
    pub will_change: bool,
}

impl Default for CustomPaint {
    fn default() -> Self {
        Self {
            child: box NoneWidget,
            foreground_painter: box NoneCustomPainter,
            is_complex: Default::default(),
            key: Default::default(),
            painter: box NoneCustomPainter,
            size: Default::default(),
            will_change: Default::default(),
        }
    }
}

impl Widget for CustomPaint {
    fn create_element(&self) -> Box<dyn Element> {
        box CustomPaintElement::new(self)
    }
}

impl WidgetProperties for CustomPaint {
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
