#![allow(unused_variables)]
#![allow(dead_code)]

use crate::prelude::*;
use crate::{
    Actor, ActorBox, AllocationFlags, AnimationMode, HandlerId, Stage, Timeline, Toolbar,
    WindowRotation,
};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct WindowProps {
    // pub native_window: NativeWindow,
    pub has_toolbar: bool,
    pub small_screen: bool,
    pub fullscreen: bool,
    pub rotate_size: bool,
    pub icon_name: Option<String>,
    pub rotation: WindowRotation,
    pub start_angle: f32,
    pub end_angle: f32,
    pub angle: f32,
    pub icon_texture: Option<dx::Handle>,
    pub toolbar: Option<Toolbar>,
    pub resize_grip: Option<Actor>,
    pub debug_actor: Option<Actor>,
    pub rotation_timeline: Option<Timeline>,
}

#[derive(Debug)]
pub struct Window {
    props: RefCell<WindowProps>,
    pub stage: Stage, // previous called stage
}

impl Window {
    pub fn new() -> Window {
        Default::default()
    }

    /// new_with_clutter_stage:
    /// @stage: A #Stage
    ///
    /// Creates a new #Stage, using @stage as the backing #Stage. This
    /// function is meant for use primarily for embedding a #Stage into
    /// a foreign stage when using a toolkit integration library.
    ///
    /// Returns: A #Stage
    ///
    pub fn with_stage(stage: &Stage) -> Window {
        //    unsafe { TODO: call ffi:window_new_with_clutter_stage() }
        unimplemented!()
    }

    /// get_for_stage:
    /// @stage: A #Stage
    ///
    /// Gets the #Stage parent of the #Stage, if it exists.
    ///
    /// Returns: (transfer none): A #Stage, or %None
    ///
    pub fn get_for_stage(stage: &Stage) -> Option<Window> {
        //    unsafe { TODO: call ffi:window_get_for_stage() }
        unimplemented!()
    }

    pub fn test_check(&self) -> String {
        "HERE".into()
    }

    fn reallocate(&self) {
        // let allocation_box = self.stage.get_allocation_box();
        // self.allocation_changed_cb(
        //     &self.stage,
        //     &allocation_box,
        //     AllocationFlags::ALLOCATION_NONE,
        // );
        unimplemented!()
    }

    fn allocation_changed_cb(
        &self,
        actor: &Stage,
        allocation_box: &ActorBox,
        flags: AllocationFlags,
    ) {
        // let padding: Padding;

        // MxWindowPrivate *priv;
        let toolbar_height = 0.0;

        // Note, ideally this would happen just before allocate, but there's
        // no signal we can connect to for that without overriding an actor.
        //
        // Instead, we do this each time the allocation is changed on the stage
        // or the toolbar. This shouldn't be a frequent occurence, but
        // unfortunately can happen multiple times before the actual relayout
        // happens.

        // priv = self.priv;

        // let from_toolbar = actor == props.toolbar;
        // actor = props.stage;

        // let (width, height) = self.inner.get_size();
        // let (stage_width, stage_height) = actor.get_size();

        // let x = (stage_width - width) / 2.0;
        // let y = (stage_height - height) / 2.0;

        // UGLY
        // if !props.has_toolbar || props.small_screen || props.fullscreen {
        //     padding.top = padding.right = padding.bottom = padding.left = 0;
        // } else {
        //     padding.top = padding.right = padding.bottom = padding.left = 1;
        // }

        // if props.has_toolbar && props.toolbar {
        //     actor_get_preferred_height(props.toolbar,
        //                         width - padding.left -
        //                         padding.right,
        //                         NULL, &toolbar_height);

        //     if !from_toolbar {
        //         actor_set_position (props.toolbar,
        //                         padding.left + x,
        //                         padding.top + y);
        //         actor_set_pivot_point(props.toolbar,
        //                         (width / 2.0 - padding.left) / actor_get_width (props.toolbar),
        //                         (height / 2.0 - padding.top) / actor_get_height (props.toolbar));
        //         actor_set_rotation_angle(props.toolbar,
        //                         CLUTTER_Z_AXIS,
        //                         props.angle);
        //         g_object_set(G_OBJECT(props.toolbar),
        //                         "natural-width",
        //                         width - padding.left - padding.right,
        //                         NULL);
        //     }
        // } else {
        //     toolbar_height = 0;
        // }

        // if props.child {
        //     g_object_set(G_OBJECT(props.child),
        //                         "natural-width", width - padding.left - padding.right,
        //                         "natural-height", height - toolbar_height -
        //                                     padding.top - padding.bottom,
        //                         "x", padding.left + x,
        //                         "y", toolbar_height + padding.top + y,
        //                         NULL);
        //     clutter_actor_set_rotation_angle(props.child, CLUTTER_Z_AXIS, props.angle);
        //     clutter_actor_set_pivot_point(props.child,
        //                         (width / 2.0 - padding.left) / clutter_actor_get_width (props.child),
        //                         (height / 2.0 - padding.top - toolbar_height) / clutter_actor_get_height (props.child));
        // }

        // if props.resize_grip {
        //     clutter_actor_get_preferred_size(props.resize_grip,
        //                         NULL, NULL,
        //                         &width, &height);
        //     clutter_actor_set_position(props.resize_grip,
        //                         stage_width - width - padding.right,
        //                         stage_height - height - padding.bottom);
        // }
    }
}

