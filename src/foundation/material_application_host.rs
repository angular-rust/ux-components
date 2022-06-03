#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    future::{self, Future},
    marker::PhantomData,
    path::PathBuf,
    pin::Pin,
    rc::Rc,
};

use cgmath::Point2;

use crate::prelude::{SetterMut, Singleton};
use crate::{
    engine::d2::Painter,
    foundation::{
        resource::{Store, StoreKey},
        time, AsyncActivity, AsyncActivityHost, Context,
    },
    winit::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{
            AxisId, DeviceId, ElementState, KeyboardInput, ModifiersState, MouseButton,
            MouseScrollDelta, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
        },
        event_loop::ControlFlow,
        window::Theme,
    },
};

use crate::{
    elements::{Element, NoneElement},
    rendering::backend::{gles::*, WidgetRenderFactory},
};

use super::MaterialApplication;

pub struct MaterialApplicationHost<T> {
    phantom: PhantomData<T>,
    root: Box<dyn Element>,
    cursor: Point2<f32>,
    pub painter: Rc<Painter>,
}

impl<T> SetterMut<Box<dyn Element>> for MaterialApplicationHost<T> {
    fn set(&mut self, param: Box<dyn Element>) -> &mut Self {
        // log::info!("Host Set root");
        self.root = param;
        self
    }
}

impl<T> Context for MaterialApplicationHost<T> {
    type Context = T;
}

impl<T> AsyncActivityHost<MaterialApplication> for MaterialApplicationHost<T> {
    fn init(
        runner: &mut MaterialApplication,
        store: &mut Store<Self::Context, StoreKey>,
        context: &mut Self::Context,
    ) -> Result<Self, Self::Error> {
        let painter = Rc::new(Painter::new());

        // Composition root in terms of Dependecy Injection
        // We fill WidgetRenderFactory with default render implementations
        {
            let factory = WidgetRenderFactory::global();
            factory.register(box AppBarRender::new(painter.clone()));
            factory.register(box ButtonRender);
            factory.register(box CanvasRender);
            factory.register(box CheckboxRender);
            factory.register(box DividerRender::new(painter.clone()));
            factory.register(box DropdownRender);
            factory.register(box ImageRender::new(painter.clone()));
            factory.register(box LabelRender);
            factory.register(box ListRender);
            factory.register(box MaterialAppRender::new(painter.clone())); // set the projection matrix and do other necessary things
            factory.register(box MaterialButtonRender::new(painter.clone()));
            factory.register(box NavigationRailDestinationRender::new(painter.clone()));
            factory.register(box NavigationRailRender::new(painter.clone()));
            factory.register(box PanelRender);
            factory.register(box ProgressIndicatorRender);
            factory.register(box ScaffoldRender::new(painter.clone()));
            factory.register(box ScrollableRender);
            factory.register(box SliderRender);
            factory.register(box TextRender::new(painter.clone()));
            factory.register(box TextEditRender);
            factory.register(box VerticalDividerRender::new(painter.clone()));
            factory.register(box WindowRender);
        }

        Ok(Self {
            phantom: PhantomData,
            root: box NoneElement::default(),
            cursor: Point2::new(0.0, 0.0),
            painter,
        })
    }
}

impl<T> AsyncActivity<MaterialApplication> for MaterialApplicationHost<T> {
    type Error = ();

    fn resized(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        width: u32,
        height: u32,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        self.root.set_size(width as f32, height as f32);
        Box::pin(future::ready(()))
    }

    fn render(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        t: self::time::Time,
        // back_buffer: &Backbuffer,
        // builder: Builder,
    ) -> Pin<Box<dyn Future<Output = bool> + Send>> {
        self.root.render();
        Box::pin(future::ready(false))
    }

    fn suspend(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn resume(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn moved(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        x: i32,
        y: i32,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn close(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
    ) -> Pin<Box<dyn Future<Output = ControlFlow> + Send>> {
        Box::pin(future::ready(ControlFlow::Exit))
    }

    fn destroyed(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn dropped_file(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        path: &PathBuf,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn hovered_file(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        path: &PathBuf,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn hovered_file_cancelled(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn received_character(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        character: char,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn focused(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        is_focused: bool,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn keyboard_input(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        input: &KeyboardInput,
        is_synthetic: bool,
    ) -> Pin<Box<dyn Future<Output = ControlFlow> + Send>> {
        match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => Box::pin(future::ready(ControlFlow::Exit)),
            _ => Box::pin(future::ready(ControlFlow::Poll)),
        }
    }

    fn modifiers_changed(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        state: &ModifiersState,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn cursor_moved(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        position: &PhysicalPosition<f64>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        self.cursor.x = position.x as f32;
        self.cursor.y = position.y as f32;

        let mut e = super::MouseEvent {
            bubble: true,
            button: super::MouseButton::None,
            state: super::InteractState::Move,
            x: self.cursor.x as i32,
            y: self.cursor.y as i32,
            timestamp: 0.0,
            xrel: 0,
            yrel: 0,
        };

        self.root.mousemove(&mut e);
        Box::pin(future::ready(()))
    }

    fn cursor_entered(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn cursor_left(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn mouse_wheel(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn mouse_input(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        state: &ElementState,
        button: &MouseButton,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let button = match button {
            MouseButton::Left => super::MouseButton::Left,
            MouseButton::Right => super::MouseButton::Right,
            MouseButton::Middle => super::MouseButton::Middle,
            MouseButton::Other(_) => {
                log::error!("Need complete logic with other buttons");
                super::MouseButton::Extra1
            }
        };

        let position = self.cursor;
        match state {
            ElementState::Pressed => {
                let mut e = super::MouseEvent {
                    bubble: true,
                    button,
                    state: super::InteractState::Down,
                    x: self.cursor.x as i32,
                    y: self.cursor.y as i32,
                    timestamp: 0.0,
                    xrel: 0,
                    yrel: 0,
                };

                self.root.mousedown(&mut e);
            }
            ElementState::Released => {
                let mut e = super::MouseEvent {
                    bubble: true,
                    button,
                    state: super::InteractState::Up,
                    x: self.cursor.x as i32,
                    y: self.cursor.y as i32,
                    timestamp: 0.0,
                    xrel: 0,
                    yrel: 0,
                };

                self.root.mouseup(&mut e);
            }
        }
        Box::pin(future::ready(()))
    }

    fn touchpad_pressure(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        pressure: f32,
        stage: i64,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn axis_motion(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        device_id: &DeviceId,
        axis: &AxisId,
        value: f64,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn touch(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        touch: &Touch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn scale_factor_changed(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        scale_factor: f64,
        new_inner_size: &&mut PhysicalSize<u32>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn theme_changed(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        theme: &Theme,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn main_events_cleared(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        // t: Time,
        // back_buffer: &Backbuffer,
        // builder: Builder,
    ) -> Pin<Box<dyn Future<Output = bool> + Send>> {
        Box::pin(future::ready(false))
    }

    fn before_event(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        event: &WindowEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }

    fn after_event(
        &mut self,
        runner: &mut MaterialApplication,
        context: &mut Self::Context,
        event: &WindowEvent,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(future::ready(()))
    }
}
