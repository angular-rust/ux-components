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
        properties::PanelProperties, ChildBounds, Helper, Id, MouseEvent, ScrollHandleVisibleEvent,
        Signal,
    },
    material::ScrollView,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder},
    services::LayoutSystem,
};

use super::{Element, PanelElement, WidgetComponent};

#[derive(Default, Debug, Clone)]
struct ScrollViewState {
    child_bounds: ChildBounds,
    drag_v: bool,    // = false;
    drag_y: f32,     // = 0.0;
    percent_v: f32,  // = 0.0;
    visible_v: bool, // = false;

    drag_h: bool,    // = false;
    drag_x: f32,     // = 0.0;
    percent_h: f32,  // = 0.0;
    visible_h: bool, // = false;

    ready: bool, // = false;
}

pub struct ScrollViewElement {
    component: Rc<RefCell<WidgetComponent>>,
    state: RefCell<ScrollViewState>,
    pub scrollh: PanelElement,
    pub scrollv: PanelElement,
    pub container: PanelElement,

    pub onchange: Signal<()>,
    pub onhandlevis: Signal<ScrollHandleVisibleEvent>,

    pub units_to_scroll_h: f32, // = 16;
    pub units_to_scroll_v: f32, // = 16;

    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ScrollViewElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ScrollElement").finish()
    }
}

//Public API
impl ScrollViewElement {
    pub fn new(widget: &ScrollView) -> Self {
        let id = widget.key.id();
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        let component = WidgetComponent::get(widget.key.id());

        let container = PanelElement::new(&PanelProperties {
            parent: Some(id),
            mouse_input: true,
            // internal_visible: widget.visible,
            x: 0.0,
            y: 0.0,
            // w: component.w,
            // h: component.h,
            ..Default::default()
        });

        let scrollv = PanelElement::new(&PanelProperties {
            parent: Some(id),
            mouse_input: true,
            // internal_visible: widget.visible,
            // x: component.w - 8.0,
            y: 0.0,
            w: 8.0,
            h: 16.0,
            ..Default::default()
        });

        let scrollh = PanelElement::new(&PanelProperties {
            parent: Some(id),
            mouse_input: true,
            // internal_visible: widget.visible,
            x: 0.0,
            // y: component.h - 8.0,
            w: 16.0,
            h: 8.0,
            ..Default::default()
        });

        // scrollv.onmousedown.listen(scrollvdown);
        // scrollv.onmouseup.listen(scrollvup);
        // scrollv.onmousemove.listen(scrollvmove);

        // scrollh.onmousedown.listen(scrollhdown);
        // scrollh.onmouseup.listen(scrollhup);
        // scrollh.onmousemove.listen(scrollhmove);

        // let renderer = rendering.get::<Scroll>(self);

        let units_to_scroll_h = 16.0; // widget.units_to_scroll_h;
        let units_to_scroll_v = 16.0; // widget.units_to_scroll_v;

        let child_bounds = container.children_bounds();

        let instance = Self {
            component,
            container,
            state: RefCell::new(ScrollViewState {
                child_bounds,
                drag_h: false,
                drag_v: false,
                drag_x: 0.0,
                drag_y: 0.0,
                percent_h: 0.0,
                percent_v: 0.0,
                ready: true,
                visible_h: false,
                visible_v: false,
            }),
            onchange: Signal::new(),
            onhandlevis: Signal::new(),
            renderer: WidgetRenderFactory::global().get::<Self>(),
            scrollh,
            scrollv,
            units_to_scroll_h,
            units_to_scroll_v,
            node,
        };

        // container.set_clip(instance);
        // scrollv.set_clip(instance);
        // scrollh.set_clip(instance);

        instance
    }

    // ?_horizontal:Option<f32>, ?_vertical:Option<f32>
    pub fn set_scroll_percent(&self, horizontal: Option<f32>, vertical: Option<f32>) {
        let mut state = self.state.borrow_mut();
        let percent_v = vertical.unwrap_or(state.percent_v);
        let percent_h = horizontal.unwrap_or(state.percent_h);

        state.percent_v = percent_v.clamp(0.0, 1.0);
        state.percent_h = percent_h.clamp(0.0, 1.0);

        self.update_scroll();
    }

    pub fn update_container(&self) {
        let mut state = self.state.borrow_mut();
        if !state.ready {
            return;
        }

        state.child_bounds = self.container.children_bounds();

        self.container.set_w(state.child_bounds.w);
        self.container.set_h(state.child_bounds.h);
    }

    //Internal

    //vertical

    fn scrollvdown(&self, e: &MouseEvent, _: f32) {
        let mut state = self.state.borrow_mut();
        if !state.visible_v {
            return;
        }

        state.drag_v = true;
        state.drag_y = e.y as f32 - self.scrollv.y();
        self.scrollv.capture();
    }

