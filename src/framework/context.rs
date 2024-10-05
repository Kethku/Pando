use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

use glamour::prelude::*;
use mockall::*;
use vide::{
    prelude::*,
    winit::{
        event_loop::ActiveEventLoop,
        window::{Cursor, CursorIcon, ResizeDirection, Window},
    },
};

use crate::framework::{
    mouse_region::{MouseRegion, MouseRegionManager},
    token::Token,
};

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

    pub fn window_rect(&self) -> Rect {
        Rect::new(point2!(0., 0.), self.window_size)
    }
}

pub struct Context<'a> {
    event_state: &'a EventState,
    event_loop: &'a dyn ContextEventLoop,
    window: Arc<dyn ContextWindow>,
    element_token: Token,
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
        event_loop: &'a dyn ContextEventLoop,
        window: Arc<dyn ContextWindow>,
        element_token: Token,
    ) -> Context<'a> {
        Context {
            event_state,
            event_loop,
            window,
            element_token,
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

    pub fn drag_resize_window(&self, direction: ResizeDirection) {
        self.window
            .drag_resize_window(direction)
            .expect("Could not drag resize window");
    }

    pub fn set_cursor(&self, icon: CursorIcon) {
        self.window.set_cursor(Cursor::Icon(icon));
    }

    pub fn token(&self) -> Token {
        self.element_token
    }

    pub fn child<'b>(&self, element_token: Token) -> Context<'b>
    where
        'a: 'b,
    {
        Context {
            event_state: self.event_state,
            event_loop: self.event_loop,
            window: self.window.clone(),
            element_token,
        }
    }
}

#[automock]
pub trait ContextEventLoop {
    fn exit(&self);
}

impl ContextEventLoop for ActiveEventLoop {
    fn exit(&self) {
        self.exit();
    }
}

#[automock]
pub trait ContextWindow {
    fn set_maximized(&self, maximized: bool);
    fn is_maximized(&self) -> bool;
    fn set_minimized(&self, minimized: bool);
    fn is_minimized(&self) -> Option<bool>;
    fn drag_window(&self) -> Result<(), String>;
    fn drag_resize_window(&self, direction: ResizeDirection) -> Result<(), String>;
    fn set_cursor(&self, cursor: Cursor);
    fn request_redraw(&self);
}

impl ContextWindow for Window {
    fn set_maximized(&self, maximized: bool) {
        self.set_maximized(maximized);
    }

    fn is_maximized(&self) -> bool {
        self.is_maximized()
    }

    fn set_minimized(&self, minimized: bool) {
        self.set_minimized(minimized);
    }

    fn is_minimized(&self) -> Option<bool> {
        self.is_minimized()
    }

    fn drag_window(&self) -> Result<(), String> {
        self.drag_window().map_err(|e| e.to_string())
    }

    fn drag_resize_window(&self, direction: ResizeDirection) -> Result<(), String> {
        self.drag_resize_window(direction)
            .map_err(|e| e.to_string())
    }

    fn set_cursor(&self, cursor: Cursor) {
        self.set_cursor(cursor);
    }

    fn request_redraw(&self) {
        self.request_redraw();
    }
}

pub struct EventContext<'a> {
    context: &'a Context<'a>,
    redraw_requested: &'a mut bool,
}

impl<'a> Deref for EventContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> EventContext<'a> {
    pub fn new(context: &'a Context<'a>, redraw_requested: &'a mut bool) -> EventContext<'a> {
        EventContext {
            context,
            redraw_requested,
        }
    }

    pub fn request_redraw(&mut self) {
        *self.redraw_requested = true;
    }
}

pub struct UpdateContext<'a> {
    context: Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    redraw_requested: &'a mut bool,
}

impl<'a> Deref for UpdateContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> UpdateContext<'a> {
    pub fn new(
        context: Context<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
        redraw_requested: &'a mut bool,
    ) -> UpdateContext<'a> {
        UpdateContext {
            context,
            mouse_region_manager,
            redraw_requested,
        }
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }

    pub fn request_redraw(&mut self) {
        *self.redraw_requested = true;
    }

    pub fn child<'b>(&'b mut self, element_token: Token) -> UpdateContext<'b>
    where
        'a: 'b,
    {
        let child_cx: Context<'b> = self.context.child(element_token);
        UpdateContext {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            redraw_requested: self.redraw_requested,
        }
    }
}

pub struct LayoutContext<'a> {
    context: Context<'a>,
    regions: &'a mut HashMap<Token, Rect>,
    children: &'a mut HashMap<Token, HashSet<Token>>,
}

impl<'a> Deref for LayoutContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        context: Context<'a>,
        regions: &'a mut HashMap<Token, Rect>,
        children: &'a mut HashMap<Token, HashSet<Token>>,
    ) -> LayoutContext<'a> {
        LayoutContext {
            context,
            regions,
            children,
        }
    }

    pub fn add_region(&mut self, token: Token, rect: Rect) {
        self.regions.insert(token, rect);
    }

    pub fn child<'b>(&'b mut self, token: Token) -> LayoutContext<'b>
    where
        'a: 'b,
    {
        self.children.entry(self.token()).or_default().insert(token);
        let child_cx: Context<'b> = self.context.child(token);
        LayoutContext::<'b> {
            context: child_cx,
            regions: self.regions,
            children: self.children,
        }
    }

    pub fn translate_descendants(&mut self, token: Token, offset: Vector2) {
        if let Some(children) = self.children.get(&token).cloned() {
            for child in children.iter() {
                if let Some(region) = self.regions.get_mut(child) {
                    *region = region.translate(offset);
                    self.translate_descendants(*child, offset);
                }
            }
        }
    }
}

pub struct DrawContext<'a> {
    context: Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    regions: &'a HashMap<Token, Rect>,
    scene: &'a mut Scene,
}

impl<'a> Deref for DrawContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DrawContext<'a> {
    pub fn new(
        context: Context<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
        regions: &'a HashMap<Token, Rect>,
        scene: &'a mut Scene,
    ) -> DrawContext<'a> {
        DrawContext {
            context,
            mouse_region_manager,
            regions,
            scene,
        }
    }

    pub fn mouse_region(&mut self, region: Rect) -> &mut MouseRegion {
        self.mouse_region_manager
            .add_region(MouseRegion::new(self.context.token(), region))
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.scene.add_layer(layer);
    }

    pub fn update_layer(&mut self, update: impl FnOnce(&mut Resources, &mut Layer)) {
        self.scene.update_layer(update);
    }

    pub fn request_redraw(&self) {
        self.context.window.request_redraw();
    }

    pub fn region(&self) -> Rect {
        self.regions
            .get(&self.context.token())
            .copied()
            .expect("Layout must not have been completed before drawing")
    }

    pub fn child<'b>(&'b mut self, token: Token) -> DrawContext<'b>
    where
        'a: 'b,
    {
        let child_cx: Context<'b> = self.context.child(token);
        DrawContext::<'b> {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            regions: self.regions,
            scene: self.scene,
        }
    }
}
