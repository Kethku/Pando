use std::{ops::Deref, sync::Arc};

use glamour::prelude::*;
use vide::{
    winit::{event_loop::ActiveEventLoop, window::Window},
    Layer, Scene,
};

use crate::framework::mouse_region::{MouseRegion, MouseRegionManager};

pub struct EventState {
    pub mouse_position: Point2,
    pub previous_mouse_position: Point2,
    pub window_size: Size2,
    pub mouse_down: bool,
    pub was_mouse_down: bool,
}

impl EventState {
    pub fn new() -> Self {
        Self {
            mouse_position: point2!(0., 0.),
            previous_mouse_position: point2!(0., 0.),
            window_size: size2!(0., 0.),
            mouse_down: false,
            was_mouse_down: false,
        }
    }

    pub fn next_frame(&mut self) {
        self.was_mouse_down = self.mouse_down;
        self.previous_mouse_position = self.mouse_position;
    }

    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }

    pub fn was_mouse_down(&self) -> bool {
        self.was_mouse_down
    }

    pub fn mouse_released(&self) -> bool {
        !self.mouse_down && self.was_mouse_down
    }

    pub fn mouse_just_down(&self) -> bool {
        self.mouse_down && !self.was_mouse_down
    }

    pub fn mouse_held(&self) -> bool {
        self.mouse_down && self.was_mouse_down
    }

    pub fn mouse_position(&self) -> Point2 {
        self.mouse_position
    }

    pub fn previous_mouse_position(&self) -> Point2 {
        self.previous_mouse_position
    }

    pub fn mouse_delta(&self) -> Vector2 {
        self.mouse_position - self.previous_mouse_position
    }

    pub fn window_size(&self) -> Size2 {
        self.window_size
    }
}

pub struct Context<'a> {
    event_state: &'a EventState,
    event_loop: &'a ActiveEventLoop,
    window: Arc<Window>,
}

impl<'a> Deref for Context<'a> {
    type Target = EventState;

    fn deref(&self) -> &Self::Target {
        self.event_state
    }
}

impl<'a> Context<'a> {
    pub fn new(
        event_state: &'a EventState,
        event_loop: &'a ActiveEventLoop,
        window: Arc<Window>,
    ) -> Context<'a> {
        Context {
            event_state,
            event_loop,
            window,
        }
    }

    pub fn close(&self) {
        self.event_loop.exit();
    }

    pub fn toggle_maximized(&self) {
        self.window.set_maximized(!self.window.is_maximized());
    }

    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    pub fn minimize(&self) {
        self.window.set_minimized(true);
    }

    pub fn is_minimized(&self) -> bool {
        self.window.is_minimized().unwrap_or_default()
    }

    pub fn drag_window(&self) {
        self.window.drag_window().expect("Could not drag window");
    }
}

pub struct UpdateContext<'a> {
    context: &'a Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
}

impl<'a> Deref for UpdateContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> UpdateContext<'a> {
    pub fn new(
        context: &'a Context<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
    ) -> UpdateContext<'a> {
        UpdateContext {
            context,
            mouse_region_manager,
        }
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }
}

pub struct DrawContext<'a> {
    context: &'a Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    scene: Scene,
}

impl<'a> Deref for DrawContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DrawContext<'a> {
    pub fn new(
        context: &'a Context<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
    ) -> DrawContext<'a> {
        DrawContext {
            context,
            mouse_region_manager,
            scene: Scene::new(),
        }
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.scene.add_layer(layer);
    }

    pub fn to_scene(self) -> Scene {
        self.scene
    }
}
