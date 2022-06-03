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
        properties::{LabelProperties, ListProperties},
        DropdownSelectEvent, MouseButton, MouseEvent, Signal,
    },
    material::DropdownButton,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, LabelElement, ListElement, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
pub struct DropdownButtonState {
    is_open: bool,       // = false;
    skip_mouse_up: bool, // = false;
}

/// A simple dropdown control that can contain any other controls
/// Additional signals: onselect
pub struct DropdownButtonElement {
    component: Rc<RefCell<WidgetComponent>>,
    pub list: ListElement,
    pub label: LabelElement,

    pub onselect: Signal<DropdownSelectEvent>,

    pub state: RefCell<DropdownButtonState>,
    height: f32, // = 110;

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for DropdownButtonElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DropdownElement").finish()
    }
}

impl DropdownButtonElement {
    pub fn new<T: Default>(widget: &DropdownButton<T>) -> Self {
        let id = widget.key.id();
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        //create the base control
        let component = WidgetComponent::get(widget.key.id());

        let height: f32 = 110.0;

        let list = ListElement::new(&ListProperties {
            parent: Some(id),
            x: 0.0,
            // y: widget.h + 1.0,
            // w: component.w - 1.0,
            h: height,
            // options: options.options.list,
            // internal_visible: widget.visible,
            ..Default::default()
        });

        // list.onselect.listen(onselected);
        // list.component.onmousedown.listen(ondeselect);

        let label = LabelElement::new(&LabelProperties {
            parent: Some(id),
            x: 5.0,
            y: 0.0,
            // w: component.w - 10.0,
            // h: component.h,
            // options: options.options.label,
            // internal_visible: widget.visible,
            // text: widget.text.clone(),
            // text_size: widget.text_size,
            // align: widget.align,
            // align_vertical: widget.align_vertical,
            ..Default::default()
        });

        list.set_visible(false);

        Self {
            height: 110.0,
            component,
            label,
            list,
            onselect: Signal::new(),
            state: Default::default(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            node,
        }
    }

    fn ondeselect(&self, e: &MouseEvent, c: &dyn Element) {
        if !self.list.contains(e.x as f32, e.y as f32) {
            self.close_list();
        }
    }

    fn onselected(&self, idx: usize, c: &dyn Element, e: &MouseEvent) {
        self.onselect.emit(&DropdownSelectEvent {
            index: Some(idx),
            component: c.id(),
            mouse_event: e.clone(),
        });
        self.close_list();
    }

    // offset_x: f32 = 0.0, offset_y: f32 = 0.0
    pub fn add_item(&self, item: &dyn Element, offset_x: f32, offset_y: f32) {
        self.list.add_item(item, offset_x, offset_y);
        self.list.set_visible(self.state.borrow().is_open);
    }

    pub fn close_list(&self) {
        self.list.uncapture();
        self.list.set_visible(false);

        //since removed by bring to front, readd
        self.list.set_x(0.0);
        self.list.set_y(self.h() + 1.0);
        self.add(&self.list);

        self.state.borrow_mut().is_open = false;
    }

    pub fn open_list(&self) {
        self.list.set_visible(true);

        //bring above everything
        let component = self.component.borrow();
        if let Some(canvas) = component.canvas.as_ref() {
            canvas.bring_to_front(&self.list);
        }
        //move to absolute space
        self.list.set_x(self.x());
        self.list.set_y(self.y() + self.h() + 1.0);

        self.list.capture();

        self.state.borrow_mut().is_open = true;
    }
}

impl AsRef<RefCell<WidgetComponent>> for DropdownButtonElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for DropdownButtonElement {
    fn destroy(&self) {
        // self.base.destroy();

        self.onselect.clear();
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        // self.base.mousedown(e);

        if e.button == MouseButton::Left {
            let _inside = self.contains(e.x as f32, e.y as f32);
            if _inside {
                let mut state = self.state.borrow_mut();
                if !state.is_open {
                    self.open_list();
                    state.skip_mouse_up = true;
                } else {
                    self.close_list();
                }
            }
        } //mouse left
    }

    fn mouseup(&self, e: &mut MouseEvent) {
        // self.base.mouseup(e);

        if e.button == MouseButton::Left {
            let mut state = self.state.borrow_mut();
            if state.is_open && !state.skip_mouse_up {
                self.close_list();
                return;
            }

            state.skip_mouse_up = false;
        } //mouse left
    }

    // _dx: f32 =0.0, _dy: f32 =0.0, _dw: f32 =0.0, _dh: f32 =0.0
    fn bounds_changed(&self, _dx: f32, _dy: f32, _dw: f32, _dh: f32) {
        // self.base.bounds_changed(_dx, _dy, _dw, _dh);

        if self.state.borrow().is_open {
            self.list.set_pos(self.x(), self.y() + self.h() + 1.0);
        }
        self.list.set_size(self.w(), self.list.h());

        self.label.set_size(self.w() - 1.0, self.h());
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