impl Default for Window {
    fn default() -> Self {
        let timeline = Timeline::new(400);
        timeline.set_progress_mode(AnimationMode::EaseInOutQuad);

        let inner = Stage::new();
        inner.set_user_resizable(true);

        let props = WindowProps {
            has_toolbar: true,
            small_screen: false,
            fullscreen: false,
            rotate_size: false,
            icon_name: None,
            toolbar: None,
            resize_grip: None,
            debug_actor: None,
            rotation: WindowRotation::Rotation0,
            rotation_timeline: Some(timeline),
            start_angle: 0.0,
            end_angle: 0.0,
            angle: 0.0,
            icon_texture: None,
        };

        let window = Self {
            stage: inner,
            props: RefCell::new(props),
        };

        // focus_manager_get_for_stage ((Stage *)stage.inner)
        // let toolbar = Toolbar::new();
        // stage.set_toolbar(&toolbar);
        // let resize_grip = Icon::new();
        // stylable_set_style_class(MX_STYLABLE (stage.resize_grip), "ResizeGrip");
        // clutter_actor_add_child(stage.inner, stage.resize_grip);

        // if stage.fullscreen || !clutter_stage_get_user_resizable(CLUTTER_STAGE(stage.inner)) || !stage.has_toolbar {
        //   clutter_actor_hide (stage.resize_grip);
        // }

        // #ifdef HAVE_X11
        //     stage.native_window = _window_x11_new (self);
        // #endif

        // #ifdef HAVE_WAYLAND
        //     stage.native_window = _window_wayland_new (self);
        // #endif

        window
    }
}

impl Object for Window {}
impl Is<Window> for Window {}

impl AsRef<Window> for Window {
    fn as_ref(&self) -> &Window {
        self
    }
}

