use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{node::Node, style};

use crate::prelude::OnDemand;

use crate::{
    foundation::{Helper, Id, KeyEvent, MouseEvent, ScaleChangeEvent, Signal, TextEvent},
    services::LayoutSystem,
    widgets::Container,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
struct ContainerState {
    depth_idx: f32, // = 0.0;

    mouse_down: bool, // = false;
}

/// A Container is a layout-only object and it do not requires a rendering instance.
/// It and handles all incoming events, propagating them to the children.
/// Additional Signals: none

pub struct ContainerElement {
    pub component: Rc<RefCell<WidgetComponent>>, // TODO: should deal with other

    /// The current focused control, None if none
    pub focused: Option<Id>, // WidgetComponent
    /// The current marked control, None if none
    pub marked: Option<Id>, // WidgetComponent
    /// The current modal control, None if none
    pub captured: Option<Id>, // WidgetComponent
    /// Whether or not the current focus needs refreshing.
    pub focus_invalid: bool, // = true;
    /// The canvas scale factor.
    /// Note that this value is a hint for rendering the canvas,
    /// it does not affect any coordinates directly.
    pub scale: f32, // = 1.0;

    /// An event for when the focused state changes
    pub onfocusedchange: Signal<Id>,
    /// An event for when the marked state changes
    pub onmarkedchange: Signal<Id>,
    /// An event for when the captured state changes
    pub oncapturedchange: Signal<Id>,
    /// On scale changed
    pub onscalechange: Signal<ScaleChangeEvent>,

    state: RefCell<ContainerState>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ContainerElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ContainerElement").finish()
    }
}

impl ContainerElement {
    pub fn new(widget: &Container) -> Self {
        let node = LayoutSystem::new_node(style::Style { ..default() }, vec![]).unwrap();

        // assert!(options.rendering.is_some(), "No Rendering given to Canvas, cannot create a canvas without one.");

        let component = WidgetComponent::get(widget.key.id());

        // component.onrender.listen(box |_| {
        //     log::warn!("Got signal to render Container");
        // });

        // canvas = self;
        // let scale = widget.scale;

        // let home_element = widget.home.create_element();
        Self {
            captured: None,
            component,
            state: Default::default(),
            focus_invalid: true,
            focused: None,
            marked: None,
            oncapturedchange: Signal::new(),
            onfocusedchange: Signal::new(),
            onmarkedchange: Signal::new(),
            onscalechange: Signal::new(),
            scale: 1.0,
            node,
        }
    }

    pub fn bring_to_front(&self, control: &dyn Element) {
        //re-add it to the canvas will put it above
        let component = self.component.borrow();
        if let Some(canvas) = component.canvas.as_ref() {
            canvas.add(control);
            self.sync_depth();
        }
    }

    /// Get the top most control under the given point, or None if there is none (or is the canvas itself)
    pub fn topmost_at_point(&self, x: f32, y: f32) -> Option<&Box<dyn Element>> {
        if let Some(control) = self.topmost_child_at_point(x, y) {
            // control is not this canvas
            if control.id() != self.id() {
                return Some(control);
            }
        }

        None
    }

    //Internal

    pub fn apply_depth(&self, control: &dyn Element) {
        let mut state = self.state.borrow_mut();
        control.set_depth(state.depth_idx + control.depth_offset());
        state.depth_idx += 1.0;

        // for child in control.as_ref().borrow().children.iter() {
        //     if let Some(widget) = child.widget() {
        //         self.apply_depth(widget.as_ref());
        //     }
        // }
    }

    pub fn sync_depth(&self) {
        self.state.borrow_mut().depth_idx = 0.0;
        self.apply_depth(self);
    }

    //getters/setters

    /// The canvas scale factor.
    /// Note that this value is a hint for rendering the canvas,
    /// it does not affect any coordinates directly.
    pub fn scale(&self) -> f32 {
        self.scale
    }
    /// The canvas scale factor.
    /// Note that this value is a hint for rendering the canvas,
    /// it does not affect any coordinates directly.
    pub fn set_scale(&mut self, value: f32) {
        let prev = self.scale;
        self.scale = value;

        if value != prev {
            self.refresh_bounds();
        }

        self.onscalechange.emit(&ScaleChangeEvent { value, prev });
    }

    /// The current focused control, None if none
    pub fn focused(&self) -> Option<&Box<dyn Element>> {
        // self.focused
        todo!()
    }

