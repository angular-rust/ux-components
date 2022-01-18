#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    default::default,
    fmt, mem,
    rc::Rc,
};

use crossbeam::channel::{
    bounded, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError,
};
use once_cell::sync::OnceCell;

use crate::{
    elements::CanvasElement,
    foundation::{
        ChildBounds, Id, KeyEvent, KeySignal, MouseEvent, MouseSignal, Signal, TextEvent,
        TextSignal, WidgetClipEvent, WidgetProperties,
    },
    prelude::{OnDemand, Singleton},
};

pub trait OnDemandMarker {}

impl<T: Default + OnDemandMarker> OnDemand<T> for Option<T> {
    fn get(&mut self) -> &T {
        match self {
            None => {
                let _ = mem::replace(self, Some(T::default()));
                self.as_ref().unwrap()
            }
            Some(val) => val,
        }
    }
}

pub struct ActorChannel<T> {
    addr: Sender<T>,
    mailbox: Receiver<T>,
}

impl<T> ActorChannel<T> {
    pub fn address(&self) -> Sender<T> {
        self.addr.clone()
    }

    pub fn mailbox(&self) -> Receiver<T> {
        self.mailbox.clone()
    }

    pub fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.addr.send(msg)
    }

    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.addr.try_send(msg)
    }
}

impl<T> Default for ActorChannel<T> {
    fn default() -> Self {
        // TODO: deal with capacity later
        let (addr, mailbox) = bounded::<T>(8);
        Self { addr, mailbox }
    }
}

impl<T> OnDemandMarker for ActorChannel<T> {}

struct WidgetComponentRegistry {
    storages: RefCell<BTreeMap<Id, Rc<RefCell<WidgetComponent>>>>,
}

unsafe impl std::marker::Send for WidgetComponentRegistry {}
unsafe impl std::marker::Sync for WidgetComponentRegistry {}

impl fmt::Debug for WidgetComponentRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WidgetComponentRegistry")
            .field("storages", &self.storages.borrow().len())
            .finish()
    }
}

impl WidgetComponentRegistry {
    // pub fn has<T: 'static>(&self) -> bool {
    //     let type_id = TypeId::of::<T>();
    //     self.storages.borrow().contains_key(&type_id)
    // }

    // pub fn register<T: 'static + fmt::Debug>(&self, render: Box<dyn WidgetRenderer<T>>) {
    //     let type_id = TypeId::of::<T>();

    //     log::debug!("Register Widget Renderer {:?}", render);

    //     self.storages
    //         .borrow_mut()
    //         .insert(type_id, Rc::new(WidgetRenderHolder(render)));
    // }

    // pub fn remove<T: 'static + fmt::Debug>(&self) -> Option<Rc<WidgetRenderHolder<T>>> {
    //     let type_id = TypeId::of::<T>();

    //     match self.storages.borrow_mut().remove(&type_id) {
    //         Some(render) => match render.downcast::<WidgetRenderHolder<T>>() {
    //             Ok(render) => Some(render),
    //             Err(_) => {
    //                 log::error!("Something wrong with render storage");
    //                 None
    //             }
    //         },
    //         None => None,
    //     }
    // }

    // /// Asks the Rendering service for a Renderer instance,
    // /// For a given control struct type and instance.
    // pub fn get<T: 'static + fmt::Debug>(&self) -> Option<Rc<WidgetRenderHolder<T>>> {
    //     let type_id = TypeId::of::<T>();

    //     match self.storages.borrow().get(&type_id) {
    //         Some(item) => match item.clone().downcast::<WidgetRenderHolder<T>>() {
    //             Ok(render) => Some(render),
    //             Err(_) => {
    //                 log::error!("Something wrong with render storage");
    //                 None
    //             }
    //         },
    //         None => {
    //             log::warn!("Not found render for {}", std::any::type_name::<T>());
    //             None
    //         }
    //     }
    // }
}

impl Default for WidgetComponentRegistry {
    fn default() -> Self {
        Self {
            storages: Default::default(),
        }
    }
}

impl Singleton for WidgetComponentRegistry {
    fn global() -> &'static Self {
        static WIDGET_COMPONENT_REGISTRY_INSTANCE: OnceCell<WidgetComponentRegistry> =
            OnceCell::new();
        WIDGET_COMPONENT_REGISTRY_INSTANCE.get_or_init(Self::default)
    }
}

/// An empty control.
/// Base struct for all controls
/// handles propogation of events,
/// mouse handling, layout and so on

#[derive(Default)]
pub struct WidgetComponent {
    /// The id of this control
    pub id: Id,
    /// Generic framework/user specific data to store with the control,
    /// which can be strong typed on the receiving end.
    pub user: Option<HashMap<String, String>>,