pub trait WindowExt: 'static {
    /// get_child:
    /// @window: A #Window
    ///
    /// Get the primary child of the window. See set_child().
    ///
    /// Returns: (transfer none): A #Actor, or %None
    ///
    fn get_child(&self) -> Option<Actor>;

    /// get_clutter_stage:
    /// @window: A #Stage
    ///
    /// Gets the #Stage managed by the window.
    ///
    /// Returns: (transfer none): A #Stage
    ///
    fn get_clutter_stage(&self) -> Option<&Stage>;

    /// get_fullscreen:
    /// @window: A #Stage
    ///
    /// Determines if the window has been set to be in fullscreen mode.
    ///
    /// Returns: %true if the window has been set to be in fullscreen mode,
    ///   otherwise %false
    ///
    fn get_fullscreen(&self) -> bool;

    /// get_has_toolbar:
    /// @window: A #Stage
    ///
    /// Determines whether the window has a toolbar or not.
    /// See set_has_toolbar().
    ///
    /// Returns: %true if the window has a toolbar, otherwise %false
    ///
    fn get_has_toolbar(&self) -> bool;

    /// get_icon_name:
    /// @window: A #Stage
    ///
    /// Gets the currently set window icon name. This will be %None if there is none
    /// set, or the icon was set with set_icon_from_cogl_texture().
    ///
    /// Returns: The window icon name, or %None
    ///
    fn get_icon_name(&self) -> Option<String>;

    fn get_resisable(&self) -> bool;

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn get_small_screen(&self) -> bool;

    /// get_title:
    /// @window: A #Stage
    ///
    /// Retrieves the title used for the window.
    ///
    /// Returns: The title used for the window
    ///
    fn get_title(&self) -> Option<String>;

    /// get_toolbar:
    /// @window: A #Stage
    ///
    /// Retrieves the toolbar associated with the window.
    ///
    /// Returns: (transfer none): A #Toolbar
    ///
    fn get_toolbar(&self) -> Option<Toolbar>;

    /// get_window_position:
    /// @window: an #Stage
    /// @x: (out) (allow-none): A pointer for the x-coordinate
    /// @y: (out) (allow-none): A pointer for the y-coordinate
    ///
    /// Retrieves the absolute position of the window on the screen.
    ///
    fn get_window_position(&self) -> (i32, i32);

    /// get_window_rotation:
    /// @window: A #Stage
    ///
    /// Retrieve the rotation of the window.
    ///
    /// Returns: An #StageRotation
    ///
    fn get_window_rotation(&self) -> WindowRotation;

    /// get_window_size:
    /// @window: A #Stage
    /// @width: (out) (allow-none): A #gint pointer for the window's width
    /// @height: (out) (allow-none): A #gint pointer for the window's height
    ///
    /// Retrieves the size of the display area of the window, taking into
    /// account any window border. This includes the area occupied by the
    /// window's toolbar, if it's enabled.
    ///
    fn get_window_size(&self) -> (i32, i32);

    /// hide:
    /// @window: A #Stage
    ///
    /// Hide the window
    ///
    fn hide(&self) -> &Self;

    /// present:
    /// @window: A #Stage
    ///
    /// Present the window. The actual behaviour is specific to the window system.
    ///
    fn present(&self);

    fn set_background_color(&self, color: Option<Color>);

    /// set_child:
    /// @window: A #Stage
    /// @actor: A #Actor
    ///
    /// Adds @actor to the window and sets it as the primary child. When the
    /// stage managed in the window changes size, the child will be resized
    /// to match it.
    ///
    fn set_child<P: Is<Actor>>(&self, actor: &P);

    /// set_fullscreen:
    /// @window: A #Stage
    /// @fullscreen: %true to request fullscreen mode, %false to disable
    ///
    /// Set the window to be in fullscreen mode or windowed mode.
    ///
    /// <note><para>
    /// Setting fullscreen mode doesn't necessarily mean the window is actually
    /// fullscreen. Setting this property is only a request to the underlying
    /// window system.
    /// </para></note>
    ///
    fn set_fullscreen(&self, fullscreen: bool) -> &Self;

    /// set_has_toolbar:
    /// @window: A #Stage
    /// @toolbar: %true if the toolbar should be displayed
    ///
    /// Sets whether the window has a toolbar or not. If the window has a toolbar,
    /// client-side window decorations will be enabled.
    ///
    fn set_has_toolbar(&self, toolbar: bool);

    fn set_icon_from_cogl_texture(&self, texture: dx::Handle);

    /// set_icon_name:
    /// @window: A #Stage
    /// @icon_name: (allow-none): An icon name, or %None
    ///
    /// Set an icon-name to use for the window icon. The icon will be looked up
    /// from the default theme.
    ///
    fn set_icon_name(&self, icon_name: Option<String>);

    fn set_resizable(&self, resizable: bool) -> &Self;

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn set_small_screen(&self, small_screen: bool);

    /// set_title:
    /// @window: A #Stage
    /// @title: A string to use for the window title name
    ///
    /// Sets the title used for the window, the results of which are
    /// window-system specific.
    ///
    fn set_title(&self, title: &str) -> &Self;

    /// set_toolbar:
    /// @window: (allow-none): A #Stage
    ///
    /// Sets the toolbar associated with the window.
    ///
    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P);

    /// set_window_position:
    /// @window: A #Stage
    /// @x: An x-coordinate
    /// @y: A y-coordinate
    ///
    /// Sets the absolute position of the window on the screen.
    ///
    fn set_window_position(&self, x: i32, y: i32);

    /// set_window_rotation:
    /// @window: A #Stage
    /// @rotation: The #StageRotation
    ///
    /// Set the rotation of the window.
    ///
    fn set_window_rotation(&self, rotation: WindowRotation);

    /// set_window_size:
    /// @window: A #Stage
    /// @width: A width, in pixels
    /// @height: A height, in pixels
    ///
    /// Sets the size of the window, taking into account any window border. This
    /// corresponds to the window's available area for its child, minus the area
    /// occupied by the window's toolbar, if it's enabled.
    ///
    /// <note><para>
    /// Setting the window size may involve a request to the underlying windowing
    /// system, and may not immediately be reflected.
    /// </para></note>
    ///
    fn set_window_size(&self, width: i32, height: i32) -> &Self;

    /// show:
    /// @window: A #Stage
    ///
    /// Show the window. Should called after size of stage is set,
    /// for proper reallocation is happen.
    ///
    fn show(&self) -> &Self;

    fn get_property_icon_cogl_texture(&self) -> Option<String>;

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>);

    fn get_property_window_rotation_angle(&self) -> f32;

    fn get_property_window_rotation_timeline(&self) -> Option<Timeline>;

    /// The ::activate signal is emitted when the stage receives key focus
    /// from the underlying window system.
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;
}