    pub fn set_focused(&mut self, control: Option<&dyn Element>) {
        if let Some(focused) = self.focused {
            // if let Some(widget) = focused.widget() {
            //     widget.set_focused(false);
            // }
        }

        self.focused = control.map(|x| x.id());

        if let Some(widget) = control {
            self.onfocusedchange.emit(&widget.id());
        }

        if let Some(focused) = self.focused {
            // if let Some(widget) = focused.widget() {
            //     widget.set_focused(true);
            // }
        }
    }

    /// The current modal control, None if none
    pub fn captured(&self) -> Option<&Box<dyn Element>> {
        // self.captured
        todo!()
    }

    /// The current modal control, None if none
    pub fn set_captured(&mut self, control: Option<&dyn Element>) {
        if let Some(captured) = self.captured {
            // if let Some(widget) = captured.widget() {
            //     widget.set_captured(false);
            // }
        }

        self.captured = control.map(|x| x.id());

        if let Some(widget) = control {
            self.oncapturedchange.emit(&widget.id());
        }

        if let Some(captured) = self.captured {
            // if let Some(widget) = captured.widget() {
            //     widget.set_captured(true);
            // }
        }
    }

    /// The current marked control, None if none
    pub fn marked(&self) -> Option<&Box<dyn Element>> {
        // self.marked
        todo!()
    }

    /// The current marked control, None if none
    pub fn set_marked(&mut self, control: Option<&dyn Element>) {
        if let Some(marked) = self.marked {
            // if let Some(widget) = marked.widget() {
            //     widget.set_marked(false);
            // }
        }

        self.marked = control.map(|x| x.id());

        if let Some(widget) = control {
            self.onmarkedchange.emit(&widget.id());
        }

        if let Some(marked) = self.marked {
            // if let Some(widget) = marked.widget() {
            //     widget.set_marked(true);
            // }
        }
    }
}

impl AsRef<RefCell<WidgetComponent>> for ContainerElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

// Overrides from WidgetComponent
impl Element for ContainerElement {
    fn mouseup(&self, e: &mut MouseEvent) {
        self.state.borrow_mut().mouse_down = false;
        let mut component = self.component.borrow_mut();
        if component.onmouseup.is_some() {
            let _ = component.onmouseup.get().try_send(e.clone());
        }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        self.state.borrow_mut().mouse_down = true;
        let mut component = self.component.borrow_mut();

        if component.onmousedown.is_some() {
            let _ = component.onmousedown.get().try_send(e.clone());
        }
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        let inside = Helper::in_rect(
            e.x as f32,
            e.y as f32,
            self.x(),
            self.y(),
            self.w(),
            self.h(),
        );

        let mut component = self.component.borrow_mut();
        //inside but leaving
        if component.ishovered && !inside {
            self.mouseleave(e);
            if self.state.borrow().mouse_down {
                // TODO: config + move to mouseleave
                self.mouseup(e);
            }
        } else if !component.ishovered && inside {
            self.mouseenter(e);
        }

        if component.onmousemove.is_some() {
            let _ = component.onmousemove.get().try_send(e.clone());
        }
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        let mut component = self.component.borrow_mut();

        if component.onmousewheel.is_some() {
            let _ = component.onmousewheel.get().try_send(e.clone());
        }
    }

    fn keyup(&self, e: &mut KeyEvent) {
        let mut component = self.component.borrow_mut();

        if component.onkeyup.is_some() {
            let _ = component.onkeyup.get().try_send(e.clone());
        }
    }

    fn keydown(&self, e: &mut KeyEvent) {
        let mut component = self.component.borrow_mut();

        if component.onkeydown.is_some() {
            let _ = component.onkeydown.get().try_send(e.clone());
        }
    }

    fn textinput(&self, e: &mut TextEvent) {
        let mut component = self.component.borrow_mut();

        if component.ontextinput.is_some() {
            let _ = component.ontextinput.get().try_send(e.clone());
        }
    }

    fn update(&self, dt: f32) {
        log::info!("Update ContainerElement");
        // let component = self.component.borrow();

        // for control in component.children.iter() {
        //     if let Some(widget) = control.widget() {
        //         widget.update(dt);
        //     }
        // }
    }

    fn destroy(&self) {
        // self.base.destroy();

        self.onfocusedchange.clear();
        self.onmarkedchange.clear();
        self.oncapturedchange.clear();
        self.onscalechange.clear();
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
