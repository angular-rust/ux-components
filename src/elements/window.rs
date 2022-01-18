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
        properties::{LabelProperties, PanelProperties, WindowProperties},
        MouseEvent, Signal, TextAlign,
    },
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, LabelElement, PanelElement, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
struct WindowState {
    dragging: bool,
    drag_x: f32,
    drag_y: f32,

    resizing: bool, // = false;
    resize_x: f32,  // = 0.0;
    resize_y: f32,  // = 0.0;
}

// FIXME: Promote the SetterMut trait for AsMut.

pub struct WindowElement {
    component: Rc<RefCell<WidgetComponent>>,
    pub title: Option<LabelElement>,
    pub close_handle: Option<PanelElement>,
    pub resize_handle: Option<PanelElement>,
    pub collapse_handle: Option<PanelElement>,

    pub closable: bool,
    pub focusable: bool,
    pub moveable: bool,
    pub resizable: bool,
    pub collapsible: bool,

    pub onclose: Signal<()>,
    pub oncollapse: Signal<bool>,

    state: RefCell<WindowState>,

    title_margin_top: f32,
    title_margin_left: f32,
    title_margin_right: f32,
    title_height: f32,

    ready: bool, // = false;

    collapsed: bool,  // = false;
    pre_h: f32,       // = 0.0;
    pre_h_min: f32,   // = 0.0;
    pre_resize: bool, // = true;

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for WindowElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("WindowElement").finish()
    }
}

impl WindowElement {
    pub fn new(widget: &WindowProperties) -> Self {
        let id = widget.key.id();

        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let moveable = widget.moveable;
        let resizable = widget.resizable;
        let closable = widget.closable;
        let focusable = widget.focusable;
        let collapsible = widget.collapsible;
        let title_height = widget.title_height;
        let title_margin_left = widget.title_margin_left;
        let title_margin_top = widget.title_margin_top;
        let title_margin_right = widget.title_margin_right;

        let component = WidgetComponent::get(widget.key.id());

        let resize_handle = Some(PanelElement::new(&PanelProperties {
            // x: component.w - 24.0,
            // y: component.h - 24.0,
            w: 24.0,
            h: 24.0,
            visible: widget.visible,
            parent: Some(id),
            ..Default::default()
        }));

        // resize_handle.mouse_input = resizable;
        // resize_handle.onmousedown.listen(on_resize_down);
        // resize_handle.onmouseup.listen(on_resize_up);
        // DV

        let title = Some(LabelElement::new(&LabelProperties {
            parent: Some(id),

            x: title_margin_left,
            y: title_margin_top,

            // w: component.w - title_margin_right,
            h: title_height,

            internal_visible: widget.visible,
            // options: options.options.label, // DV
            text: widget.title.clone(),
            align: TextAlign::Center,
            align_vertical: TextAlign::Center,
            text_size: widget.text_size,
            ..Default::default()
        }));

        let close_handle = Some(PanelElement::new(&PanelProperties {
            // x: component.w - title_margin_right - 24.0,
            y: title_margin_top,
            w: 22.0,
            h: title_height,
            visible: widget.visible,
            parent: Some(id),
            ..Default::default()
        }));

        let collapse_handle = Some(PanelElement::new(&PanelProperties {
            // x: if closable {
            //     component.w - title_margin_right - 48.0
            // } else {
            //     component.w - title_margin_right - 24.0
            // },
            y: title_margin_top,
            w: 22.0,
            h: title_height,
            visible: widget.visible,
            parent: Some(id),
            // options: options.options.collapse_handle,
            ..Default::default()
        }));

        // close_handle.mouse_input = closable;

        // if !closable {
        //     close_handle.visible = false;
        // } else {
        //     close_handle.onmousedown.listen(on_close);
        // }

        // collapse_handle.mouse_input = collapsible;

        // if !collapsible {
        //     collapse_handle.visible = false;
        // } else {
        //     collapse_handle.onmousedown.listen(on_collapse);
        // }

        Self {
            closable,
            close_handle,
            collapse_handle,
            collapsed: false,
            collapsible,
            component,
            state: Default::default(),
            focusable,
            moveable,
            onclose: Signal::new(),
            oncollapse: Signal::new(),
            pre_h: 0.0,
            pre_h_min: 0.0,
            pre_resize: true,
            ready: true,
            renderer: WidgetRenderFactory::global().get::<Self>(),
            resizable,
            resize_handle,
            title,
            title_height,
            title_margin_left,
            title_margin_right,
            title_margin_top,
            node,
        }
    }

    fn on_resize_up(&self, e: &mut MouseEvent, _: f32) {
        if !self.resizable {
            return;
        }

        if self.collapsed {
            return;
        }

        self.state.borrow_mut().resizing = false;
        self.resize_handle
            .as_ref()
            .map(|ref handle| handle.uncapture());
    }

    fn on_resize_down(&mut self, e: &mut MouseEvent, _: f32) {
        if !self.resizable {
            return;
        }

        if self.collapsed {
            return;
        }

        let mut state = self.state.borrow_mut();
        if state.resizing {
            return;
        }

        state.resizing = true;
        state.resize_x = e.x as f32;
        state.resize_y = e.y as f32;
        self.resize_handle.as_ref().map(|handle| handle.capture());
        e.bubble = false;
    }