    // /// Root canvas that this element belongs to
    pub canvas: Box<Option<CanvasElement>>,
    /// the top most control below the canvas that holds us
    pub closest_to_canvas: Option<Id>, // WidgetComponent

    /// The x position of the control bounds, world coordinate
    pub x: f32,
    /// The y position of the control bounds, world coordinate
    pub y: f32,
    /// The width of the control bounds
    pub w: f32,
    /// The height of the control bounds
    pub h: f32,

    /// The minimum width
    pub w_min: f32, // = 0,
    /// The minimum height
    pub h_min: f32, // = 0,
    /// The maximum width
    pub w_max: f32, // = 0,
    /// The maximum height
    pub h_max: f32, // = 0,

    /// The right edge of the control bounds
    // pub right (get, never) -> f32,
    /// The bottom edge of the control bounds
    // pub bottom (get, never) -> f32,

    /// The x position of the control bounds, relative to its container
    pub x_local: f32,
    /// The y position of the control bounds, relative to its container
    pub y_local: f32,

    /// The control this one is clipped by
    pub clip: Option<Id>, // WidgetComponent
    /// the list of children added to this control
    pub children: Vec<Id>,
    /// the number of controls below and including this one
    // pub nodes (get,never) -> i32,

    /// if the control has focus
    pub isfocused: bool, // = false,
    /// if the control is marked for potential focus
    pub ismarked: bool, // = false,
    /// if the control has modal focus
    pub iscaptured: bool, // = false,
    /// if the control is under the mouse
    pub ishovered: bool, // = false,
    /// if the control accepts mouse events
    pub mouse_input: bool, // = false,
    /// if the control accepts key events
    pub key_input: bool, // = false;
    /// if the control emits a render signal
    pub renderable: bool, // = false;
    /// if the control has been destroyed and is no longer usable
    pub destroyed: bool, // = false;

    /// If the control is visible
    pub visible: bool, // = true;
    /// A getter for the bounds information about the children and their children in this control
    pub children_bounds: ChildBounds,

    /// An event for when the control is created. Used by the rendering service
    /// TODO: its very strange construction, seems it should moved into declerative part
    pub oncreate: Option<ActorChannel<()>>,
    /// An event for when (if) a control is marked as renderable and is rendered.
    pub onrender: Option<ActorChannel<()>>,
    /// An event for when the bounds of the control change.
    pub onbounds: Option<ActorChannel<()>>,
    /// An event for when the control is being destroyed.
    pub ondestroy: Option<ActorChannel<()>>,
    /// An event for when the visibility of the control changes.
    pub onvisible: Option<ActorChannel<bool>>,
    /// An event for when the control moves in depth in the canvas.
    pub ondepth: Option<ActorChannel<f32>>,
    /// An event for when the clipping rectangle changes for the control.
    pub onclip: Option<ActorChannel<WidgetClipEvent>>,
    /// An event for when a child is added to the control.
    pub onchildadd: Option<ActorChannel<Id>>,
    /// An event for when a child is removed from the control.
    pub onchildremove: Option<ActorChannel<Id>>,
    /// An event for when the mouse is clicked on this control (if `mouse_input`).
    pub onmousedown: Option<ActorChannel<MouseEvent>>,
    /// An event for when the mouse is released on this control (if `mouse_input`).
    pub onmouseup: Option<ActorChannel<MouseEvent>>,
    /// An event for when the mouse is moved inside this control (if `mouse_input`).
    pub onmousemove: Option<ActorChannel<MouseEvent>>,
    /// An event for when the mousewheel is moved while the mouse is inside this control (if `mouse_input`).
    pub onmousewheel: Option<ActorChannel<MouseEvent>>,
    /// An event for when the mouse enters the control (if `mouse_input`).
    pub onmouseenter: Option<ActorChannel<MouseEvent>>,
    /// An event for when the mouse leaves the control (if `mouse_input`).
    pub onmouseleave: Option<ActorChannel<MouseEvent>>,
    /// An event for when a key is pressed and the control is focused (if `key_input`).
    pub onkeydown: Option<ActorChannel<KeyEvent>>,
    /// An event for when a key is released and the control is focused (if `key_input`).
    pub onkeyup: Option<ActorChannel<KeyEvent>>,
    /// An event for when a text/typing event happened and the control is focused (if `key_input`).
    pub ontextinput: Option<ActorChannel<TextEvent>>,
    /// An event for when this control gains or loses focus
    pub onfocused: Option<ActorChannel<bool>>,
    /// An event for when this control is marked or unmarked for focus
    pub onmarked: Option<ActorChannel<bool>>,
    /// An event for when this control is made or unmade the modal focus
    pub oncaptured: Option<ActorChannel<bool>>,

