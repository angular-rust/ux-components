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
    foundation::{properties::LabelProperties, Signal},
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct LabelState {
    /// The text displayed by the label
    text: String,
}

/// A simple label control
/// Additional Signals: onchange
pub struct LabelElement {
    component: Rc<RefCell<WidgetComponent>>,
    state: RefCell<LabelState>,
    /// Emitted whenever text is changed.
    /// `fn(text:String)`
    pub onchange: Signal<String>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for LabelElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("LabelElement").finish()
    }
}

impl LabelElement {
    pub fn new(widget: &LabelProperties) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // if let Some(ref onclick) = widget.onclick {
        //     component.mouse_input = true;
        //     // let handler = onclick.clone();
        //     // component.onmouseup.listen(box move |e| handler.emit(e));
        // }

        let text = widget.text.clone();

        Self {
            component,
            state: RefCell::new(LabelState { text }),
            onchange: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    //Internal

    /// The text displayed by the label
    pub fn text(&self) -> String {
        self.state.borrow().text.clone()
    }

    /// The text displayed by the label
    pub fn set_text(&self, str: String) {
        self.state.borrow_mut().text = str.clone();

        self.onchange.emit(&str);
    }
}

impl AsRef<RefCell<WidgetComponent>> for LabelElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for LabelElement {
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
