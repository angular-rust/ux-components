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
    foundation::{
        properties::ListProperties, Id, ListItemEnterEvent, ListItemLeaveEvent,
        ListItemSelectEvent, MouseEvent, Signal,
    },
    material::Scrollable,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, ScrollableElement, WidgetComponent};

pub struct ListElement {
    component: Rc<RefCell<WidgetComponent>>,
    pub view: ScrollableElement,
    pub items: Rc<RefCell<Vec<Id>>>,

    pub onselect: Signal<ListItemSelectEvent>,
    pub onitementer: Signal<ListItemEnterEvent>,
    pub onitemleave: Signal<ListItemLeaveEvent>,

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ListElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ListElement").finish()
    }
}

impl ListElement {
    pub fn new(widget: &ListProperties) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        let view = ScrollableElement::new(&Scrollable {
            // parent: Some(id),
            // x: 0.0,
            // y: 0.0,
            // w: component.w,
            // h: component.h,
            // // options: options.options.view,
            // internal_visible: options.visible,
            ..Default::default()
        });

        Self {
            component,
            items: Rc::new(RefCell::new(Vec::new())),
            onitementer: Signal::new(),
            onitemleave: Signal::new(),
            onselect: Signal::new(),
            view,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    // offset_x: f32 = 0.0, offset_y: f32 = 0.0
    pub fn add_item(&self, item: &dyn Element, offset_x: f32, offset_y: f32) {
        let childbounds = &self.view.container.children_bounds();

        item.set_y_local(item.y_local() + (childbounds.y_local + childbounds.h) + offset_y);
        item.set_x_local(item.x_local() + offset_x);

        self.view.add(item);

        // item.mouse_input = true;
        let mut items = self.items.borrow_mut();
        items.push(item.id());

        // item.onmouseup.listen(box |e| {
        //     // self.item_mouseup(e, item.id())
        // });
        // item.onmouseenter.listen(box |e| {
        //     // self.item_mouseenter(e, item.id())
        // });
        // item.onmouseleave.listen(box |e| {
        //     // self.item_mouseleave(e, item.id())
        // });
    }

    fn item_mouseenter(&self, event: &MouseEvent, ctrl: Id) {
        let items = self.items.borrow();
        match items.iter().position(|w| w == &ctrl) {
            Some(idx) => self.onitementer.emit(&ListItemEnterEvent {
                index: Some(idx),
                component: Some(ctrl),
                mouse_event: Some(event.clone()),
            }),
            None => {
                log::info!("MouseEnter not found");
            }
        }
    }

    fn item_mouseleave(&self, event: &MouseEvent, ctrl: Id) {
        let items = self.items.borrow();
        match items.iter().position(|w| w == &ctrl) {
            Some(idx) => self.onitemleave.emit(&ListItemLeaveEvent {
                index: Some(idx),
                component: Some(ctrl),
                mouse_event: Some(event.clone()),
            }),
            None => {
                log::info!("MouseLeave not found");
            }
        }
    }

    fn item_mouseup(&self, event: &MouseEvent, ctrl: Id) {
        let items = self.items.borrow();
        match items.iter().position(|w| w == &ctrl) {
            Some(idx) => self.onselect.emit(&ListItemSelectEvent {
                index: Some(idx),
                component: Some(ctrl),
                mouse_event: Some(event.clone()),
            }),
            None => {
                log::info!("MouseUp not fund");
            }
        }
    }

    pub fn clear(&mut self) {
        let mut items = self.items.borrow_mut();
        while let Some(id) = items.pop() {
            // if let Some(widget) = id.widget() {
            //     widget.destroy()
            // }
        }

        self.view.refresh_scroll();
        self.onselect.emit(&ListItemSelectEvent {
            index: None,
            component: None,
            mouse_event: None,
        });
    }
}

impl AsRef<RefCell<WidgetComponent>> for ListElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for ListElement {
    fn destroy(&self) {
        // self.base.destroy();

        self.onselect.clear();
        self.onitementer.clear();
        self.onitemleave.clear();
    }

    // _dx: f32 =0.0, _dy: f32 =0.0, _dw: f32 =0.0, _dh: f32 =0.0
    fn bounds_changed(&self, dx: f32, dy: f32, dw: f32, dh: f32) {
        // self.base.bounds_changed(_dx, _dy, _dw, _dh);

        // if self.view.is_some() {
        self.view.set_size(self.w(), self.h());
        // }
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
