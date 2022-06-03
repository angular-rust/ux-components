use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{node::Node, style};

use crate::prelude::Singleton;

use crate::{
    foundation::{MouseEvent, Signal, SliderChangeEvent},
    material::Slider,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct SliderState {
    bar_x: f32,       // = 2.0;
    bar_y: f32,       // = 2.0;
    bar_w: f32,       // = 0.0;
    bar_h: f32,       // = 0.0;
    dragging: bool,   // = false;
    ignore_set: bool, // = true;
    percent: f32,     // = 1;
    value: f32,       // = 1;
}

/// A simple slider control
/// Additional Signals: onchange
pub struct SliderElement {
    component: Rc<RefCell<WidgetComponent>>,
    state: RefCell<SliderState>,

    pub min: f32, // = 0;
    pub max: f32, // = 1;
    pub step: Option<f32>,
    pub vertical: bool, // = false;
    pub invert: bool,   // = false;

    pub onchange: Signal<SliderChangeEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for SliderElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("SliderElement").finish()
    }
}

impl SliderElement {
    pub fn new(widget: &Slider) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let max = widget.max;
        let min = widget.min;
        let value = widget.value;
        let vertical = false; // widget.vertical;
        let invert = false; // widget.invert;
        let step = None; //widget.step;

        let component = WidgetComponent::get(widget.key.id());

        // instance.update_value(value);

        Self {
            state: RefCell::new(SliderState {
                bar_h: 0.0,
                bar_w: 0.0,
                bar_x: 2.0,
                bar_y: 2.0,
                ignore_set: true,
                dragging: false,
                value,
                percent: 1.0,
            }),
            component,
            invert,
            max,
            min,
            onchange: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            step,
            vertical,
            node,
        }
    }

    #[inline]
    pub fn range(&self) -> f32 {
        self.max - self.min
    }

    #[inline]
    fn update_value(&self, value: f32) {
        let mut value = value.clamp(self.min, self.max);

        if let Some(step) = self.step {
            value = (value / step).round() * step;
        }

        if self.vertical {
            let mut state = self.state.borrow_mut();
            state.bar_w = self.w() - 4.0;
            state.bar_h = (self.h() - 4.0) * (value - self.min) / (self.max - self.min);
            state.bar_y = if !self.invert {
                (self.h() - ((self.h() - 4.0) * (value - self.min) / (self.max - self.min))) - 2.0
            } else {
                2.0
            };
            state.bar_h = state.bar_h.clamp(1.0, self.h() - 4.0);
        } else {
            let mut state = self.state.borrow_mut();
            state.bar_w = (self.w() - 4.0) * (value - self.min) / (self.max - self.min);
            state.bar_w = state.bar_w.clamp(1.0, self.w() - 4.0);
            state.bar_h = self.h() - 4.0;
            state.bar_x = if !self.invert {
                2.0
            } else {
                (self.w() - ((self.w() - 4.0) * (value - self.min) / (self.max - self.min))) - 2.0
            };
        }

        let mut state = self.state.borrow_mut();
        state.percent = value / self.range();

        state.ignore_set = true;
        state.value = value;
        state.ignore_set = false;

        self.onchange.emit(&SliderChangeEvent {
            value,
            percent: state.percent,
        });
    }

    pub fn value(&self) -> f32 {
        self.state.borrow().value
    }

    #[inline]
    pub fn set_value(&self, value: f32) {
        let mut state = self.state.borrow_mut();
        if state.ignore_set {
            state.value = value;
            return;
        }

        self.update_value(value);
    }

    #[inline]
    fn update_value_from_mouse(&self, e: &MouseEvent) {
        if !self.vertical {
            let mut dx = if !self.invert {
                e.x as f32 - self.x()
            } else {
                (self.w()) - (e.x as f32 - self.x())
            };

            if dx < 1.0 {
                dx = 1.0;
            }

            if dx >= self.w() - 4.0 {
                dx = self.w() - 4.0;
            }

            let value: f32 = ((dx - 1.0) / (self.w() - 5.0)) * self.range() + self.min;

            self.update_value(value);
        } else {
            let mut dy = if !self.invert {
                (self.h()) - (e.y as f32 - self.y())
            } else {
                e.y as f32 - self.y()
            };

            if dy < 1.0 {
                dy = 1.0;
            }
            if dy >= self.h() - 4.0 {
                dy = self.h() - 4.0;
            }

            let value: f32 = ((dy - 1.0) / (self.h() - 5.0)) * self.range() + self.min;

            self.update_value(value);
        }
    }
}

impl AsRef<RefCell<WidgetComponent>> for SliderElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for SliderElement {
    fn destroy(&self) {
        // self.base.destroy();
        self.onchange.clear();
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        // self.base.mousedown(e);

        self.state.borrow_mut().dragging = true;
        self.capture();
        self.update_value_from_mouse(e);
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        if self.state.borrow().dragging {
            self.update_value_from_mouse(e);
        } //dragging
    }

    fn mouseup(&self, e: &mut MouseEvent) {
        self.state.borrow_mut().dragging = false;
        self.uncapture();

        // self.base.mouseup(e);
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn relayout(&self, origin: Point2<f32>) {
        let update_childs = match LayoutSystem::layout(self.node) {
            Ok(layout) => {
                let mut comp = self.as_ref().borrow_mut();
                comp.x = layout.location.x + origin.x;
                comp.y = layout.location.y + origin.y;
                comp.w = layout.size.width;
                comp.h = layout.size.height;

                true
            }
            Err(e) => {
                log::error!("{}", e);
                false
            }
        };

        if update_childs {
            // self.leading.relayout();
            // self.title.relayout();
            // self.flexible_space.relayout();
        }
    }
}
