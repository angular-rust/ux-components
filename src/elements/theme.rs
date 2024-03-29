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
    foundation::{Helper, MouseEvent},
    material::Theme,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

/// A utility class for building Material buttons that depend on the ambient ButtonTheme and Theme
///
/// Additional Signals: none

pub struct ThemeElement {
    component: Rc<RefCell<WidgetComponent>>,

    /// The label the button displays
    pub child: Box<dyn Element>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ThemeElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ButtonElement").finish()
    }
}

impl ThemeElement {
    pub fn new(widget: &Theme) -> Self {
        let node = LayoutSystem::new_node(
            style::Style {
                padding: geometry::Rect {
                    start: style::Dimension::Points(2.0),
                    end: style::Dimension::Points(2.0),
                    top: style::Dimension::Points(2.0),
                    bottom: style::Dimension::Points(2.0),
                },
                size: geometry::Size {
                    width: style::Dimension::Percent(1.0),
                    height: style::Dimension::Percent(1.0),
                },
                ..default()
            },
            vec![],
        )
        .unwrap();

        let component = WidgetComponent::get(widget.key.id());

        // if widget.onclick.is_some() {
        //     // onmouseup.listen(options.onclick);
        // }

        let child = widget.child.create_element();
        if let Some(child) = child.node() {
            let child_style = LayoutSystem::style(child).unwrap();
            LayoutSystem::set_style(
                child,
                style::Style {
                    size: geometry::Size {
                        width: style::Dimension::Percent(1.0),
                        height: style::Dimension::Percent(1.0),
                    },
                    ..child_style
                },
            )
            .unwrap();
            LayoutSystem::set_children(node, vec![child]).unwrap()
        };

        Self {
            component,
            child,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }
}

impl AsRef<RefCell<WidgetComponent>> for ThemeElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ThemeElement {
    fn mousedown(&self, e: &mut MouseEvent) {
        // self.state.borrow_mut().mouse_down = true;
        // let component = self.component.borrow();
        // component.onmousedown.emit(e);

        log::info!("ThemeElement clicked");
        // self.state.borrow_mut().mouse_down = true;
        // let component = self.component.borrow();
        // component.onmousedown.emit(e);

        // TODO: remove this check
        let inside = {
            let comp = self.component.borrow();
            Helper::in_rect(e.x as f32, e.y as f32, comp.x, comp.y, comp.w, comp.h)
        };

        // Scaffold should used as root widget after MaterialApp
        // so all events should be inside
        if inside {
            let x = e.x as f32;
            let y = e.y as f32;

            if self.child.contains(x, y) {
                self.child.mousedown(e);
            }

            // if self.title.contains(x, y) {
            //     log::info!("Is an Title");
            //     self.title.mousedown(e);
            // }

            // if self.flexible_space.contains(x, y) {
            //     // log::info!("Is an FAB");
            //     self.flexible_space.mousedown(e);
            // }

            // for action in self.actions.iter() {
            //     if action.contains(x, y) {
            //         log::info!("Is an ACTION");
            //         action.mousedown(e);
            //     }
            // }
            // if self.drawer.contains(x, y) {
            //     // log::info!("Is an Drawer");
            //     self.drawer.mousedown(e);
            // }
        }
    }

    fn render(&self) {
        // {
        //     let comp = self.component.borrow();

        //     assert!(
        //         !comp.destroyed,
        //         "Widget was already destroyed but is being interacted with"
        //     );

        //     if comp.renderable {
        //         comp.onrender.emit(&());
        //     }
        // }

        if let Some(ref render) = self.renderer {
            render.render(self);
        }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         widget.render();
        //     }
        // }

        // center do not have a render, so we render the child
        self.child.render();
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
            let comp = self.as_ref().borrow();
            self.child.relayout(Point2 {
                x: comp.x,
                y: comp.y,
            });
        }
    }
}
