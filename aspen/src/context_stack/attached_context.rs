use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use mockall::*;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Cursor, CursorIcon, ResizeDirection, Window},
};

use super::Context;

use crate::token::Token;

pub struct AttachedContext<'a> {
    context: Context<'a>,
    window: Arc<dyn ContextWindow>,
    event_loop: &'a dyn ContextEventLoop,
}

impl<'a> Deref for AttachedContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DerefMut for AttachedContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl<'a> AttachedContext<'a> {
    pub fn new(
        context: Context<'a>,
        window: Arc<dyn ContextWindow>,
        event_loop: &'a dyn ContextEventLoop,
    ) -> AttachedContext<'a> {
        AttachedContext {
            context,
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

    pub fn drag_resize_window(&self, direction: ResizeDirection) {
        self.window
            .drag_resize_window(direction)
            .expect("Could not drag resize window");
    }

    pub fn set_cursor(&self, icon: CursorIcon) {
        self.window.set_cursor(Cursor::Icon(icon));
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn child<'b>(
        &'b mut self,
        element_token: Token,
        element_children: &'b Vec<Token>,
    ) -> AttachedContext<'b> {
        let child_cx: Context<'b> = self.context.child(element_token, element_children);
        AttachedContext {
            context: child_cx,
            window: self.window.clone(),
            event_loop: self.event_loop,
        }
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

#[automock]
pub trait ContextEventLoop {
    fn exit(&self);
}

impl ContextEventLoop for ActiveEventLoop {
    fn exit(&self) {
        self.exit();
    }
}
