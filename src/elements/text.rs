use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{geometry, node::Node, style};

use crate::prelude::Singleton;

use crate::{
    foundation::Signal,
    prelude::OnDemand,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
    widgets::Text,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct TextState {
    /// The text displayed by the label
    text: String,
}

/// A simple label control
/// Additional Signals: onchange
pub struct TextElement {
    component: Rc<RefCell<WidgetComponent>>,
    state: RefCell<TextState>,
    /// Emitted whenever text is changed.
    /// `fn(text:String)`
    pub onchange: Signal<String>,

    // The concrete renderer for this control instance
    pub render: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for TextElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("LabelElement").finish()
    }
}

impl TextElement {
    pub fn new(widget: &Text) -> Self {
        let node = LayoutSystem::new_node(
            style::Style {
                size: geometry::Size {
                    width: style::Dimension::Points(64.0),
                    height: style::Dimension::Points(64.0),
                },
                ..default()
            },
            vec![],
        )
        .unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // if let Some(ref onclick) = widget.onclick {
        //     component.mouse_input = true;
        //     let handler = onclick.clone();
        //     component.onmouseup.listen(box move |e| handler.emit(e));
        // }

        let text = widget.data.clone();

        Self {
            component,
            state: RefCell::new(TextState { text }),
            onchange: Signal::new(),
            render: WidgetRenderFactory::global().get::<Self>(),
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

impl AsRef<RefCell<WidgetComponent>> for TextElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for TextElement {
    fn render(&self) {
        log::info!("Render: [{}]", self.text());
        {
            let mut comp = self.component.borrow_mut();

            assert!(
                comp.destroyed == false,
                "Widget was already destroyed but is being interacted with"
            );

            if comp.renderable && comp.onrender.is_some() {
                let _ = comp.onrender.get().try_send(());
            }
        }

        if let Some(ref render) = self.render {
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
                    "Relayout TextElement [{}] {}x{} {}x{}",
                    self.text(),
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
