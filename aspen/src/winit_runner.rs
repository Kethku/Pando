use futures::executor::block_on;
use std::{sync::Arc};

use vello::kurbo::{Point, Size, Vec2};

use winit::{
    application::ApplicationHandler,
    event::MouseScrollDelta,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::windows::WindowAttributesExtWindows,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    application::Application, context_stack::Context,
    element::{Element, ElementPointer}, winit_renderer::WinitRenderer
};

struct WinitApplicationHandler<Root: Element> {
    application: Application<Root>,
    renderer: Option<WinitRenderer>,
}

impl<Root: Element> WinitApplicationHandler<Root> {
    fn new<F>(root_constructor: F) -> Self
    where
        F: for<'a> FnOnce(&mut Context<'a>) -> ElementPointer<Root>,
    {
        WinitApplicationHandler {
            application: Application::new(root_constructor),
            renderer: None,
        }
    }

    async fn create_renderer(window: Arc<Window>) -> WinitRenderer {
        WinitRenderer::new(window).await
    }

    fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
        let window = self.renderer.as_ref().unwrap().window.clone();
        if let Some(scene) = self.application.tick(window.clone(), event_loop) {
            self.renderer.as_mut().unwrap().draw(&scene, &window);

            if !window.is_visible().unwrap_or_default() {
                window.set_visible(true);
            }
            // Request a redraw so that we can continue timers if they need to
            window.request_redraw();
        }
    }
}

impl<A: Element> ApplicationHandler for WinitApplicationHandler<A> {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            // Window Specific Events
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.draw_frame(event_loop);
            }
            WindowEvent::Resized(new_size) => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .resize(new_size.width, new_size.height);
                self.application.event_state.window_size =
                    Size::new(new_size.width as f64, new_size.height as f64);
                self.application.force_redraw = true;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }

            // Mouse Specific Events
            WindowEvent::CursorMoved { position, .. } => {
                self.application.event_state.mouse_position = Some(Point::new(position.x, position.y));
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.application.event_state.mouse_position = None;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.application.event_state.mouse_down = state == ElementState::Pressed;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => {
                self.application.event_state.right_mouse_down = state == ElementState::Pressed;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.application.event_state.scroll_delta += Vec2::new(x as f64 * 10., y as f64 * 10.)
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.application.event_state.scroll_delta += Vec2::new(delta.x as f64, delta.y as f64)
                    }
                }

                self.renderer.as_ref().unwrap().window.request_redraw();
            }

            // Keyboard Specific Events
            WindowEvent::ModifiersChanged(modifiers) => {
                self.application.event_state.modifiers = modifiers.into();
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.application.event_state.key_events.push(event.clone());
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_none() {
            let attributes = WindowAttributes::default()
                .with_visible(false)
                .with_resizable(true)
                .with_decorations(false)
                .with_undecorated_shadow(true);
            let window = Arc::new(
                event_loop
                    .create_window(attributes)
                    .expect("Failed to create window"),
            );

            self.application.force_redraw = true;
            self.renderer = Some(block_on(Self::create_renderer(window)));
            self.draw_frame(event_loop);
        } else {
            self.renderer.as_mut().unwrap().resumed();
        }
    }
}

pub fn run<A: Element, F>(app_constructor: F)
where
    F: for<'a> FnOnce(&mut Context<'a>) -> ElementPointer<A>,
{
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut application_handler = WinitApplicationHandler::new(app_constructor);

    event_loop.run_app(&mut application_handler).ok();
}