impl<O: Is<Window>> WindowExt for O {
    /// get_child:
    /// @window: A #Stage
    ///
    /// Get the primary child of the window. See set_child().
    ///
    /// Returns: (transfer none): A #Actor, or %None
    ///
    fn get_child(&self) -> Option<Actor> {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        // props.child.clone()
        unimplemented!()
    }

    /// get_clutter_stage:
    /// @window: A #Stage
    ///
    /// Gets the #Stage managed by the window.
    ///
    /// Returns: (transfer none): A #Stage
    ///
    fn get_clutter_stage(&self) -> Option<&Stage> {
        let stage = self.as_ref();
        Some(&stage.stage)
    }

    /// get_fullscreen:
    /// @window: A #Stage
    ///
    /// Determines if the window has been set to be in fullscreen mode.
    ///
    /// Returns: %true if the window has been set to be in fullscreen mode,
    ///   otherwise %false
    ///
    fn get_fullscreen(&self) -> bool {
        let stage = self.as_ref();
        stage.stage.get_fullscreen()
    }

    /// get_has_toolbar:
    /// @window: A #Stage
    ///
    /// Determines whether the window has a toolbar or not.
    /// See set_has_toolbar().
    ///
    /// Returns: %true if the window has a toolbar, otherwise %false
    ///
    fn get_has_toolbar(&self) -> bool {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        props.has_toolbar
    }

    /// get_icon_name:
    /// @window: A #Stage
    ///
    /// Gets the currently set window icon name. This will be %None if there is none
    /// set, or the icon was set with set_icon_from_cogl_texture().
    ///
    /// Returns: The window icon name, or %None
    ///
    fn get_icon_name(&self) -> Option<String> {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        props.icon_name.clone()
    }

    fn get_resisable(&self) -> bool {
        let stage = self.as_ref();
        stage.stage.get_user_resizable()
    }

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn get_small_screen(&self) -> bool {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        props.small_screen
    }

    /// get_title:
    /// @window: A #Stage
    ///
    /// Retrieves the title used for the window.
    ///
    /// Returns: The title used for the window
    ///
    fn get_title(&self) -> Option<String> {
        let window = self.as_ref();
        window.stage.get_title().map(|title| title.as_str().into())
    }

