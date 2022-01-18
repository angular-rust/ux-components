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
    foundation::Signal,
    material::Chip,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Debug, Clone)]
pub struct ChipStateChangeEvent {
    value: bool,
    prev: bool,
}

/// A checkbox is a simple true or false switch.
/// Changing the state will trigger the signal.
/// Additional Signals: onchange
pub struct ChipElement {
    component: Rc<RefCell<WidgetComponent>>,
    /// The current state. Read/Write
    state: bool, // = true;

    /// Emitted whenever state is changed.
    /// `function(new_state: bool, prev_state: bool)`
    pub onchange: Signal<ChipStateChangeEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ChipElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("CheckboxElement").finish()
    }
}

impl ChipElement {
    pub fn new(widget: &Chip) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // let state = widget.state;
        let state = true;

        // onmouseup.listen(onclick);

        // let onchange = widget.onchange.clone();
        // let onchange = Signal::new();

        let instance = Self {
            component,
            onchange: Signal::new(),
            state,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        };

        // if let Some(signal) = onchange {
        //     instance.onchange.listen(box move |e| signal.emit(e));
        // }

        instance
    }

    //Internal

    pub fn onclick<T>(&mut self, _: T, _: T) {
        self.state = !self.state;
    }

    /// The current state. Read/Write
    pub fn state(&self) -> bool {
        self.state
    }

    /// The current state. Read/Write
    pub fn set_state(&mut self, value: bool) {
        let prev = self.state;

        self.state = value;

        self.onchange.emit(&ChipStateChangeEvent { value, prev });
    }
}

impl AsRef<RefCell<WidgetComponent>> for ChipElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ChipElement {
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

                log::warn!(
                    "Relayout ChipElement {}x{} {}x{}",
                    comp.x,
                    comp.y,
                    comp.w,
                    comp.h
                );
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
