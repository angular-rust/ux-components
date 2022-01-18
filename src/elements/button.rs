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
    foundation::properties::{ButtonProperties, LabelProperties},
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, LabelElement, WidgetComponent};

/// A simple button with a label
/// Additional Signals: none

pub struct ButtonElement {
    component: Rc<RefCell<WidgetComponent>>,
    /// The label the button displays
    pub label: LabelElement,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,

    // The node in layout system
    pub node: Node,
}

impl Debug for ButtonElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ButtonElement").finish()
    }
}

impl ButtonElement {
    pub fn new(widget: &ButtonProperties) -> Self {
        let id = widget.key.id();

        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        let label = LabelElement::new(&LabelProperties {
            parent: Some(id),
            x: 0.0,
            y: 0.0,
            // w: component.w,
            // h: component.h,
            // options: options.options.label,
            mouse_input: false,
            internal_visible: widget.visible,
            text: widget.text.clone(),
            text_size: widget.text_size,
            align: widget.align,
            align_vertical: widget.align_vertical,
            ..Default::default()
        });

        if widget.onclick.is_some() {
            // onmouseup.listen(options.onclick);
        }

        Self {
            component,
            label,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }
}

impl AsRef<RefCell<WidgetComponent>> for ButtonElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ButtonElement {
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
                    "Relayout ButtonElement {}x{} {}x{}",
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
