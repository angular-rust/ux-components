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
    layout::LayoutSystem,
    material::Switch,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    Signal, WidgetComponent, WidgetId,
};

use super::Element;

#[derive(Debug, Clone)]
pub struct SwitchStateChangeEvent {
    value: bool,
    prev: bool,
}

/// A checkbox is a simple true or false switch.
/// Changing the state will trigger the signal.
/// Additional Signals: onchange
pub struct SwitchElement {
    component: Rc<RefCell<WidgetComponent>>,
    /// The current state. Read/Write
    state: bool, // = true;

    /// Emitted whenever state is changed.
    /// `function(new_state: bool, prev_state: bool)`
    pub onchange: Signal<SwitchStateChangeEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for SwitchElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("SwitchElement").finish()
    }
}

impl SwitchElement {
    pub fn new(widget: &Switch) -> Self {
        let id = WidgetId(0x0);

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

        self.onchange.emit(&SwitchStateChangeEvent { value, prev });
    }
}

impl AsRef<RefCell<WidgetComponent>> for SwitchElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for SwitchElement {
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
