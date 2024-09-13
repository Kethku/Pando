use futures::executor::block_on;
use std::{cell::RefCell, sync::Arc};

use glamour::prelude::*;
use rust_embed::RustEmbed;
use vide::{
    winit::{
        application::ApplicationHandler,
        event::{ElementState, MouseButton, WindowEvent},
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        platform::windows::WindowAttributesExtWindows,
        window::{Cursor, CursorIcon, ResizeDirection, Window, WindowAttributes, WindowId},
    },
    WinitRenderer,
};

use crate::framework::{
    context::{Context, DrawContext, EventState, UpdateContext},
    mouse_region::MouseRegionManager,
};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

struct WinitApplicationHandler<A: FrameworkApplication> {
    mouse_region_manager: RefCell<MouseRegionManager>,
    app: RefCell<A>,
    event_state: EventState,
    renderer: Option<WinitRenderer>,

    force_redraw: bool,
}

impl<A: FrameworkApplication> WinitApplicationHandler<A> {
    fn new(app: A) -> Self {
        WinitApplicationHandler {
            mouse_region_manager: RefCell::new(MouseRegionManager::new()),
            app: RefCell::new(app),
            event_state: EventState::new(),
            renderer: None,

            force_redraw: false,
        }
    }

    async fn create_renderer(window: Arc<Window>) -> WinitRenderer {
        WinitRenderer::new(window)
            .await
            .with_default_drawables()
            .await
    }

    fn context<'a>(&'a self, event_loop: &'a ActiveEventLoop) -> Context<'a> {
        Context::new(
            &self.event_state,
            event_loop,
            self.renderer.as_ref().unwrap().window.clone(),
        )
    }

    fn handle_resize_regions(&mut self) {
        let window = self.renderer.as_ref().unwrap().window.clone();

        let mut result = None;
        if self.event_state.mouse_position.x < 12. && self.event_state.mouse_position.y < 12. {
            result = Some((ResizeDirection::NorthWest, CursorIcon::NwResize));
        } else if self.event_state.mouse_position.x < 12.
            && self.event_state.mouse_position.y > self.event_state.window_size.height - 12.
        {
            result = Some((ResizeDirection::SouthWest, CursorIcon::SwResize));
        } else if self.event_state.mouse_position.x > self.event_state.window_size.width - 12.
            && self.event_state.mouse_position.y < 12.
        {
            result = Some((ResizeDirection::NorthEast, CursorIcon::NeResize));
        } else if self.event_state.mouse_position.x > self.event_state.window_size.width - 12.
            && self.event_state.mouse_position.y > self.event_state.window_size.height - 12.
        {
            result = Some((ResizeDirection::SouthEast, CursorIcon::SeResize));
        } else if self.event_state.mouse_position.x < 8. {
            result = Some((ResizeDirection::West, CursorIcon::WResize));
        } else if self.event_state.mouse_position.x > self.event_state.window_size.width - 8. {
            result = Some((ResizeDirection::East, CursorIcon::EResize));
        } else if self.event_state.mouse_position.y < 8. {
            result = Some((ResizeDirection::North, CursorIcon::NResize));
        } else if self.event_state.mouse_position.y > self.event_state.window_size.height - 8. {
            result = Some((ResizeDirection::South, CursorIcon::SResize));
        }

        if let Some((direction, icon)) = result {
            window.set_cursor(Cursor::Icon(icon));
            if self.event_state.mouse_down {
                window.drag_resize_window(direction).ok();
            }
        } else {
            window.set_cursor(CursorIcon::Default);
        }
    }

    fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
        let mut mouse_region_manager = self.mouse_region_manager.borrow_mut();
        let mut app = self.app.borrow_mut();
        let context = self.context(event_loop);
        mouse_region_manager.process_regions(&context);

        let needs_redraw = {
            let mut update_context = UpdateContext::new(&context, &mut mouse_region_manager);
            app.update(&update_context)
        };
        if needs_redraw || self.force_redraw {
            mouse_region_manager.clear_regions();
            let mut draw_context = DrawContext::new(&context, &mut mouse_region_manager);
            app.draw(&mut draw_context);
            let scene = draw_context.to_scene();

            self.renderer.as_mut().unwrap().draw(&scene);
            self.force_redraw = false;
            self.renderer.as_ref().unwrap().window.request_redraw();
        }
        self.event_state.next_frame();
    }
}

impl<A: FrameworkApplication> ApplicationHandler for WinitApplicationHandler<A> {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.event_state.mouse_position = point2!(position.x as f32, position.y as f32);
                self.handle_resize_regions();
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.event_state.mouse_down = state == ElementState::Pressed;
                self.handle_resize_regions();
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
                    size2!(new_size.width as f32, new_size.height as f32);
                self.force_redraw = true;
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_none() {
            let attributes = WindowAttributes::default()
                .with_resizable(true)
                .with_decorations(false)
                .with_undecorated_shadow(true);
            let window = Arc::new(
                event_loop
                    .create_window(attributes)
                    .expect("Failed to create window"),
            );
            self.renderer = Some(block_on(Self::create_renderer(window)));
        } else {
            self.renderer.as_mut().unwrap().resumed();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer.as_mut().unwrap().suspended();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: ()) {
        self.renderer.as_ref().unwrap().window.request_redraw();
    }
}

pub trait FrameworkApplication {
    fn update(&mut self, cx: &Context) -> bool;
    fn draw(&self, cx: &mut DrawContext);
}

pub fn run<A: FrameworkApplication>(app: A) {
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut application_handler = WinitApplicationHandler::new(app);

    event_loop.run_app(&mut application_handler).ok();
}