    /// get_toolbar:
    /// @window: A #Stage
    ///
    /// Retrieves the toolbar associated with the window.
    ///
    /// Returns: (transfer none): A #Toolbar
    ///
    fn get_toolbar(&self) -> Option<Toolbar> {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        // props.toolbar.clone()
        unimplemented!()
    }

    /// get_window_position:
    /// @window: an #Stage
    /// @x: (out) (allow-none): A pointer for the x-coordinate
    /// @y: (out) (allow-none): A pointer for the y-coordinate
    ///
    /// Retrieves the absolute position of the window on the screen.
    ///
    fn get_window_position(&self) -> (i32, i32) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     return stage.native_window.get_position();
        // }

        (0, 0)
    }

    /// get_window_rotation:
    /// @window: A #Stage
    ///
    /// Retrieve the rotation of the window.
    ///
    /// Returns: An #StageRotation
    ///
    fn get_window_rotation(&self) -> WindowRotation {
        let stage = self.as_ref();
        let props = stage.props.borrow();
        props.rotation
    }

    /// get_window_size:
    /// @window: A #Stage
    /// @width: (out) (allow-none): A #gint pointer for the window's width
    /// @height: (out) (allow-none): A #gint pointer for the window's height
    ///
    /// Retrieves the size of the display area of the window, taking into
    /// account any window border. This includes the area occupied by the
    /// window's toolbar, if it's enabled.
    ///
    fn get_window_size(&self) -> (i32, i32) {
        // let stage = self.as_ref();
        // let (width, height) = stage.stage.get_size();
        // (width as i32, height as i32)
        unimplemented!()
    }

    /// hide:
    /// @window: A #Stage
    ///
    /// Hide the window
    ///
    fn hide(&self) -> &Self {
        // let stage = self.as_ref();
        // stage.stage.hide();
        // self
        unimplemented!()
    }

    /// present:
    /// @window: A #Stage
    ///
    /// Present the window. The actual behaviour is specific to the window system.
    ///
    fn present(&self) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     stage.native_window.present();
        // }
    }

    fn set_background_color(&self, value: Option<Color>) {
        // let stage = self.as_ref();
        // let inner = &stage.stage;

        // inner.set_background_color(value);
        unimplemented!()
    }

    /// set_child:
    /// @window: A #Stage
    /// @actor: A #Actor
    ///
    /// Adds @actor to the window and sets it as the primary child. When the
    /// stage managed in the window changes size, the child will be resized
    /// to match it.
    ///
    fn set_child<P: Is<Actor>>(&self, actor: &P) {
        let window = self.as_ref();
        let actor = actor.as_ref();

        // TODO: we should to find other way to control primary child, so atm disable it
        // // if there are other childs we remove them all,
        // // coz by original design we should have only primaty child Actor
        // if window.stage.get_n_children() > 0 {
        //     window.stage.remove_all_children();
        // }

        // window.stage.add_child(actor);

        // // actor.real_queue_relayout();
        // // actor.queue_redraw();

        // window.reallocate();
        // window.stage.queue_relayout();
        // // window.stage.queue_redraw();
        // // window.stage.ensure_redraw();
        // // g_object_notify(G_OBJECT(window), "child");
        unimplemented!()
    }

    /// set_fullscreen:
    /// @window: A #Stage
    /// @fullscreen: %true to request fullscreen mode, %false to disable
    ///
    /// Set the window to be in fullscreen mode or windowed mode.
    ///
    /// <note><para>
    /// Setting fullscreen mode doesn't necessarily mean the window is actually
    /// fullscreen. Setting this property is only a request to the underlying
    /// window system.
    /// </para></note>
    ///
    fn set_fullscreen(&self, fullscreen: bool) -> &Self {
        let stage = self.as_ref();
        stage.stage.set_fullscreen(fullscreen);
        self
    }

    /// set_has_toolbar:
    /// @window: A #Stage
    /// @toolbar: %true if the toolbar should be displayed
    ///
    /// Sets whether the window has a toolbar or not. If the window has a toolbar,
    /// client-side window decorations will be enabled.
    ///
    fn set_has_toolbar(&self, toolbar: bool) {
        let stage = self.as_ref();
        let mut props = stage.props.borrow_mut();

        if props.has_toolbar != toolbar {
            props.has_toolbar = toolbar;

            if !toolbar {
                // clutter_actor_hide(stage.toolbar);
                // clutter_actor_hide(stage.resize_grip);
            } else {
                // clutter_actor_show(stage.toolbar);
                // if clutter_stage_get_user_resizable((ClutterStage *)stage.stage) {
                //     clutter_actor_show(stage.resize_grip);
                // }
            }

            // g_object_notify(G_OBJECT(window), "has-toolbar");
            // stage.reallocate(window);
        }
    }

    fn set_icon_from_cogl_texture(&self, texture: dx::Handle) {
        // unsafe { TODO: call ffi:window_set_icon_from_cogl_texture() }
        unimplemented!()
    }

    /// set_icon_name:
    /// @window: A #Stage
    /// @icon_name: (allow-none): An icon name, or %None
    ///
    /// Set an icon-name to use for the window icon. The icon will be looked up
    /// from the default theme.
    ///
    fn set_icon_name(&self, icon_name: Option<String>) {
        let stage = self.as_ref();
        let mut props = stage.props.borrow_mut();

        // if props.icon_name && icon_name && g_str_equal(stage.icon_name, icon_name)) {
        //     return;
        // }

        if props.icon_name.is_none() && icon_name.is_none() {
            return;
        }

        props.icon_name = icon_name;
        // g_object_notify(G_OBJECT(window), "icon-name");
    }

    fn set_resizable(&self, resizable: bool) -> &Self {
        let stage = self.as_ref();
        stage.stage.set_user_resizable(resizable);
        self
    }

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn set_small_screen(&self, small_screen: bool) {
        let stage = self.as_ref();
        let mut props = stage.props.borrow_mut();

        if props.small_screen != small_screen {
            props.small_screen = small_screen;
            // g_object_notify(G_OBJECT(window), "small-screen");
        }
    }

    /// set_title:
    /// @window: A #Stage
    /// @title: A string to use for the window title name
    ///
    /// Sets the title used for the window, the results of which are
    /// window-system specific.
    ///
    fn set_title(&self, title: &str) -> &Self {
        let stage = self.as_ref();
        stage.stage.set_title(title);
        self
    }

    /// set_toolbar:
    /// @window: (allow-none): A #Stage
    ///
    /// Sets the toolbar associated with the window.
    ///
    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P) {
        let stage = self.as_ref();
        let toolbar = toolbar.as_ref();

        // if stage.toolbar == (ClutterActor *)toolbar {
        //     return;
        // }

        // // Remove old toolbar
        // if stage.toolbar {
        //     g_signal_handlers_disconnect_by_func(stage.toolbar,
        //                                             stage.allocation_changed_cb,
        //                                             window);
        //     g_object_remove_weak_pointer(G_OBJECT(stage.toolbar), (gpointer *)&stage.toolbar);
        //     clutter_actor_remove_child(stage.stage, stage.toolbar);
        // }

        // stage.toolbar = (ClutterActor *)toolbar;

        // // Add new toolbar
        // if (toolbar) {
        //     clutter_actor_add_child(stage.stage, stage.toolbar);
        //     g_object_add_weak_pointer(G_OBJECT (stage.toolbar), (gpointer *)&stage.toolbar);
        //     g_signal_connect(stage.toolbar, "allocation-changed",
        //                         G_CALLBACK(stage.allocation_changed_cb), window);
        // }

        // stage.has_toolbar = stage.toolbar ? true : false;
    }

    /// set_window_position:
    /// @window: A #Stage
    /// @x: An x-coordinate
    /// @y: A y-coordinate
    ///
    /// Sets the absolute position of the window on the screen.
    ///
    fn set_window_position(&self, x: i32, y: i32) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     stage.native_window.set_position(x, y);
        // }
    }

    /// set_window_rotation:
    /// @window: A #Stage
    /// @rotation: The #StageRotation
    ///
    /// Set the rotation of the window.
    ///
    fn set_window_rotation(&self, rotation: WindowRotation) {
        use WindowRotation::*;

        let stage = self.as_ref();
        let mut props = stage.props.borrow_mut();

        if props.rotation == rotation {
            return;
        }

        if ((props.rotation == Rotation0) || (props.rotation == Rotation180))
            && ((rotation == Rotation90) || (rotation == Rotation270))
        {
            props.rotate_size = true;
        } else if ((props.rotation == Rotation90) || (props.rotation == Rotation270))
            && ((rotation == Rotation0) || (rotation == Rotation180))
        {
            props.rotate_size = true;
        }

        props.rotation = rotation;
        props.start_angle = props.angle;

        match rotation {
            Rotation0 => {
                props.end_angle = 0.0;
            }
            Rotation90 => {
                props.end_angle = 90.0;
            }
            Rotation180 => {
                props.end_angle = 180.0;
            }
            Rotation270 => {
                props.end_angle = 270.0;
            }
        }

        if props.end_angle - props.start_angle > 180.0 {
            props.end_angle -= 360.0;
        } else if props.end_angle - props.start_angle < -180.0 {
            props.end_angle += 360.0;
        }

        // let msecs = (guint)((ABS(props.end_angle - props.start_angle) / 90.0) * 400.0);
        // clutter_timeline_rewind(props.rotation_timeline);
        // clutter_timeline_set_duration(props.rotation_timeline, msecs);
        // clutter_timeline_start(props.rotation_timeline);

        // g_object_notify(G_OBJECT(window), "window-rotation");
    }

    /// set_window_size:
    /// @window: A #Stage
    /// @width: A width, in pixels
    /// @height: A height, in pixels
    ///
    /// Sets the size of the window, taking into account any window border. This
    /// corresponds to the window's available area for its child, minus the area
    /// occupied by the window's toolbar, if it's enabled.
    ///
    /// <note><para>
    /// Setting the window size may involve a request to the underlying windowing
    /// system, and may not immediately be reflected.
    /// </para></note>
    ///
    fn set_window_size(&self, width: i32, height: i32) -> &Self {
        // let stage = self.as_ref();
        // stage.stage.set_size(width as f32, height as f32);
        // self
        unimplemented!()
    }

    /// show:
    /// @window: A #Stage
    ///
    /// Show the window. Should called after size of stage is set,
    /// for proper reallocation is happen.
    ///
    fn show(&self) -> &Self {
        // let stage = self.as_ref();
        // stage.stage.show();
        // self
        unimplemented!()
    }

    fn get_property_icon_cogl_texture(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `icon-cogl-texture` getter")
        // }
        unimplemented!()
    }

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         Value::from(icon_cogl_texture).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_angle(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-angle\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-angle` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_timeline(&self) -> Option<Timeline> {
        // unsafe {
        //     let mut value = Value::from_type(<Timeline as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-timeline\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-timeline` getter")
        // }
        unimplemented!()
    }

    // unsafe fn unsafe_cast_ref<T: ObjectType>(&self) -> &T {
    //     debug_assert!(self.is::<T>());
    //     // This cast is safe because all our wrapper types have the
    //     // same representation except for the name and the phantom data
    //     // type. IsA<> is an unsafe trait that must only be implemented
    //     // if this is a valid wrapper type
    //     &*(self as *const Self as *const T)
    // }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        let window = self.as_ref();

        let this = unsafe { &*(window as *const Window as *const Self) };

        window.stage.connect_activate(move |widget| f(this))
    }

    // TODO: &Self
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // let stage = self.as_ref();
        // let this = unsafe { &*(stage as *const Window as *const Self) };

        // stage.stage.connect_destroy(move |_| {
        //     f(this);
        // })
        unimplemented!()
    }

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::child\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_child_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_fullscreen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fullscreen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fullscreen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_has_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::has-toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_has_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_cogl_texture_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-cogl-texture\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_cogl_texture_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_small_screen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::small-screen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_small_screen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::title\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_title_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        // unsafe extern "C" fn notify_window_rotation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_window_rotation_angle_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-angle\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_angle_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        // unsafe extern "C" fn notify_window_rotation_timeline_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-timeline\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_timeline_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window")
    }
}