    //the parent control, None if no parent
    pub parent: Option<Id>, // WidgetComponent
    //the depth of this control
    pub depth: f32, // = 1.0;
    // /// The rendering service that this instance uses, defaults to the canvas render service
    // pub rendering: Option<Rendering>,
    /// The control specific options
    // options: O,
    pub depth_offset: f32, // = 0;

    //This is set by set_visible, to allow controls to retain their logical
    //visibility state when their parent is trying to
    //change it against what it"s set at
    pub vis_state: bool,        // = true;
    pub update_vis_state: bool, // = true;

    pub updating: bool,       // = false;
    pub ignore_spatial: bool, // = false;
}

impl WidgetComponent {
    /// Create a WidgetComponent with the given options.
    pub fn new<W: WidgetProperties>(widget: &W) -> Self {
        let id = widget.key().id();
        let canvas = match widget.parent() {
            Some(parent) => {
                // options.parent.add(self);
                // parent.canvas
            }
            None => {
                // //parent.is_some()

                // if !Std.is(self, Canvas) && canvas.is_none() {
                //     panic!("WidgetComponent without a canvas {}", _options_);
                // }
            }
        };

        let depth_offset = widget.depth();

        let h = widget.h();
        let h_max = widget.h_max();
        let h_min = widget.h_min();

        let key_input = widget.key_input();
        let mouse_input = widget.mouse_input();

        let parent = widget.parent();
        let renderable = widget.renderable();
        // let rendering = options.rendering(); //.unwrap_or(canvas.rendering);

        // let user = options.user.unwrap_or_default();

        let w_min = widget.w_min();

        let w_max = widget.w_max();
        let x = widget.x();
        let y = widget.y();
        let w = widget.w();

        let x_local = x;
        let y_local = y;

        let instance = Self {
            id,
            canvas: box None,
            children: Vec::new(),
            children_bounds: ChildBounds {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
                x_local: 0.0,
                y_local: 0.0,
            },
            clip: None,
            closest_to_canvas: None,
            depth: 1.0,
            depth_offset,
            destroyed: false,
            h,
            h_max,
            h_min,
            ignore_spatial: false,
            iscaptured: false,
            isfocused: false,
            ishovered: false,
            ismarked: false,
            key_input,
            mouse_input,
            onbounds: None,
            oncaptured: None,
            onchildadd: None,
            onchildremove: None,
            onclip: None,
            oncreate: None,
            ondepth: None,
            ondestroy: None,
            onfocused: None,
            onkeydown: None,
            onkeyup: None,
            onmarked: None,
            onmousedown: None,
            onmouseenter: None,
            onmouseleave: None,
            onmousemove: None,
            onmouseup: None,
            onmousewheel: None,
            onrender: None,
            ontextinput: None,
            onvisible: None,
            // options,
            parent,
            renderable,
            // renderer: None, // may be should use options for that
            // rendering,
            update_vis_state: true,
            updating: false,
            user: None, // may be use some id related things
            vis_state: true,
            visible: true,
            w: 0.0,
            w_max: 0.0,
            w_min: 0.0,
            x: 0.0,
            x_local: 0.0,
            y: 0.0,
            y_local: 0.0,
        };

        // let closest_to_canvas = instance.find_top_parent(None);

        // //canvas must be valid here

        // instance.renderable = if options.renderable() {
        //     true
        // } else {
        //     // match canvas {
        //     //     Some(ref cancas) => {
        //     //         // canvas.renderable // TODO: DV
        //     //         true
        //     //     },
        //     //     None => false,
        //     // },
        //     false
        // };

        // if options.visible() {
        //     instance.set_visible(true);
        // } else {
        //     if options.internal_visible() {
        //         instance.set_visible_only(options.internal_visible())
        //     } else {
        //         if let Some(parent) = instance.parent() {
        //             instance.set_visible_only(parent.visible())
        //         }
        //     }
        // };

        // TODO: DV

        instance
    }

    pub fn get(id: Id) -> Rc<RefCell<Self>> {
        let mut registry = WidgetComponentRegistry::global().storages.borrow_mut();
        match registry.get(&id) {
            Some(component) => component.clone(),
            None => {
                let component = Rc::new(RefCell::new(WidgetComponent { id, ..default() }));
                registry.insert(id, component.clone());
                component
            }
        }
    }
}

impl PartialEq for WidgetComponent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> PartialEq<WidgetComponent> for &'a WidgetComponent {
    fn eq(&self, other: &WidgetComponent) -> bool {
        *self == other
    }
}
