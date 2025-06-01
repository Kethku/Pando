use futures::executor::block_on;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

use vello::{
    kurbo::{Affine, Point, Size, Vec2},
    Scene,
};
use winit::{
    application::ApplicationHandler,
    event::MouseScrollDelta,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::windows::WindowAttributesExtWindows,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    context::{Context, DrawContext, EventState, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    mouse_region::MouseRegionManager,
    shaper::Shaper,
    token::Token,
    winit_renderer::WinitRenderer,
};

struct WinitApplicationHandler<A: Element> {
    mouse_region_manager: RefCell<MouseRegionManager>,
    app: RefCell<ElementPointer<A>>,
    event_state: EventState,
    renderer: Option<WinitRenderer>,
    shaper: RefCell<Shaper>,

    regions: RefCell<HashMap<Token, (Affine, Size)>>,
    token: Token,
    force_redraw: bool,
}

impl<A: Element> WinitApplicationHandler<A> {
    fn new(app: impl Into<ElementPointer<A>>) -> Self {
        WinitApplicationHandler {
            mouse_region_manager: RefCell::new(MouseRegionManager::new()),
            app: RefCell::new(app.into()),
            event_state: EventState::new(),
            renderer: None,
            shaper: RefCell::new(Shaper::new()),

            regions: RefCell::new(HashMap::new()),
            token: Token::new::<Self>(),
            force_redraw: false,
        }
    }

    async fn create_renderer(window: Arc<Window>) -> WinitRenderer {
        WinitRenderer::new(window).await
    }

    fn context<'a>(&'a self, event_loop: &'a ActiveEventLoop) -> Context<'a> {
        Context::new(
            &self.event_state,
            event_loop,
            self.renderer.as_ref().unwrap().window.clone(),
            &self.shaper,
            self.token,
        )
    }

    fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
        let mut mouse_region_manager = self.mouse_region_manager.borrow_mut();
        let mut app = self.app.borrow_mut();
        let mut regions = self.regions.borrow_mut();

        let mut redraw_requested =
            mouse_region_manager.process_regions(&mut regions, &self.context(event_loop));
        {
            let mut update_context = UpdateContext::new(
                self.context(event_loop),
                &mut mouse_region_manager,
                &mut redraw_requested,
            );
            app.update(&mut update_context);
        }

        if redraw_requested || self.force_redraw {
            let mut child_lookup = HashMap::new();
            {
                let mut layout_context =
                    LayoutContext::new(self.context(event_loop), &mut regions, &mut child_lookup);
                let result = app.layout(
                    self.event_state.window_size,
                    self.event_state.window_size,
                    &mut layout_context,
                );
                result.position(Affine::IDENTITY, &mut layout_context);
            }

            mouse_region_manager.clear_regions();
            let mut scene = Scene::new();
            let mut draw_context = DrawContext::new(
                self.context(event_loop),
                &mut mouse_region_manager,
                &child_lookup,
                &regions,
                &mut scene,
            );
            app.draw(&mut draw_context);
            // mouse_region_manager.draw_mouse_regions(&mut scene);

            self.renderer.as_mut().unwrap().draw(&scene);
            self.force_redraw = false;

            let window = &self.renderer.as_ref().unwrap().window;
            if !window.is_visible().unwrap_or_default() {
                window.set_visible(true);
            }
            // Request a redraw so that we can continue timers if they need to
            window.request_redraw();
        }
        self.event_state.next_frame();
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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.event_state.mouse_position = Point::new(position.x, position.y);
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.event_state.mouse_down = state == ElementState::Pressed;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => {
                self.event_state.right_mouse_down = state == ElementState::Pressed;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.event_state.scroll_delta += Vec2::new(x as f64 * 10., y as f64 * 10.)
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.event_state.scroll_delta += Vec2::new(delta.x as f64, delta.y as f64)
                    }
                }

                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.draw_frame(event_loop);
            }
            WindowEvent::Resized(new_size) => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .resize(new_size.width, new_size.height);
                self.event_state.window_size =
                    Size::new(new_size.width as f64, new_size.height as f64);
                self.force_redraw = true;
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

            self.force_redraw = true;
            self.renderer = Some(block_on(Self::create_renderer(window)));
            self.draw_frame(event_loop);
        } else {
            self.renderer.as_mut().unwrap().resumed();
        }
    }
}

pub fn run<A: Element>(app: impl Into<ElementPointer<A>>) {
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut application_handler = WinitApplicationHandler::new(app.into());

    event_loop.run_app(&mut application_handler).ok();
}
