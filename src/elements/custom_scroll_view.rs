use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{geometry, node::Node, style};

#[allow(unused_imports)]
use crate::{prelude::OnDemand, rendering::MainAxisSize};

use crate::{
    foundation::{Helper, Id, KeyEvent, MouseEvent, ScaleChangeEvent, Signal, TextEvent},
    services::LayoutSystem,
    widgets::CustomScrollView,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
struct CustomScrollViewState {
    depth_idx: f32, // = 0.0;

    mouse_down: bool, // = false;
}

/// A CustomScrollView is a layout-only object and it do not requires a rendering instance.
/// It and handles all incoming events, propagating them to the children.
/// Additional Signals: none

pub struct CustomScrollViewElement {
    pub component: Rc<RefCell<WidgetComponent>>, // TODO: should deal with other

    pub children: Vec<Box<dyn Element>>,

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

    state: RefCell<CustomScrollViewState>,

    // The node in layout system
    pub node: Node,
}

impl Debug for CustomScrollViewElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("CustomScrollViewElement").finish()
    }
}

impl CustomScrollViewElement {
    pub fn new(widget: &CustomScrollView) -> Self {
        // assert!(options.rendering.is_some(), "No Rendering given to Canvas, cannot create a canvas without one.");

        let component = WidgetComponent::get(widget.key.id());

        // component.onrender.listen(box |_| {
        //     log::warn!("Got signal to render CustomScrollView");
        // });

        // canvas = self;
        // let scale = widget.scale;

        let mut children = Vec::new();

        let mut child_nodes = Vec::new();
        let mut children_height = 0.0;
        let mut children_height_correct = true;

        for child in widget.slivers.iter() {
            let element = child.create_element();
            let child_nodes = &mut child_nodes;
            if let Some(child) = element.node() {
                // let child_style = LayoutSystem::style(child).unwrap();
                // LayoutSystem::set_style(
                //     child,
                //     style::Style {
                //         align_self: style::AlignSelf::Center,
                //         ..child_style
                //     },
                // )
                // .unwrap();

                // calculate row height
                let child_style = LayoutSystem::style(child).unwrap();
                if let style::Dimension::Points(height) = child_style.size.height {
                    children_height += height;
                } else {
                    #[allow(unused_assignments)]
                    {
                        children_height_correct = false;
                    }
                }

                child_nodes.push(child);
            }

            children.push(element);
        }

        // let height = match widget.main_axis_size {
        //     MainAxisSize::Min => {
        //         // should use calculated sum of childs height
        //         if children_height_correct {
        //             style::Dimension::Points(children_height)
        //         } else {
        //             style::Dimension::Percent(1.0)
        //         }
        //     },
        //     MainAxisSize::Max => style::Dimension::Percent(1.0)
        // };

        let height = style::Dimension::Points(children_height);

        let node = LayoutSystem::new_node(
            style::Style {
                flex_direction: style::FlexDirection::Column,
                align_items: style::AlignItems::Stretch,
                // align_content: style::AlignContent::FlexStart,
                // justify_content: style::JustifyContent::Center,
                size: geometry::Size {
                    width: style::Dimension::Percent(1.0),
                    height,
                },
                ..default()
            },
            vec![],
        )
        .unwrap();

        LayoutSystem::set_children(node, child_nodes).unwrap();
        
        Self {
            captured: None,
            children,
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

impl AsRef<RefCell<WidgetComponent>> for CustomScrollViewElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

// Overrides from WidgetComponent
impl Element for CustomScrollViewElement {
    fn mouseup(&self, e: &mut MouseEvent) {
        self.state.borrow_mut().mouse_down = false;
        let mut component = self.component.borrow_mut();
        if component.onmouseup.is_some() {
            let _ = component.onmouseup.get().try_send(e.clone());
        }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        // self.state.borrow_mut().mouse_down = true;
        // let component = self.component.borrow();
        // component.onmousedown.emit(e);

        // log::info!("{:?}", e);
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

            // if self.title.contains(x, y) {
            //     log::info!("Is an Title");
            //     self.title.mousedown(e);
            // }

            // if self.flexible_space.contains(x, y) {
            //     // log::info!("Is an FAB");
            //     self.flexible_space.mousedown(e);
            // }

            for child in self.children.iter() {
                if child.contains(x, y) {
                    child.mousedown(e);
                }
            }
            // if self.drawer.contains(x, y) {
            //     // log::info!("Is an Drawer");
            //     self.drawer.mousedown(e);
            // }
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
        log::info!("Update CustomScrollViewElement");
        let component = self.component.borrow();

        // for control in component.children.iter() {
        //     if let Some(widget) = control.widget() {
        //         widget.update(dt);
        //     }
        // }
    }

    fn render(&self) {
        let comp = self.as_ref().borrow();

        assert!(
            !comp.destroyed,
            "Widget was already destroyed but is being interacted with"
        );

        // if comp.renderable {
        //     comp.onrender.emit(&());
        // }

        // for child in comp.children.iter() {
        //     if let Some(widget) = child.widget() {
        //         widget.render();
        //     }
        // }
        for child in self.children.iter() {
            child.render();
        }
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
            let comp = self.as_ref().borrow();
            let origin = Point2 {
                x: comp.x,
                y: comp.y,
            };

            for child in self.children.iter() {
                child.relayout(origin);
            }
        }
    }
}
