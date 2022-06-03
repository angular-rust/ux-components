//! Application runner.
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{marker::PhantomData, sync::Arc};
use structopt::StructOpt;

use crate::prelude::SetterMut;

use crate::widgets::StatefulWidget;
use crate::{
    platform::core::{Instance, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages},
    widgets::{BuildContext, Widget, WidgetBuilder, State},
    winit::{
        dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Position, Size},
        error::{ExternalError, NotSupportedError},
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::{CursorIcon, Fullscreen, Icon, UserAttentionType, Window, WindowBuilder},
    },
};

use super::{
    asyncwinit::{EventAsync as Event, EventLoopAsync},
    error,
    resource::{Store, StoreKey, StoreOpt},
    time::{DurationSpec, Monotonic},
    ApplicationSettings, AsyncActivity, AsyncActivityHost, Context, MaterialApplicationHost,
};

/// Application runner.
///
/// This runner shall be used whenever wanted to debug a application.
#[derive(Debug)]
pub struct MaterialApplication {
    pub window: Arc<Window>,
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Width of the viewport.
    #[structopt(short = "w", long = "width")]
    width: Option<u32>,

    /// Height of the viewport.
    #[structopt(short = "h", long = "height")]
    height: Option<u32>,

    /// Shall the viewport be in fullscreen mode?
    #[structopt(short = "f", long = "fullscreen")]
    fullscreen: Option<bool>,

    /// Set a maximum runtime duration. Whenever the time arrives at this duration limit, it will
    /// wrap around to 0. If unset, the application will run with a forever increasing time.
    ///
    /// The syntax is “MmSs”, where M is optional. M must be a natural specifiying the number of
    /// minutes and S a natural specifying the number of seconds. 30s will then be 30 seconds and 1m42
    /// will be 1 minute and 42 seconds. The number of seconds must not exceed 59.
    #[structopt(short = "z", long = "wrap-at")]
    wrap_at: Option<DurationSpec>,

    /// Start the application at a given time.
    #[structopt(short = "s", long = "start-at", default_value = "0s")]
    start_at: DurationSpec,
}

impl MaterialApplication {
    pub fn run<D>(mut context: D::Context, settings: ApplicationSettings)
    where
        D: StatefulWidget<Out = D> + Context + Default + 'static, 
        // D: SceneActivity<Self> + WidgetBuilder + Context + 'static,
    {
        log::info!("starting async \"{}\"", settings.title);

        // get CLI options
        let opt = Opt::from_args();
        let width = opt.width.unwrap_or(settings.width);
        let height = opt.height.unwrap_or(settings.height);
        let fullscreen = opt.fullscreen.unwrap_or(settings.fullscreen);

        // // build the WindowDim
        // let win_dim = if fullscreen {
        //   if opt.width.is_some() && opt.height.is_some() {
        //     log::info!("window mode: fullscreen restricted ({}×{})", width, height);
        //     WindowDim::FullscreenRestricted(width, height)
        //   } else {
        //     log::info!("window mode: fullscreen");
        //     WindowDim::Fullscreen
        //   }
        // } else {
        //   log::info!("window mode: windowed ({}×{})", width, height);
        //   WindowDim::Windowed(width, height)
        // };

        // let win_opt = WindowOpt::default();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(Size::Logical(LogicalSize::new(64.0, 64.0)))
            .with_inner_size(Size::Physical(PhysicalSize::new(width, height)))
            .with_title(settings.title)
            .build(&event_loop)
            .unwrap();

        // initialize and configure
        let instance = Instance::new();
        let surface = instance.create_surface(&window);
        let adapter = instance.request_adapter();
        let (device, _) = adapter.request_device();
        let desc = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: PresentMode::Fifo,
        };
        let swapchain = device.create_swap_chain(&surface, &desc);

        // create the store
        let store_opt = StoreOpt::default().set_root("assets");
        let mut store: Store<D::Context, StoreKey> = Store::new(store_opt)
            .map_err(|e| error::Error::cannot_create_store(format!("{}", e)))
            .unwrap();

        let window = Arc::new(window);

        // create an instance of our runner to pass to the application
        let mut runner = MaterialApplication {
            window: window.clone(),
        };

