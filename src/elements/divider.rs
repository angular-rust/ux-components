use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{geometry, node::Node, style};

use crate::prelude::{OnDemand, Singleton, Color};

use crate::{
    foundation::Signal,
    material::Divider,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Debug, Clone)]
pub struct DividerStateChangeEvent {
    value: bool,
    prev: bool,
}

/// A checkbox is a simple true or false switch.
/// Changing the state will trigger the signal.
/// Additional Signals: onchange
pub struct DividerElement {
    component: Rc<RefCell<WidgetComponent>>,
    /// The current state. Read/Write
    state: bool, // = true;

    pub height: f32,
    pub thickness: f32,
    pub indent: f32,
    pub end_indent: f32,
    pub color: Color,

    /// Emitted whenever state is changed.
    /// `function(new_state: bool, prev_state: bool)`
    pub onchange: Signal<DividerStateChangeEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for DividerElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DividerElement").finish()
    }
}

impl DividerElement {
    pub fn new(widget: &Divider) -> Self {
        let thickness = if widget.thickness != 0.0 {
            widget.thickness
        } else {
            1.0
        };

        let height = if widget.height != 0.0 {
            widget.height
        } else {
            thickness
        };

        let node = LayoutSystem::new_node(
            style::Style {
                size: geometry::Size {
                    width: style::Dimension::Percent(1.0),
                    height: style::Dimension::Points(height),
                },
                min_size: geometry::Size {
                    width: style::Dimension::Percent(1.0),
                    height: style::Dimension::Points(height),
                },
                ..default()
            },
            vec![],
        )
        .unwrap();

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
            height,
            thickness,
            indent: widget.indent,
            end_indent: widget.end_indent,
            color: widget.color,
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

        self.onchange.emit(&DividerStateChangeEvent { value, prev });
    }
}

impl AsRef<RefCell<WidgetComponent>> for DividerElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for DividerElement {
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

    fn render(&self) {
        {
            let mut comp = self.component.borrow_mut();

            assert!(
                !comp.destroyed,
                "Widget was already destroyed but is being interacted with"
            );

            if comp.renderable && comp.onrender.is_some() {
                let _ = comp.onrender.get().try_send(());
            }
        }

        if let Some(ref render) = self.renderer {
            render.render(self);
        }

        {
            // let comp = self.component.borrow();
            // for child in comp.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         widget.render();
            //     }
            // }
        }
    }
}