    fn on_collapse(&mut self, e: &mut MouseEvent, _: f32) {
        if !self.collapsible {
            return;
        }
        self.collapsed = !self.collapsed;

        if self.collapsed == true {
            self.pre_resize = if let Some(ref handle) = self.resize_handle {
                handle.visible()
            } else {
                false
            };

            self.pre_h = self.h();
            self.pre_h_min = self.h_min();

            let component = self.component.borrow();
            // for child in component.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         if let Some(ref title) = self.title {
            //             if widget.id() == title.id() {
            //                 continue;
            //             }
            //         }

            //         if let Some(ref collapse_handle) = self.collapse_handle {
            //             if widget.id() == collapse_handle.id() {
            //                 continue;
            //             }
            //         }

            //         if let Some(ref close_handle) = self.close_handle {
            //             if widget.id() == close_handle.id() {
            //                 continue;
            //             }
            //         }

            //         widget.set_visible_only(false);
            //     }
            // }

            if let Some(ref title) = self.title {
                self.set_h_min(title.h() + 6.0);
                self.set_h(title.h());
            }
        } else {
            let component = self.component.borrow();
            // for child in component.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         if let Some(ref title) = self.title {
            //             if widget.id() == title.id() {
            //                 continue;
            //             }
            //         }

            //         if let Some(ref collapse_handle) = self.collapse_handle {
            //             if widget.id() == collapse_handle.id() {
            //                 continue;
            //             }
            //         }

            //         if let Some(ref close_handle) = self.close_handle {
            //             if widget.id() == close_handle.id() {
            //                 continue;
            //             }
            //         }
            //         widget.set_visible_only(true);
            //     }
            // }

            self.set_h_min(self.pre_h_min);
            self.set_h(self.pre_h);
        }

        self.oncollapse.emit(&self.collapsed);
        e.bubble = false;
    } //on_collapse

    fn on_close(&self, e: &mut MouseEvent, _: f32) {
        self.onclose.emit(&());

        if self.closable {
            self.close();
        }
    }

    pub fn close(&self) {
        self.set_visible(false);
    }

    pub fn open(&self) {
        self.set_visible(true);
    }
}

impl AsRef<RefCell<WidgetComponent>> for WindowElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

impl Element for WindowElement {
    fn destroy(&self) {
        // self.base.destroy();

        self.onclose.clear();
        self.oncollapse.clear();
    }

    fn add(&self, child: &dyn Element) {
        // self.base.add(child);

        //readd so it"s always above
        if let Some(ref resize_handle) = self.resize_handle {
            if self.ready && child.id() != resize_handle.id() {
                self.add(resize_handle);
            }
        }
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        let ex = e.x as f32;
        let ey = e.y as f32;
        let mut state = self.state.borrow_mut();
        if state.resizing {
            let dx = ex - state.resize_x;
            let dy = ey - state.resize_y;

            let ww = self.w() + dx;
            let hh = self.h() + dy;

            let mut resized = false;

            if (ww >= self.w_min() || ww <= self.w_max()) && ex >= self.x() {
                state.resize_x = ex;
                resized = true;
            }

            if (hh >= self.h_min() || hh <= self.h_max()) && ey >= self.y() {
                state.resize_y = ey;
                resized = true;
            }

            if resized {
                self.set_size(ww, hh);
            }
        } else if state.dragging {
            let dx = ex - state.drag_x;
            let dy = ey - state.drag_y;

            state.drag_x = ex;
            state.drag_y = ey;

            self.set_pos(self.x() + dx, self.y() + dy);
        } else {
            //dragging

            // self.base.mousemove(e);
        }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        let ex = e.x as f32;
        let ey = e.y as f32;

        if let Some(ref close_handle) = self.close_handle {
            if close_handle.contains(ex, ey) && self.closable {
                return;
            }
        }

        if let Some(ref collapse_handle) = self.collapse_handle {
            if collapse_handle.contains(ex, ey) && self.collapsible {
                return;
            }
        }

        let in_title = match self.title {
            Some(ref title) => title.contains(ex, ey),
            None => false,
        };

        if !in_title {
            // self.base.mousedown(e);
        }

        if self.focusable {
            let component = self.component.borrow();
            if let Some(canvas) = component.canvas.as_ref() {
                canvas.bring_to_front(self);
            }
        }

        if self.moveable && in_title {
            let mut state = self.state.borrow_mut();
            state.dragging = true;
            state.drag_x = ex;
            state.drag_y = ey;
            self.capture();
        }
    }

    fn mouseup(&self, e: &mut MouseEvent) {
        // self.base.mouseup(e);

        let mut state = self.state.borrow_mut();
        if state.dragging {
            state.dragging = false;
            self.uncapture();
        }
    }

    // _dx: f32 =0.0, _dy: f32 =0.0, _dw: f32 =0.0, _dh: f32 =0.0
    fn bounds_changed(&self, dx: f32, dy: f32, dw: f32, dh: f32) {
        // self.base.bounds_changed(_dx, _dy, _dw, _dh);

        if let Some(ref close_handle) = self.close_handle {
            close_handle.set_x_local(self.w() - self.title_margin_right - 24.0);
        }

        if let Some(ref collapse_handle) = self.collapse_handle {
            let x = if self.closable {
                self.w() - self.title_margin_right - 48.0
            } else {
                self.w() - self.title_margin_right - 24.0
            };

            collapse_handle.set_x_local(x);
        }

        if let Some(ref title) = self.title {
            title.set_w(self.w() - self.title_margin_right);
        }

        if let Some(ref resize_handle) = self.resize_handle {
            resize_handle.set_pos(self.x() + self.w() - 24.0, self.y() + self.h() - 24.0);
        }
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
                    "Relayout WindowElement {}x{} {}x{}",
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