    fn scrollvup(&self, e: &MouseEvent, _: f32) {
        self.state.borrow_mut().drag_v = false;
        self.scrollv.uncapture();
    }

    fn scrollvmove(&self, e: &MouseEvent, _: f32) {
        let mut state = self.state.borrow_mut();
        if state.drag_v && state.visible_v {
            let _dest =
                (e.y as f32 - state.drag_y).clamp(self.y(), self.bottom() - self.scrollv.h());
            state.percent_v = (_dest - self.y()) / (self.h() - self.scrollv.h());
            self.update_scroll();
        } //drag_v
    }

    //horizontal

    fn scrollhdown(&self, e: &MouseEvent, _: f32) {
        let mut state = self.state.borrow_mut();
        if !state.visible_h {
            return;
        }

        state.drag_h = true;
        state.drag_x = e.x as f32 - self.scrollh.x();
        self.scrollh.capture();
    }

    fn scrollhup(&self, e: &MouseEvent, _: f32) {
        self.state.borrow_mut().drag_h = false;
        self.scrollh.uncapture();
    }

    fn scrollhmove(&self, e: &MouseEvent, _: f32) {
        let mut state = self.state.borrow_mut();
        if state.drag_h && state.visible_h {
            let _dest =
                (e.x as f32 - state.drag_x).clamp(self.x(), self.right() - self.scrollh.w());
            state.percent_h = (_dest - self.x()) / (self.w() - self.scrollh.w());
            self.update_scroll();
        } //drag_h
    }

    //Refresh all relevant values

    fn update_scroll(&self) {
        let mut state = self.state.borrow_mut();
        if !state.ready {
            return;
        }

        let mut dy = self.h() - self.container.h();
        let mut dx = self.w() - self.container.w();

        state.visible_h = dx < 0.0;
        state.visible_v = dy < 0.0;

        if dx >= 0.0 {
            dx = 0.0;
        }

        if dy >= 0.0 {
            dy = 0.0;
        }

        self.container.set_x(self.x() + (dx * state.percent_h));
        self.container.set_y(self.y() + (dy * state.percent_v));

        self.scrollh
            .set_x(self.x() + (state.percent_h * (self.w() - self.scrollh.w())));
        self.scrollv
            .set_y(self.y() + (state.percent_v * (self.h() - self.scrollv.h())));

        self.onchange.emit(&());
        self.onhandlevis.emit(&ScrollHandleVisibleEvent {
            horizontal: state.visible_h,
            vertical: state.visible_v,
        });
    }

    #[inline]
    pub fn step_h(&self) -> f32 {
        self.units_to_scroll_h / self.container.w()
    }

    #[inline]
    pub fn step_v(&self) -> f32 {
        self.units_to_scroll_v / self.container.h()
    }

    pub fn refresh_scroll(&self) {
        self.update_container();
        self.update_scroll();
    }
}

impl AsRef<RefCell<WidgetComponent>> for ScrollViewElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

// Overrides from WidgetComponent
impl Element for ScrollViewElement {
    fn destroy(&self) {
        self.state.borrow_mut().ready = false;

        // self.base.destroy();

        self.onchange.clear();
        self.onhandlevis.clear();
    }

    fn add(&self, child: &dyn Element) {
        //if the internal controls add them normally
        if !self.state.borrow().ready {
            // self.base.add(child);
        } else {
            self.container.add(child);

            self.refresh_scroll();

            child.set_clip(Some(self.id()));
            // depth = depth;
        }
    }

    fn remove(&self, child: Id) {
        // self.base.remove(child);

        self.refresh_scroll();
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        let state = self.state.borrow();
        // self.base.mousewheel(e);

        if e.x != 0 && state.visible_h {
            let dir = Helper::sign(e.x as f32) as f32;
            self.set_scroll_percent(
                Some(state.percent_h + (dir * self.step_h())),
                Some(state.percent_v),
            );
        }

        if e.y != 0 && state.visible_v {
            let dir = Helper::sign(e.y as f32) as f32;
            self.set_scroll_percent(
                Some(state.percent_h),
                Some(state.percent_v + (dir * self.step_v())),
            );
        }
    }

    // _dx: f32 =0.0, _dy: f32 =0.0, _dw: f32 =0.0, _dh: f32 =0.0
    fn bounds_changed(&self, dx: f32, dy: f32, dw: f32, dh: f32) {
        // self.base.bounds_changed(_dx, _dy, _dw, _dh);

        self.refresh_scroll();

        // if self.scrollh.is_some() {
        self.scrollh.set_y_local(self.h() - 8.0);
        // }

        // if self.scrollv.is_some() {
        self.scrollv.set_x_local(self.w() - 8.0);
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
