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
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
    widgets::Icon,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct IconState {
    /// The current Icon path/id. Read/Write
    path: String,
}

/// A simple Icon control
/// Additional Signals: onchange
pub struct IconElement {
    component: Rc<RefCell<WidgetComponent>>,

    state: RefCell<IconState>,

    /// Emitted whenever the path/id is changed.
    /// `fn(new_path:String)`
    pub onchange: Signal<String>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for IconElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("IconElement").finish()
    }
}

impl IconElement {
    pub fn new(widget: &Icon) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // let path = widget.path.clone();
        let path = String::new();

        Self {
            component,
            state: RefCell::new(IconState { path }),
            onchange: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    //Internal
    /// The current Icon path/id. Read/Write
    pub fn path(&self) -> String {
        self.state.borrow().path.clone()
    }

    /// The current Icon path/id. Read/Write
    pub fn set_path(&self, path: String) {
        self.state.borrow_mut().path = path.clone();

        self.onchange.emit(&path);
    }
}

impl AsRef<RefCell<WidgetComponent>> for IconElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for IconElement {
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
