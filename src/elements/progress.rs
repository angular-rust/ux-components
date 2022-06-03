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
    foundation::{ProgressChangeEvent, Signal},
    material::ProgressIndicator,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

/// A simple progress control, ranging from 0 to 1.
/// Additional Signals: onchange
pub struct ProgressIndicatorElement {
    component: Rc<RefCell<WidgetComponent>>,
    progress: f32, // = 0.5;
    pub onchange: Signal<ProgressChangeEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ProgressIndicatorElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ProgressElement").finish()
    }
}

impl ProgressIndicatorElement {
    pub fn new(widget: &ProgressIndicator) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        let progress = widget.value;

        Self {
            component,
            onchange: Signal::new(),
            progress,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn set_progress(&mut self, value: f32) -> f32 {
        let prev = self.progress;

        let value = value.clamp(0.0, 1.0);

        self.progress = value;

        self.onchange.emit(&ProgressChangeEvent { value, prev });

        self.progress
    }
}

impl AsRef<RefCell<WidgetComponent>> for ProgressIndicatorElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ProgressIndicatorElement {
    fn destroy(&self) {
        // self.base.destroy();

        self.onchange.clear();
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
