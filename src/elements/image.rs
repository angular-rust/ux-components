use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{node::Node, style};

use crate::prelude::{OnDemand, Singleton};

use crate::{
    foundation::Signal,
    painting::{ImageInfo, ImageStreamListener},
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
    widgets::Image,
    engine::d2,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct ImageState {
    /// The current image path/id. Read/Write
    path: String,
}

/// A simple image control
/// Additional Signals: onchange
pub struct ImageElement {
    component: Rc<RefCell<WidgetComponent>>,

    state: RefCell<ImageState>,

    pub image: Option<d2::Image>,
    /// Emitted whenever the path/id is changed.
    /// `fn(new_path:String)`
    pub onchange: Signal<String>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ImageElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ImageElement").finish()
    }
}

impl ImageElement {
    pub fn new(widget: &Image) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        let stream = widget.image.create_stream(Default::default());

        let image = {
            let container: Rc<RefCell<Option<d2::Image>>> = Rc::new(RefCell::new(None));
            let image = container.clone();
            stream.add_listener(ImageStreamListener::new(
                box move |info: ImageInfo, synchronous_call| {
                    image.replace(Some(info.image));
                },
                None,
                None,
            ));
            container.take()
        };

        // let path = widget.path.clone();
        let path = String::new();

        Self {
            component,
            state: RefCell::new(ImageState { path }),
            image,
            onchange: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    //Internal
    /// The current image path/id. Read/Write
    pub fn path(&self) -> String {
        self.state.borrow().path.clone()
    }

    /// The current image path/id. Read/Write
    pub fn set_path(&self, path: String) {
        self.state.borrow_mut().path = path.clone();

        self.onchange.emit(&path);
    }
}

impl AsRef<RefCell<WidgetComponent>> for ImageElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ImageElement {
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