        // initialize the application
        let mut app = MaterialApplicationHost::init(&mut runner, &mut store, &mut context)
            .map_err(|e| error::Error::app_initialization_failure(format!("{:?}", e)))
            .unwrap();

        // let state: Box<dyn State<D>> = D::default().create_state();
        
        // set the root to  application
        app.set(
            D::default()
                .create_state()
                .build(Some(BuildContext {
                    painter: app.painter.clone(),
                }))
                .create_element(),
        );

        // loop over time and run the application
        let start_time = Monotonic::now();
        let start_at = opt.start_at;
        let wrap_at = opt.wrap_at;

        log::info!("initialized; running…");

        // render loop
        event_loop.run_async(async move |mut asyncrunner| {
            'events: loop {
                asyncrunner.wait().await;

                let mut recv_events = asyncrunner.recv_events().await;
                while let Some(event) = recv_events.next().await {
                    match event {
                        Event::WindowEvent {
                            ref event,
                            window_id,
                        } if window_id == window.id() => {
                            // route to external handlers
                            app.before_event(&mut runner, &mut context, event).await;

                            match event {
                                WindowEvent::Resized(physical_size) => {
                                    // The size of the window has changed. Contains the client area's new dimensions.
                                    //
                                    // make sure the viewport matches the new window dimensions; note that width and
                                    // height will be significantly larger than specified on retina displays.
                                    app.resized(
                                        &mut runner,
                                        &mut context,
                                        physical_size.width,
                                        physical_size.height,
                                    )
                                    .await;
                                }
                                WindowEvent::Moved(position) => {
                                    // The position of the window has changed. Contains the window's new position.
                                    app.moved(&mut runner, &mut context, position.x, position.y)
                                        .await;
                                }

                                WindowEvent::CloseRequested => {
                                    // The window has been requested to close.
                                    let control_flow = app.close(&mut runner, &mut context).await;

                                    if control_flow == ControlFlow::Exit {
                                        break 'events;
                                    }
                                }

                                WindowEvent::Destroyed => {
                                    // The window has been destroyed.
                                    app.destroyed(&mut runner, &mut context).await;
                                }

                                WindowEvent::DroppedFile(path) => {
                                    // A file has been dropped into the window.
                                    //
                                    // When the user drops multiple files at once, this event will be emitted for each file
                                    // separately.
                                    app.dropped_file(&mut runner, &mut context, path).await;
                                }

                                WindowEvent::HoveredFile(path) => {
                                    // A file is being hovered over the window.
                                    //
                                    // When the user hovers multiple files at once, this event will be emitted for each file
                                    // separately.
                                    app.hovered_file(&mut runner, &mut context, path).await;
                                }
                                WindowEvent::HoveredFileCancelled => {
                                    // A file was hovered, but has exited the window.
                                    //
                                    // There will be a single `HoveredFileCancelled` event triggered even if multiple files were
                                    // hovered.
                                    app.hovered_file_cancelled(&mut runner, &mut context).await;
                                }

                                WindowEvent::ReceivedCharacter(character) => {
                                    // The window received a unicode character.
                                    app.received_character(&mut runner, &mut context, *character)
                                        .await;
                                }
                                WindowEvent::Focused(is_focused) => {
                                    // The window gained or lost focus.
                                    //
                                    // The parameter is true if the window has gained focus, and false if it has lost focus.
                                    app.focused(&mut runner, &mut context, *is_focused).await;
                                }
                                WindowEvent::KeyboardInput {
                                    device_id,
                                    input,
                                    is_synthetic,
                                } => {
                                    // An event from the keyboard has been received.
                                    // If `true`, the event was generated synthetically by winit
                                    // in one of the following circumstances:
                                    //
                                    // * Synthetic key press events are generated for all keys pressed
                                    //   when a window gains focus. Likewise, synthetic key release events
                                    //   are generated for all keys pressed when a window goes out of focus.
                                    //   ***Currently, this is only functional on X11 and Windows***
                                    //
                                    // Otherwise, this value is always `false`.

                                    let control_flow = app
                                        .keyboard_input(
                                            &mut runner,
                                            &mut context,
                                            device_id,
                                            input,
                                            *is_synthetic,
                                        )
                                        .await;

                                    if control_flow == ControlFlow::Exit {
                                        break 'events;
                                    }
                                }
                                WindowEvent::ModifiersChanged(state) => {
                                    // The keyboard modifiers have changed.
                                    //
                                    // Platform-specific behavior:
                                    // - **Web**: This API is currently unimplemented on the web. This isn't by design - it's an
                                    //   issue, and it should get fixed - but it's the current state of the API.
                                    app.modifiers_changed(&mut runner, &mut context, state)
                                        .await;
                                }
                                WindowEvent::CursorMoved {
                                    device_id,
                                    position,
                                    ..
                                } => {
                                    // The cursor has moved on the window.
                                    //
                                    // (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
                                    // limited by the display area and it may have been transformed by the OS to implement effects such as cursor
                                    // acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
                                    app.cursor_moved(
                                        &mut runner,
                                        &mut context,
                                        device_id,
                                        position,
                                    )
                                    .await;
                                }
                                WindowEvent::CursorEntered { device_id } => {
                                    // The cursor has entered the window.
                                    app.cursor_entered(&mut runner, &mut context, device_id)
                                        .await;
                                }
                                WindowEvent::CursorLeft { device_id } => {
                                    // The cursor has left the window.
                                    app.cursor_left(&mut runner, &mut context, device_id).await;
                                }
                                WindowEvent::MouseWheel {
                                    device_id,
                                    delta,
                                    phase,
                                    ..
                                } => {
                                    // A mouse wheel movement or touchpad scroll occurred.
                                    app.mouse_wheel(
                                        &mut runner,
                                        &mut context,
                                        device_id,
                                        delta,
                                        phase,
                                    )
                                    .await;
                                }
                                WindowEvent::MouseInput {
                                    device_id,
                                    state,
                                    button,
                                    ..
                                } => {
                                    // An mouse button press has been received.
                                    app.mouse_input(
                                        &mut runner,
                                        &mut context,
                                        device_id,
                                        state,
                                        button,
                                    )
                                    .await;
                                }
                                WindowEvent::TouchpadPressure {
                                    device_id,
                                    pressure,
                                    stage,
                                } => {
                                    // Touchpad pressure event.
                                    //
                                    // At the moment, only supported on Apple forcetouch-capable macbooks.
                                    // The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
                                    // is being pressed) and stage (integer representing the click level).
                                    app.touchpad_pressure(
                                        &mut runner,
                                        &mut context,
                                        device_id,
                                        *pressure,
                                        *stage,
                                    )
                                    .await;
                                }
                                WindowEvent::AxisMotion {
                                    device_id,
                                    axis,
                                    value,
                                } => {
                                    // Motion on some analog axis. May report data redundant to other, more specific events.
                                    app.axis_motion(
                                        &mut runner,
                                        &mut context,
                                        device_id,
                                        axis,
                                        *value,
                                    )
                                    .await;
                                }
                                WindowEvent::Touch(touch) => {
                                    // Touch event has been received
                                    app.touch(&mut runner, &mut context, touch).await;
                                }
                                WindowEvent::ScaleFactorChanged {
                                    scale_factor,
                                    new_inner_size,
                                } => {
                                    // The window's scale factor has changed.
                                    //
                                    // The following user actions can cause DPI changes:
                                    //
                                    // * Changing the display's resolution.
                                    // * Changing the display's scale factor (e.g. in Control Panel on Windows).
                                    // * Moving the window to a display with a different scale factor.
                                    //
                                    // After this event callback has been processed, the window will be resized to whatever value
                                    // is pointed to by the `new_inner_size` reference. By default, this will contain the size suggested
                                    // by the OS, but it can be changed to any value.
                                    //
                                    // For more information about DPI in general, see the [`dpi`](crate::dpi) module.
                                    app.scale_factor_changed(
                                        &mut runner,
                                        &mut context,
                                        *scale_factor,
                                        new_inner_size,
                                    )
                                    .await;
                                }
                                WindowEvent::ThemeChanged(theme) => {
                                    // The system window theme has changed.
                                    //
                                    // AsyncApplications might wish to react to this to change the theme of the content of the window
                                    // when the system changes the window theme.
                                    //
                                    // At the moment this is only supported on Windows.
                                    app.theme_changed(&mut runner, &mut context, theme).await;
                                }
                            }

                            // route to external handlers
                            app.after_event(&mut runner, &mut context, event).await;
                        }
                        // Event::MainEventsCleared => {
                        //     // We draw only in RedrawRequested to filter only current window

                        //     // Redraw here for active games like a RPG or RTS
                        //     // per-frame time logic
                        //     if app.main_events_cleared(&mut runner, &mut context) {
                        //         // request redraw again
                        //         window.request_redraw();
                        //     }
                        // }
                        // Event::RedrawRequested(window_id) if window_id == window.id() => {}
                        // // Event::DeviceEvent | Event::Resumed | Event::Suspended | Event::UserEvent(E)
                        // // _ => println!("{:?}", event),
                        _ => {}
                    }
                }

                // window.request_redraw();

                let mut redraw_requests = recv_events.redraw_requests().await;
                while let Some(window_id) = redraw_requests.next().await {
                    // println!("redraw {:?}", window_id);
                    // redraw here when something changed

                    // render a frame
                    let time = start_time.elapsed_secs();
                    let time = if let Some(wrap_t) = wrap_at {
                        time.wrap_around(wrap_t.into())
                    } else {
                        time
                    };
                    let time = time.offset(start_at.into());

                    let redraw = app.render(&mut runner, &mut context, time).await;

                    // swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
                    swapchain.present(&surface);

                    if redraw {
                        // request redraw again
                        window.request_redraw();
                    }
                }
            }
        });
    }

    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    ///
    /// See the [`dpi`](crate::winit::dpi) module for more information.
    ///
    /// Note that this value can change depending on user action (for example if the window is
    /// moved to another screen); as such, tracking `WindowEvent::ScaleFactorChanged` events is
    /// the most robust way to track the DPI you need to use to draw.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** This respects Xft.dpi, and can be overridden using the `WINIT_X11_SCALE_FACTOR` environment variable.
    /// - **Android:** Always returns 1.0.
    /// - **iOS:** Can only be called on the main thread. Returns the underlying `UIView`'s
    ///   [`contentScaleFactor`].
    ///
    /// [`contentScaleFactor`]: https://developer.apple.com/documentation/uikit/uiview/1622657-contentscalefactor?language=objc
    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    /// Returns the position of the top-left hand corner of the window's client area relative to the
    /// top-left hand corner of the desktop.
    ///
    /// The same conditions that apply to `outer_position` apply to this method.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the top left coordinates of the
    ///   window's [safe area] in the screen space coordinate system.
    /// - **Web:** Returns the top-left coordinates relative to the viewport. _Note: this returns the
    ///    same value as `outer_position`._
    /// - **Android / Wayland:** Always returns [`NotSupportedError`].
    ///
    /// [safe area]: https://developer.apple.com/documentation/uikit/uiview/2891103-safeareainsets?language=objc
    #[inline]
    pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.window.inner_position()
    }

    /// Returns the position of the top-left hand corner of the window relative to the
    ///  top-left hand corner of the desktop.
    ///
    /// Note that the top-left hand corner of the desktop is not necessarily the same as
    ///  the screen. If the user uses a desktop with multiple monitors, the top-left hand corner
    ///  of the desktop is the top-left hand corner of the monitor at the top-left of the desktop.
    ///
    /// The coordinates can be negative if the top-left hand corner of the window is outside
    ///  of the visible screen region.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the top left coordinates of the
    ///   window in the screen space coordinate system.
    /// - **Web:** Returns the top-left coordinates relative to the viewport.
    /// - **Android / Wayland:** Always returns [`NotSupportedError`].
    #[inline]
    pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.window.outer_position()
    }

    /// Modifies the position of the window.
    ///
    /// See `outer_position` for more information about the coordinates. This automatically un-maximizes the
    /// window if it's maximized.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Sets the top left coordinates of the
    ///   window in the screen space coordinate system.
    /// - **Web:** Sets the top-left coordinates relative to the viewport.
    /// - **Android / Wayland:** Unsupported.
    #[inline]
    pub fn set_outer_position<P: Into<Position>>(&self, position: P) {
        self.window.set_outer_position(position.into())
    }

    /// Returns the physical size of the window's client area.
    ///
    /// The client area is the content of the window, excluding the title bar and borders.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the `PhysicalSize` of the window's
    ///   [safe area] in screen space coordinates.
    /// - **Web:** Returns the size of the canvas element.
    ///
    /// [safe area]: https://developer.apple.com/documentation/uikit/uiview/2891103-safeareainsets?language=objc
    #[inline]
    pub fn inner_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    /// Modifies the inner size of the window.
    ///
    /// See `inner_size` for more information about the values. This automatically un-maximizes the
    /// window if it's maximized.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    /// - **Web:** Sets the size of the canvas element.
    #[inline]
    pub fn set_inner_size<S: Into<Size>>(&self, size: S) {
        self.window.set_inner_size(size.into())
    }

    /// Returns the physical size of the entire window.
    ///
    /// These dimensions include the title bar and borders. If you don't want that (and you usually don't),
    /// use `inner_size` instead.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the `PhysicalSize` of the window in
    ///   screen space coordinates.
    /// - **Web:** Returns the size of the canvas element. _Note: this returns the same value as
    ///   `inner_size`._
    #[inline]
    pub fn outer_size(&self) -> PhysicalSize<u32> {
        self.window.outer_size()
    }

    /// Sets a minimum dimension size for the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn set_min_inner_size<S: Into<Size>>(&self, min_size: Option<S>) {
        self.window.set_min_inner_size(min_size.map(|s| s.into()))
    }

    /// Sets a maximum dimension size for the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn set_max_inner_size<S: Into<Size>>(&self, max_size: Option<S>) {
        self.window.set_max_inner_size(max_size.map(|s| s.into()))
    }

    /// Modifies the title of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    #[inline]
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title)
    }

    /// Modifies the window's visibility.
    ///
    /// If `false`, this will hide the window. If `true`, this will show the window.
    /// ## Platform-specific
    ///
    /// - **Android / Wayland / Web:** Unsupported.
    /// - **iOS:** Can only be called on the main thread.
    #[inline]
    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible)
    }

    /// Sets whether the window is resizable or not.
    ///
    /// Note that making the window unresizable doesn't exempt you from handling `Resized`, as that event can still be
    /// triggered by DPI scaling, entering fullscreen mode, etc.
    ///
    /// ## Platform-specific
    ///
    /// This only has an effect on desktop platforms.
    ///
    /// Due to a bug in XFCE, this has no effect on Xfwm.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable)
    }

    /// Sets the window to minimized or back
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    /// - **Wayland:** Un-minimize is unsupported.
    #[inline]
    pub fn set_minimized(&self, minimized: bool) {
        self.window.set_minimized(minimized);
    }

    /// Sets the window to maximized or back.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn set_maximized(&self, maximized: bool) {
        self.window.set_maximized(maximized)
    }

    /// Gets the window's current maximized state.
    ///
    /// ## Platform-specific
    ///
    /// - **Wayland / X11:** Not implemented.
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    /// Sets the window to fullscreen or back.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** `Fullscreen::Exclusive` provides true exclusive mode with a
    ///   video mode change. *Caveat!* macOS doesn't provide task switching (or
    ///   spaces!) while in exclusive fullscreen mode. This mode should be used
    ///   when a video mode change is desired, but for a better user experience,
    ///   borderless fullscreen might be preferred.
    ///
    ///   `Fullscreen::Borderless` provides a borderless fullscreen window on a
    ///   separate space. This is the idiomatic way for fullscreen games to work
    ///   on macOS. See `WindowExtMacOs::set_simple_fullscreen` if
    ///   separate spaces are not preferred.
    ///
    ///   The dock and the menu bar are always disabled in fullscreen mode.
    /// - **iOS:** Can only be called on the main thread.
    /// - **Wayland:** Does not support exclusive fullscreen mode and will no-op a request.
    /// - **Windows:** Screen saver is disabled in fullscreen mode.
    /// - **Android:** Unsupported.
    #[inline]
    pub fn set_fullscreen(&self, fullscreen: Option<Fullscreen>) {
        self.window.set_fullscreen(fullscreen)
    }

    /// Gets the window's current fullscreen state.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread.
    /// - **Android:** Will always return `None`.
    /// - **Wayland:** Can return `Borderless(None)` when there are no monitors.
    #[inline]
    pub fn fullscreen(&self) -> Option<Fullscreen> {
        self.window.fullscreen()
    }

    /// Turn window decorations on or off.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    ///
    /// [`setPrefersStatusBarHidden`]: https://developer.apple.com/documentation/uikit/uiviewcontroller/1621440-prefersstatusbarhidden?language=objc
    #[inline]
    pub fn set_decorations(&self, decorations: bool) {
        self.window.set_decorations(decorations)
    }

    /// Change whether or not the window will always be on top of other windows.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland:** Unsupported.
    #[inline]
    pub fn set_always_on_top(&self, always_on_top: bool) {
        self.window.set_always_on_top(always_on_top)
    }

    /// Sets the window icon. On Windows and X11, this is typically the small icon in the top-left
    /// corner of the titlebar.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland / macOS:** Unsupported.
    ///
    /// On Windows, this sets `ICON_SMALL`. The base size for a window icon is 16x16, but it's
    /// recommended to account for screen scaling and pick a multiple of that, i.e. 32x32.
    ///
    /// X11 has no universal guidelines for icon sizes, so you're at the whims of the WM. That
    /// said, it's usually in the same ballpark as on Windows.
    #[inline]
    pub fn set_window_icon(&self, window_icon: Option<Icon>) {
        self.window.set_window_icon(window_icon)
    }

    /// Sets location of IME candidate box in client area coordinates relative to the top left.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web:** Unsupported.
    #[inline]
    pub fn set_ime_position<P: Into<Position>>(&self, position: P) {
        self.window.set_ime_position(position.into())
    }

    /// Requests user attention to the window, this has no effect if the application
    /// is already focused. How requesting for user attention manifests is platform dependent,
    /// see `UserAttentionType` for details.
    ///
    /// Providing `None` will unset the request for user attention. Unsetting the request for
    /// user attention might not be done automatically by the WM when the window receives input.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland:** Unsupported.
    /// - **macOS:** `None` has no effect.
    /// - **X11:** Requests for user attention must be manually cleared.
    #[inline]
    pub fn request_user_attention(&self, request_type: Option<UserAttentionType>) {
        self.window.request_user_attention(request_type)
    }

    /// Modifies the cursor icon of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    #[inline]
    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        self.window.set_cursor_icon(cursor);
    }

    /// Changes the position of the cursor in window coordinates.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland:** Always returns an [`ExternalError::NotSupported`].
    #[inline]
    pub fn set_cursor_position<P: Into<Position>>(&self, position: P) -> Result<(), ExternalError> {
        self.window.set_cursor_position(position.into())
    }

    /// Grabs the cursor, preventing it from leaving the window.
    ///
    /// There's no guarantee that the cursor will be hidden. You should
    /// hide it by yourself if you want so.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** This locks the cursor in a fixed location, which looks visually awkward.
    /// - **iOS / Android / Web:** Always returns an [`ExternalError::NotSupported`].
    #[inline]
    pub fn set_cursor_grab(&self, grab: bool) -> Result<(), ExternalError> {
        self.window.set_cursor_grab(grab)
    }

    /// Modifies the cursor's visibility.
    ///
    /// If `false`, this will hide the cursor. If `true`, this will show the cursor.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** The cursor is only hidden within the confines of the window.
    /// - **X11:** The cursor is only hidden within the confines of the window.
    /// - **Wayland:** The cursor is only hidden within the confines of the window.
    /// - **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor is
    ///   outside of the window.
    /// - **iOS / Android:** Unsupported.
    #[inline]
    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible)
    }

    /// Moves the window with the left mouse button until the button is released.
    ///
    /// There's no guarantee that this will work unless the left mouse button was pressed
    /// immediately before this fn is called.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** Un-grabs the cursor.
    /// - **Wayland:** Requires the cursor to be inside the window to be dragged.
    /// - **macOS:** May prevent the button release event to be triggered.
    /// - **iOS / Android / Web:** Always returns an [`ExternalError::NotSupported`].
    #[inline]
    pub fn drag_window(&self) -> Result<(), ExternalError> {
        self.window.drag_window()
    }
}
