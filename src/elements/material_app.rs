use cgmath::Point2;
use std::{
    cell::RefCell,
    default::default,
    fmt::{Debug, Formatter, Result},
    rc::Rc,
};
use stretch::{geometry, node::Node, number, style};

use crate::prelude::{OnDemand, Singleton};

use crate::{
    foundation::{Id, KeyEvent, MouseEvent, ScaleChangeEvent, Signal, TextEvent},
    material::MaterialApp,
    rendering::backend::{WidgetRenderFactory, WidgetRenderHolder, WidgetRenderer},
    services::LayoutSystem,
};

use super::{Element, WidgetComponent};

#[derive(Default, Debug, Clone, Copy)]
struct MaterialAppState {
    depth_idx: f32, // = 0.0;

    mouse_down: bool, // = false;
}

/// A canvas is a root object::
/// It requires a rendering instance, and handles all incoming events,
/// propagating them to the children.
/// Additional Signals: none

pub struct MaterialAppElement {
    pub component: Rc<RefCell<WidgetComponent>>, // TODO: should deal with other

    pub home: Box<dyn Element>,

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

    state: RefCell<MaterialAppState>,
    // The concrete renderer for this control instance
    pub renderer: Option<Rc<WidgetRenderHolder<Self>>>,
    // The node in layout system
    pub node: Node,
}

impl Debug for MaterialAppElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("MaterialAppElement").finish()
    }
}

impl MaterialAppElement {
    pub fn new(widget: &MaterialApp) -> Self {
        let node = LayoutSystem::new_node(
            style::Style {
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

        // component.onrender.listen(box |_| {
        //     log::info!("Got signal to render MaterialApp");
        // });

        // canvas = self;
        // let scale = widget.scale;

        let home = widget.home.create_element();

        // If home is not NullElement so override
        // the home layout and set it as child
        if let Some(child) = home.node() {
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
        }

        Self {
            component,
            home,
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
            renderer: WidgetRenderFactory::global().get::<Self>(),
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

impl AsRef<RefCell<WidgetComponent>> for MaterialAppElement {
    fn as_ref(&self) -> &RefCell<WidgetComponent> {
        self.component.as_ref()
    }
}

// Overrides from WidgetComponent
impl Element for MaterialAppElement {
    fn mouseup(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        // self.state.borrow_mut().mouse_down = false;
        // let component = self.component.borrow();
        // component.onmouseup.emit(e);

        // coz home is fit whole MaterialApp,
        // so we do not check that MouseEvent inside home bounds
        self.home.mouseup(e);
    }

    fn mousedown(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        // self.state.borrow_mut().mouse_down = true;
        // let component = self.component.borrow();
        // component.onmousedown.emit(e);

        // coz home is fit whole MaterialApp,
        // so we do not check that MouseEvent inside home bounds
        self.home.mousedown(e);
    }

    fn mousemove(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        // let inside = Helper::in_rect(
        //     e.x as f32,
        //     e.y as f32,
        //     self.x(),
        //     self.y(),
        //     self.w(),
        //     self.h(),
        // );

        // let component = self.component.borrow();
        // //inside but leaving
        // if component.ishovered && !inside {
        //     self.mouseleave(e);
        //     if self.state.borrow().mouse_down {
        //         // TODO: config + move to mouseleave
        //         self.mouseup(e);
        //     }
        // } else if !component.ishovered && inside {
        //     self.mouseenter(e);
        // }

        // if component.onmousemove.is_some() {
        //     component.onmousemove.get().try_send(e.clone());
        // }

        // coz home is fit whole MaterialApp,
        // so we do not check that MouseEvent inside home bounds
        self.home.mousemove(e);
    }

    fn mousewheel(&self, e: &mut MouseEvent) {
        // log::info!("{:?}", e);
        // let component = self.component.borrow();

        // if component.onmousewheel.is_some() {
        //     component.onmousewheel.get().try_send(e.clone());
        // }

        // coz home is fit whole MaterialApp,
        // so we do not check that MouseEvent inside home bounds
        self.home.mousemove(e);
    }

    fn keyup(&self, e: &mut KeyEvent) {
        // log::info!("{:?}", e);
        // let component = self.component.borrow();
        //
        // if component.onkeyup.is_some() {
        //     component.onkeyup.get().try_send(e.clone());
        // }

        // coz home is fit whole MaterialApp,
        // so we do not check that home is focused
        self.home.keyup(e);
    }

    fn keydown(&self, e: &mut KeyEvent) {
        // log::info!("{:?}", e);
        // let component = self.component.borrow();
        //
        // if component.onkeydown.is_some() {
        //     component.onkeydown.get().try_send(e.clone());
        // }

        // coz home is fit whole MaterialApp,
        // so we do not check that home is focused
        self.home.keydown(e);
    }

    fn textinput(&self, e: &mut TextEvent) {
        // log::info!("{:?}", e);
        // let component = self.component.borrow();
        //
        // if component.ontextinput.is_some() {
        //     component.ontextinput.get().try_send(e.clone());
        // }

        // coz home is fit whole MaterialApp,
        // so we do not check that home is focused
        self.home.textinput(e);
    }

    fn set_size(&self, w: f32, h: f32) {
        log::info!("Set Size MaterialAppElement {}x{}", w, h);

        // let (dw, dh) = {
        //     let mut comp = self.as_ref().borrow_mut();

        //     assert!(
        //         !comp.destroyed,
        //         "Widget was already destroyed but is being interacted with"
        //     );

        //     comp.updating = true;

        //     // we calculate here delta from prev size to emit this value
        //     // and relayout childs
        //     let dw = w - comp.w;
        //     let dh = h - comp.h;

        //     comp.w = w;
        //     comp.h = h;

        //     comp.updating = false;

        //     (dw, dh)
        // };

        // self.bounds_changed(0.0, 0.0, dw, dh);

        // just compute layout, update size and relayout whole widget tree
        {
            let mut comp = self.as_ref().borrow_mut();
            comp.w = w;
            comp.h = h;
        }

        let _ = LayoutSystem::compute_layout(
            self.node,
            geometry::Size {
                width: number::Number::Defined(w),
                height: number::Number::Defined(h),
            },
        );

        // println!("{:#?}", LayoutSystem::layout(self.node).unwrap());

        self.relayout(Point2 { x: 0.0, y: 0.0 });
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
            self.home.relayout(Point2 {
                x: comp.x,
                y: comp.y,
            });
        }
    }

    fn update(&self, dt: f32) {
        log::info!("Update MaterialAppElement");
        let component = self.component.borrow();

        // for control in component.children.iter() {
        //     if let Some(widget) = control.widget() {
        //         widget.update(dt);
        //     }
        // }
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

        self.home.render();

        {
            let comp = self.component.borrow();
            // for child in comp.children.iter() {
            //     if let Some(widget) = child.widget() {
            //         widget.render();
            //     }
            // }
        }

        if let Some(ref render) = self.renderer {
            render.post_render(self);
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
