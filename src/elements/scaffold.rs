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
    foundation::{Helper, Id, KeyEvent, MouseEvent, ScaleChangeEvent, Signal, TextEvent},
    material::Scaffold,
    prelude::OnDemand,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
struct ScaffoldState {
    depth_idx: f32, // = 0.0;

    mouse_down: bool, // = false;
}

/// A canvas is a root object::
/// It requires a rendering instance, and handles all incoming events,
/// propagating them to the children.
/// Additional Signals: none

pub struct ScaffoldElement {
    pub component: Rc<RefCell<WidgetComponent>>, // TODO: should deal with other

    pub app_bar: Box<dyn Element>,
    pub body: Box<dyn Element>,
    pub floating_action_button: Box<dyn Element>,
    pub drawer: Box<dyn Element>,
    pub end_drawer: Box<dyn Element>,
    pub bottom_navigation_bar: Box<dyn Element>,
    pub bottom_sheet: Box<dyn Element>,

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

    state: RefCell<ScaffoldState>,
    // The concrete renderer for this control instance
    pub render: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for ScaffoldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ScaffoldElement").finish()
    }
}

impl ScaffoldElement {
    pub fn new(widget: &Scaffold) -> Self {
        let node = LayoutSystem::new_node(
            style::Style {
                flex_direction: style::FlexDirection::Column,
                align_items: style::AlignItems::Stretch,
                // align_content: style::AlignContent::FlexStart,
                // justify_content: style::JustifyContent::Center,
                size: geometry::Size {
                    width: style::Dimension::Percent(1.0),
                    height: style::Dimension::Percent(1.0),
                },
                ..default()
            },
            vec![],
        )
        .unwrap();

        // assert!(options.rendering.is_some(), "No Rendering given to Canvas, cannot create a canvas without one.");

        let component = WidgetComponent::get(widget.key.id());

        // canvas = self;
        // let scale = widget.scale;

        let mut childs = Vec::new();

        let app_bar = widget.app_bar.create_element();
        {
            let childs = &mut childs;
            app_bar.node().map(|child| {
                let child_style = LayoutSystem::style(child).unwrap();
                LayoutSystem::set_style(
                    child,
                    style::Style {
                        align_self: style::AlignSelf::FlexStart,
                        // size: geometry::Size {
                        //     width: style::Dimension::Percent(1.0),
                        //     height: style::Dimension::Points(64.0),
                        // },
                        ..child_style
                    },
                )
                .unwrap();
                childs.push(child);
            });
        }

        let body = widget.body.create_element();
        {
            let childs = &mut childs;
            body.node().map(|child| {
                let child_style = LayoutSystem::style(child).unwrap();
                LayoutSystem::set_style(
                    child,
                    style::Style {
                        flex_grow: 1.0,
                        size: geometry::Size {
                            width: style::Dimension::Percent(1.0),
                            height: style::Dimension::Points(64.0),
                        },
                        ..child_style
                    },
                )
                .unwrap();
                childs.push(child);
            });
        }

        // Not in layout tree
        let floating_action_button = widget.floating_action_button.create_element();
        // Not in layout tree. drawer from left side
        let drawer = widget.drawer.create_element();
        // Not in layout tree. drawer from right side
        let end_drawer = widget.end_drawer.create_element();

        let bottom_navigation_bar = widget.bottom_navigation_bar.create_element();
        {
            let childs = &mut childs;
            bottom_navigation_bar.node().map(|child| {
                let child_style = LayoutSystem::style(child).unwrap();
                LayoutSystem::set_style(
                    child,
                    style::Style {
                        align_self: style::AlignSelf::FlexEnd,
                        size: geometry::Size {
                            width: style::Dimension::Percent(1.0),
                            height: style::Dimension::Points(64.0),
                        },
                        ..child_style
                    },
                )
                .unwrap();
                childs.push(child);
            });
        }

        // Not in Nodes tree
        let bottom_sheet = widget.bottom_sheet.create_element();

        LayoutSystem::set_children(node, childs).unwrap();

        Self {
            component,

            app_bar,
            body,
            floating_action_button,
            drawer,
            end_drawer,
            bottom_navigation_bar,
            bottom_sheet,

            captured: None,
            state: Default::default(),
            focus_invalid: true,
            focused: None,
            marked: None,
            oncapturedchange: Signal::new(),
            onfocusedchange: Signal::new(),
            onmarkedchange: Signal::new(),
            onscalechange: Signal::new(),
            scale: 1.0,
            render: WidgetRenderFactory::global().get::<Self>(),
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

        return None;
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

impl AsRef<RefCell<WidgetComponent>> for ScaffoldElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

// Overrides from WidgetComponent
impl Element for ScaffoldElement {
    fn mouseup(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        self.state.borrow_mut().mouse_down = false;
        let mut component = self.component.borrow_mut();

        if component.onmouseup.is_some() {
            let _ = component.onmouseup.get().try_send(e.clone());
        }
    }

    fn mousedown(&self, e: &mut MouseEvent) {
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

            if self.app_bar.contains(x, y) {
                // log::info!("Is an AppBar");
                self.app_bar.mousedown(e);
            }

            if self.body.contains(x, y) {
                // log::info!("Is an Body");
                self.body.mousedown(e);
            }

            if self.floating_action_button.contains(x, y) {
                // log::info!("Is an FAB");
                self.floating_action_button.mousedown(e);
            }

            if self.drawer.contains(x, y) {
                // log::info!("Is an Drawer");
                self.drawer.mousedown(e);
            }

            if self.end_drawer.contains(x, y) {
                // log::info!("Is an EndDrawer");
                self.end_drawer.mousedown(e);
            }

            if self.bottom_navigation_bar.contains(x, y) {
                // log::info!("Is an BottomNavigationBar");
                self.bottom_navigation_bar.mousedown(e);
            }

            if self.bottom_sheet.contains(x, y) {
                // log::info!("Is an BottomSheet");
                self.bottom_sheet.mousedown(e);
            }
        }
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        let inside = Helper::in_rect(
            e.x as f32,
            e.y as f32,
            self.x(),
            self.y(),
            self.w(),
            self.h(),
        );

        let ishovered = self.component.borrow().ishovered;

        //inside but leaving
        if ishovered && !inside {
            self.mouseleave(e);
            if self.state.borrow().mouse_down {
                // TODO: config + move to mouseleave
                self.mouseup(e);
            }
        } else if !ishovered && inside {
            self.mouseenter(e);
        }

        let mut component = self.component.borrow_mut();
        if component.onmousemove.is_some() {
            let _ = component.onmousemove.get().try_send(e.clone());
        }
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        log::info!("{:?}", e);
        let mut component = self.component.borrow_mut();

        if component.onmousewheel.is_some() {
            let _ = component.onmousewheel.get().try_send(e.clone());
        }
    }

    fn keyup(&self, e: &mut KeyEvent) {
        log::info!("{:?}", e);
        let mut component = self.component.borrow_mut();

        if component.onkeyup.is_some() {
            let _ = component.onkeyup.get().try_send(e.clone());
        }
    }

    fn keydown(&self, e: &mut KeyEvent) {
        log::info!("{:?}", e);
        let mut component = self.component.borrow_mut();

        if component.onkeydown.is_some() {
            let _ = component.onkeydown.get().try_send(e.clone());
        }
    }

    fn textinput(&self, e: &mut TextEvent) {
        log::info!("{:?}", e);
        let mut component = self.component.borrow_mut();

        if component.ontextinput.is_some() {
            let _ = component.ontextinput.get().try_send(e.clone());
        }
    }

    fn set_size(&self, w: f32, h: f32) {
        // log::info!("Set Size Scaffold {}x{}", w, h);

        let (dw, dh) = {
            let mut comp = self.as_ref().borrow_mut();

            assert!(
                comp.destroyed == false,
                "Widget was already destroyed but is being interacted with"
            );

            comp.updating = true;

            let dw = w - comp.w;
            let dh = h - comp.h;

            comp.w = w;
            comp.h = h;

            comp.updating = false;

            (dw, dh)
        };

        self.bounds_changed(0.0, 0.0, dw, dh);

        // self.app_bar.set_w(w);
        // self.body.set_w(w);
        // self.bottom_navigation_bar.set_w(w);
        // self.bottom_sheet.set_w(w);
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

            self.app_bar.relayout(origin);
            self.body.relayout(origin);
            self.floating_action_button.relayout(origin);
            self.end_drawer.relayout(origin);
            self.bottom_navigation_bar.relayout(origin);
            self.bottom_sheet.relayout(origin);
        }
    }

    fn update(&self, dt: f32) {
        log::info!("Update Scaffolld");
        let component = self.component.borrow();

        // for control in component.children.iter() {
        //     if let Some(widget) = control.widget() {
        //         widget.update(dt);
        //     }
        // }
    }

    fn render(&self) {
        log::info!("Render Scaffold");
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

        let comp = self.as_ref().borrow();
        self.app_bar.render();
        self.body.render();
        self.floating_action_button.render();
        self.drawer.render();
        self.end_drawer.render();
        self.bottom_navigation_bar.render();
        self.bottom_sheet.render();

        {
            let comp = self.component.borrow();
            // for child in comp.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         widget.render();
            //     }
            // }
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
}
